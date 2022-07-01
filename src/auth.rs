// Copyright 2022 The Casdoor Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    vec,
};

use jsonwebtoken;
use reqwest::Response;
use serde_json;
use x509_parser::prelude::*;

use super::{networkconfig::*, user::*};
/// CasdoorSDK struct, the core struct of our SDK
///
/// We provided two ways of initialize
///
/// 1. construct a CasdoorSDK struct using new() method
///
/// ``` no run
/// use casdoorsdk::CasdoorSDK;
/// let app = CasdoorSDK::new("http://localhost:8080", "client_id", "client_secret", certificate, org_name, front_endpoint, None, None);
/// ```
/// 2. construct a CasdoorSDK struct using newsdk! macro
///
/// endpoint, client_id, client_secret, certificate, org_name is a must, the left is optional
///
/// ``` no run
/// let app = newsdk!(endpoint, client_id, client_secret, certificate, org_name);
/// ```
///
#[derive(Debug, Clone)]
pub struct CasdoorSDK {
    endpoint: String,
    client_id: String,
    client_secret: String,
    certificate: Box<[u8]>,
    org_name: String,
    front_endpoint: String,
    grant_type: String,
    algorithms: Vec<String>,
}

#[macro_export]
macro_rules! newsdk {
    ($endpoint:expr, $client_id:expr, $client_secret:expr, $certificate:expr, $org_name:expr) => {
        CasdoorSDK::new(
            $endpoint,
            $client_id,
            $client_secret,
            $certificate,
            $org_name,
            None,
            None,
            None,
        )
    };
    ($endpoint:expr, $client_id:expr, $client_secret:expr, $certificate:expr, $org_name:expr, $front_endpoint:expr) => {
        CasdoorSDK::new(
            $endpoint,
            $client_id,
            $client_secret,
            $certificate,
            $org_name,
            $front_endpoint,
            None,
            None,
        )
    };
    ($endpoint:expr, $client_id:expr, $client_secret:expr, $certificate:expr, $org_name:expr, $front_endpoint:expr, $grant_type:expr, $algorithms:expr) => {
        CasdoorSDK::new(
            $endpoint,
            $client_id,
            $client_secret,
            $certificate,
            $org_name,
            $front_endpoint,
            $grant_type,
            $algorithms,
        )
    };
}

impl Default for CasdoorSDK {
    fn default() -> Self {
        CasdoorSDK::new("", "", "", Box::new([0]), "", None, None, None)
    }
}

impl CasdoorSDK {
    /// Construct a new CasdoorSDK struct
    /// the parameters are:
    ///
    /// 1. endpoint: the endpoint of casdoor server, e.g. http://localhost:8080
    ///
    /// 2. client_id: the client id of your application, you can find it in your App console
    ///
    /// 3. client_secret: the client secret of your application, you can find it in your App console
    ///
    /// 4. certificate: the your certificate, same as Casdoor certificate
    ///
    /// 5. org_name: the organization name of your application
    ///
    /// 6. front_endpoint(Optional, Some(&str) or None): the frontend endpoint of your application, e.g. http://localhost:7001
    ///
    /// if you don't set it, the SDK will use the endpoint(replace port 8000 with 7001) as the frontend endpoint
    ///
    /// 7. grant_type(Optional, Some(&str) or None): the grant type of your application, you can find it in your App console.
    ///	if you don't set it, the SDK will use "authorization_code" as the grant type. *We now only support "authorization_code"*
    ///
    /// 8. algorithms(Optional, Some(Vec<String>) or None): the algorithms of your application, you can find it in your App console.
    /// if you don't set it, the SDK will use "RS256" as the algorithms. *We now only support "RS256"*
    ///
    pub fn new(
        endpoint: &str,
        client_id: &str,
        client_secret: &str,
        certificate: Box<[u8]>,
        org_name: &str,
        front_endpoint: Option<&str>,
        grant_type: Option<&str>,
        algorithms: Option<Vec<String>>,
    ) -> Self {
        let front_endpoint = front_endpoint
            .unwrap_or(&endpoint.replace(":8000", ":7001"))
            .to_owned();
        CasdoorSDK {
            endpoint: endpoint.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            certificate,
            org_name: org_name.to_string(),
            front_endpoint,
            // default value, no need to specify in most cases
            grant_type: grant_type.unwrap_or("authorization_code").to_string(),
            algorithms: algorithms.unwrap_or(vec!["RS256".to_owned()]).to_vec(),
        }
    }
    ///get the auth link of casdoor, you can send it to user, when they finished signin,
    /// your call back address will receive
    #[tokio::main]
    pub async fn get_auth_link(
        &self,
        config: &NetWorkConfig,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut url = self.front_endpoint.clone() + "/login/oauth/authorize";
        let mut params = config.into_map();
        params.insert("client_id".to_owned(), self.client_id.clone());
        get_formed_url(&params, &mut url);
        let client = reqwest::Client::new();
        let res = client.get(&url).send().await?.error_for_status()?;
        Ok(res.url().to_string())
    }

