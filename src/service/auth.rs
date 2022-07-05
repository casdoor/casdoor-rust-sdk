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

use crate::entity::{CasdoorConfig, CasdoorUser};

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, TokenResponse, TokenUrl};

pub struct AuthService<'a> {
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> AuthService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub fn get_auth_token(&self, code: String) -> Result<String, Box<dyn std::error::Error>> {
        let client_id = ClientId::new(self.config.client_id.clone());
        let client_secret = ClientSecret::new(self.config.client_secret.clone());
        let auth_url = AuthUrl::new(format!(
            "{}/api/login/oauth/authorize",
            self.config.endpoint
        ))?;
        let token_url = TokenUrl::new(format!(
            "{}/api/login/oauth/access_token",
            self.config.endpoint
        ))?;
        let code = AuthorizationCode::new(code);

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url));
        let token_res = client.exchange_code(code).request(http_client)?;

        Ok(token_res.access_token().secret().to_string())
    }

    pub fn parse_jwt_token(
        &self,
        token: String,
    ) -> Result<CasdoorUser, Box<dyn std::error::Error>> {
        let res = jsonwebtoken::decode::<CasdoorUser>(
            &token,
            &DecodingKey::from_rsa_pem(self.config.jwt_pub_key.as_bytes())?,
            &Validation::new(Algorithm::RS256),
        )?;

        Ok(res.claims)
    }

    pub fn get_signin_url(&self, redirect_url: String) -> String {
        let scope = "read";
        let state = self.config.app_name.clone().unwrap_or_default();
        format!("{}/login/oauth/authorize?client_id={}&response_type=code&redirect_uri={}&scope={}&state={}", 
            self.config.endpoint,
            self.config.client_id,
            urlencoding::encode(&redirect_url).into_owned(),
            scope, state)
    }

    pub fn get_signup_url(&self, redirect_url: String) -> String {
        redirect_url.replace("/login/oauth/authorize", "/signup/oauth/authorize")
    }

    pub fn get_signup_url_enable_password(&self) -> String {
        format!(
            "{}/signup/{}",
            self.config.endpoint,
            self.config.app_name.clone().unwrap_or_default()
        )
    }

    pub fn get_user_profile_url(&self, uname: String, token: Option<String>) -> String {
        let param = match token {
            Some(token) if !token.is_empty() => format!("?access_token={}", token),
            _ => "".to_string(),
        };
        format!(
            "{}/users/{}/{}{}",
            self.config.endpoint, self.config.org_name, uname, param
        )
    }

    pub fn get_my_profile_url(&self, token: Option<String>) -> String {
        let param = match token {
            Some(token) if !token.is_empty() => format!("?access_token={}", token),
            _ => "".to_string(),
        };
        format!("{}/account{}", self.config.endpoint, param)
    }
}