    pub async fn get_oauth_token(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut url = self.endpoint.clone() + "/api/login/oauth/access_token";
        let mut params = BTreeMap::new();
        params.insert("grant_type".to_owned(), self.grant_type.clone());
        params.insert("code".to_owned(), code.to_string());
        params.insert("client_id".to_owned(), self.client_id.clone());
        params.insert("client_secret".to_owned(), self.client_secret.clone());
        get_formed_url(&params, &mut url);
        let client = reqwest::Client::new();
        let res = client.post(&url).send().await?;
        let json: serde_json::Value = res.json().await?;
        Ok(json["access_token"].as_str().unwrap().to_string())
    }

    //remain unsolved
    pub fn parse_jwt_token(&self, token: &str) -> Result<User, Box<dyn std::error::Error>> {
        for pem in Pem::iter_from_buffer(&self.certificate) {
            let pem = pem.expect("Reading next PEM block failed");
            let x509 = pem.parse_x509().expect("X.509: decoding DER failed");
            let key = x509.public_key().subject_public_key.data;
            let key = jsonwebtoken::DecodingKey::from_rsa_der(key);
            let validator = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
            let json_info = jsonwebtoken::decode::<User>(&token, &key, &validator)?;
            return Ok(json_info.claims);
        }
        Err(Box::new(X509Error::InvalidCertificate))
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let mut url = self.endpoint.clone() + "/api/get-users";
        let mut params = BTreeMap::new();
        params.insert("owner".to_owned(), &self.org_name);
        params.insert("clientId".to_owned(), &self.client_id);
        params.insert("clientSecret".to_owned(), &self.client_secret);
        get_formed_url(&params, &mut url);
        let client = reqwest::Client::new();
        let res = client.get(&url).send().await?;
        let json: serde_json::Value = res.json().await?;
        let users: Vec<User> = serde_json::from_value(json)?;
        Ok(users)
    }

    /// get user by name
    /// warning: this is unstable,
    // TODO: implement Deserialize manually for User, making it safer
    pub async fn get_user_by_name(
        &self,
        name: &str,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let mut url = self.endpoint.clone() + "/api/get-user";
        let user_info = format!("{}/{}", &self.org_name, name);
        let mut params: BTreeMap<String, &str> = BTreeMap::new();
        params.insert("clientId".to_owned(), &self.client_id);
        params.insert("clientSecret".to_owned(), &self.client_secret);
        params.insert("id".to_owned(), &user_info);
        get_formed_url(&params, &mut url);
        let client = reqwest::Client::new();
        let res = client.get(&url).send().await?;
        let json: serde_json::Value = res.json().await?;
        let user: User = serde_json::from_value(json)?;
        Ok(Some(user))
    }

    //general function used by add user, update user and delete user
    async fn modify_user(
        &self,
        action: &str,
        mut user: User,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = self.endpoint.clone() + format!("/api/{}", action).as_str();
        user.owner = self.org_name.clone();
        let mut params = BTreeMap::new();
        let id = format!("{}/{}", user.owner, user.name);
        params.insert("id".to_owned(), &id);
        params.insert("clientId".to_owned(), &self.client_id);
        params.insert("clientSecret".to_owned(), &self.client_secret);
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .form(&params)
            .body(serde_json::to_string(&user).unwrap())
            .send()
            .await?;
        Ok(res)
    }

    /// add user function, will get user and add it to casdoor
    /// # Arguments
    /// * `user` - user to add
    /// # Returns
    /// * `Response` - response from casdoor
    /// # Example
    ///
    /// ```no run
    /// use casdoor_sdk::*;
    /// let sdk = newsdk!("http://localhost:8080", "client_id", "client_secret", "certificate", "org_name");
    /// let user = user!(name = "Bob")
    /// let res = sdk.add_user(user).await?;
    /// ```
    pub async fn add_user(&self, user: User) -> Result<Response, Box<dyn std::error::Error>> {
        self.modify_user("add-user", user).await
    }

    pub async fn update_user(&self, user: User) -> Result<Response, Box<dyn std::error::Error>> {
        self.modify_user("update-user", user).await
    }

    pub async fn delete_user(&self, user: User) -> Result<Response, Box<dyn std::error::Error>> {
        self.modify_user("delete-user", user).await
    }
}

fn get_formed_url<T, P>(params: &BTreeMap<T, P>, url: &mut String)
where
    T: Display,
    P: Display,
{
    url.push('?');
    for (key, value) in params {
        url.push_str(&format!("{}={}&", key, value));
    }
    match url.pop() {
        Some(ch) if ch != '&' => url.push(ch),
        _ => {}
    }
}
