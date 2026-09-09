// Copyright 2026 The Casdoor Authors. All Rights Reserved.
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

use std::collections::HashMap;

use reqwest::{multipart, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::AuthConfig;
use crate::error::{CasdoorError, Result};

/// Response is the general response returned by the Casdoor server APIs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub msg: String,
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub data2: Value,
    #[serde(default)]
    pub data3: Value,
}

/// The body of a POST request sent to the Casdoor server.
pub enum PostBody<'a> {
    /// A raw body posted as `text/plain;charset=UTF-8`, it's what most of the Casdoor
    /// APIs expect.
    Raw(Vec<u8>),
    /// A `multipart/form-data` body made of text fields.
    Form(HashMap<String, String>),
    /// A `multipart/form-data` body made of a single file field named `file`.
    File(&'a [u8]),
}

/// Client is the Casdoor client, it holds the config of the Casdoor application and sends
/// all the API requests to the Casdoor server.
///
/// ```no_run
/// use casdoor_rust_sdk::Client;
///
/// # async fn f() -> casdoor_rust_sdk::Result<()> {
/// let client = Client::new(
///     "http://localhost:8000",
///     "<client-id>",
///     "<client-secret>",
///     "<certificate>",
///     "built-in",
///     "app-built-in",
/// );
///
/// let users = client.get_users().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    /// The config of the Casdoor application.
    pub config: AuthConfig,
    /// The extra HTTP headers added to every request sent to the Casdoor server.
    pub custom_headers: HashMap<String, String>,
    /// The user's access token. If it's not empty, all the API requests sent by this
    /// client are authenticated as the user who owns the access token (via the
    /// `Authorization: Bearer` header), instead of as the application itself (via the
    /// client ID and client secret's Basic Auth).
    /// Use [`Client::with_access_token()`] to get such a client.
    pub access_token: Option<String>,
    http: reqwest::Client,
}

impl Client {
    /// Create a new client from the 6 config values.
    pub fn new(
        endpoint: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        certificate: impl Into<String>,
        organization_name: impl Into<String>,
        application_name: impl Into<String>,
    ) -> Self {
        Self::from_config(AuthConfig::new(
            endpoint,
            client_id,
            client_secret,
            certificate,
            organization_name,
            application_name,
        ))
    }

    /// Create a new client from an [`AuthConfig`].
    pub fn from_config(config: AuthConfig) -> Self {
        Self {
            config,
            custom_headers: HashMap::new(),
            access_token: None,
            http: reqwest::Client::new(),
        }
    }

    /// Create a new client from a TOML config file, see [`AuthConfig::from_toml()`].
    pub fn from_toml(path: &str) -> Result<Self> {
        Ok(Self::from_config(AuthConfig::from_toml(path)?))
    }

    /// Return a copy of the client that calls all the Casdoor APIs as the user who owns
    /// the given access token, rather than as the application. The access token is the
    /// user's OAuth access token returned by [`Client::get_oauth_token()`],
    /// [`Client::refresh_oauth_token()`], [`Client::get_oauth_token_by_password()`] or
    /// [`Client::impersonate_user()`].
    ///
    /// The returned client is independent from the original one, so the original client
    /// keeps using the application's client ID and client secret, and it's safe to create
    /// one client per user request:
    ///
    /// ```no_run
    /// # async fn f(client: &casdoor_rust_sdk::Client, code: &str) -> casdoor_rust_sdk::Result<()> {
    /// let token = client.get_oauth_token(code).await?;
    /// let user = client.with_access_token(token.access_token).get_account().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Note that the APIs are still subject to Casdoor's permission check, so a non-admin
    /// user can only access their own data.
    pub fn with_access_token(&self, access_token: impl Into<String>) -> Self {
        Self {
            access_token: Some(access_token.into()),
            ..self.clone()
        }
    }

    /// Return a copy of the client that adds the given HTTP header to every request.
    pub fn with_custom_header(&self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut client = self.clone();
        client.custom_headers.insert(key.into(), value.into());
        client
    }

    /// Return a copy of the client that uses the given [`reqwest::Client`], so that the
    /// timeout, the proxy, the TLS options, etc. can be customized.
    pub fn with_http_client(&self, http: reqwest::Client) -> Self {
        Self {
            http,
            ..self.clone()
        }
    }

    /// The underlying HTTP client, shared by all the requests sent by this client.
    pub(crate) fn http(&self) -> &reqwest::Client {
        &self.http
    }

    /// Return the URL of an API action, such as
    /// `http://localhost:8000/api/get-user?id=built-in%2Fadmin`.
    pub fn get_url(&self, action: &str, query: &[(&str, &str)]) -> String {
        let query = query
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        format!("{}/api/{}?{}", self.config.endpoint, action, query)
    }

    /// Return the Casdoor object ID (`"owner/name"`) of an object owned by an
    /// organization. If `name` is already a qualified `"owner/name"` ID, it is returned
    /// as-is, so that a client can also address objects that live outside of its own
    /// organization. Object names never contain a slash in Casdoor.
    pub fn get_id(&self, name: &str) -> String {
        get_id(name, &self.config.organization_name)
    }

    /// The general function to send a GET request and get the whole [`Response`].
    pub async fn do_get_response(&self, url: &str) -> Result<Response> {
        let bytes = self.do_get_bytes_raw_without_check(url).await?;
        let response: Response = serde_json::from_slice(&bytes)?;

        if response.status != "ok" {
            return Err(CasdoorError::Casdoor(response.msg));
        }

        Ok(response)
    }

    /// The general function to send a GET request and get the raw response body.
    pub async fn do_get_bytes_raw(&self, url: &str) -> Result<Vec<u8>> {
        let bytes = self.do_get_bytes_raw_without_check(url).await?;

        if let Ok(response) = serde_json::from_slice::<Response>(&bytes) {
            if response.status == "error" {
                return Err(CasdoorError::Casdoor(response.msg));
            }
        }

        Ok(bytes)
    }

    /// The general function to send a POST request and get the whole [`Response`].
    pub async fn do_post(
        &self,
        action: &str,
        query: &[(&str, &str)],
        body: PostBody<'_>,
    ) -> Result<Response> {
        let url = self.get_url(action, query);
        let bytes = self.do_post_bytes_raw(&url, body).await?;

        let response: Response = serde_json::from_slice(&bytes)?;
        if response.status != "ok" {
            return Err(CasdoorError::Casdoor(response.msg));
        }

        Ok(response)
    }

    /// The general function to send a POST request and get the raw response body.
    pub async fn do_post_bytes_raw(&self, url: &str, body: PostBody<'_>) -> Result<Vec<u8>> {
        let mut req = self.http.post(url);

        req = match body {
            PostBody::Raw(bytes) => req
                .header(reqwest::header::CONTENT_TYPE, "text/plain;charset=UTF-8")
                .body(bytes),
            PostBody::Form(fields) => {
                let form = fields
                    .into_iter()
                    .fold(multipart::Form::new(), |form, (k, v)| form.text(k, v));
                req.multipart(form)
            }
            PostBody::File(bytes) => {
                let part = multipart::Part::bytes(bytes.to_vec()).file_name("file");
                req.multipart(multipart::Form::new().part("file", part))
            }
        };

        read_body(self.set_headers(req).send().await?).await
    }

    async fn do_get_bytes_raw_without_check(&self, url: &str) -> Result<Vec<u8>> {
        let req = self.set_headers(self.http.get(url));
        read_body(req.send().await?).await
    }

    /// Set the `Authorization` header and the custom headers of a request. The user's
    /// access token is used when the client has one, so that the API is called as the
    /// user instead of as the application. Otherwise the application's client ID and
    /// client secret are used.
    fn set_headers(&self, req: RequestBuilder) -> RequestBuilder {
        let mut req = match &self.access_token {
            Some(token) if !token.is_empty() => req.bearer_auth(token),
            _ => req.basic_auth(&self.config.client_id, Some(&self.config.client_secret)),
        };

        for (key, value) in &self.custom_headers {
            req = req.header(key, value);
        }

        req
    }

    /// Get one object from an API action, `None` is returned when the Casdoor server
    /// answers with a `null` data, which means the object doesn't exist.
    pub(crate) async fn get_object<T: DeserializeOwned>(
        &self,
        action: &str,
        query: &[(&str, &str)],
    ) -> Result<Option<T>> {
        let url = self.get_url(action, query);
        let response = self.do_get_response(&url).await?;

        if response.data.is_null() {
            return Ok(None);
        }

        Ok(Some(serde_json::from_value(response.data)?))
    }

    /// Get a list of objects from an API action.
    pub(crate) async fn get_list<T: DeserializeOwned>(
        &self,
        action: &str,
        query: &[(&str, &str)],
    ) -> Result<Vec<T>> {
        let url = self.get_url(action, query);
        let response = self.do_get_response(&url).await?;

        if response.data.is_null() {
            return Ok(Vec::new());
        }

        Ok(serde_json::from_value(response.data)?)
    }

    /// Get one page of objects from an API action, the returned count is the total number
    /// of the objects.
    pub(crate) async fn get_pagination<T: DeserializeOwned>(
        &self,
        action: &str,
        owner: &str,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<T>, i32)> {
        let (p, page_size) = (p.to_string(), page_size.to_string());

        let mut full_query = vec![
            ("owner", owner),
            ("p", p.as_str()),
            ("pageSize", page_size.as_str()),
        ];
        full_query.extend_from_slice(query);

        let url = self.get_url(action, &full_query);
        let response = self.do_get_response(&url).await?;

        let objects = if response.data.is_null() {
            Vec::new()
        } else {
            serde_json::from_value(response.data).map_err(|_| CasdoorError::InvalidData)?
        };

        let count = response.data2.as_f64().ok_or(CasdoorError::InvalidData)? as i32;

        Ok((objects, count))
    }

    /// The encapsulation of the CUD (Create, Update, Delete) operations, the returned bool
    /// tells whether the object was really affected.
    pub(crate) async fn modify_object<T: Serialize>(
        &self,
        action: &str,
        id: &str,
        object: &T,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let columns = columns.join(",");

        let mut query = vec![("id", id)];
        if !columns.is_empty() {
            query.push(("columns", columns.as_str()));
        }

        let post_bytes = serde_json::to_vec(object)?;
        let response = self
            .do_post(action, &query, PostBody::Raw(post_bytes))
            .await?;

        let affected = response.data.as_str() == Some("Affected");
        Ok((response, affected))
    }
}

/// `get_admin_id` is [`Client::get_id()`] for the object types that are owned by `admin`
/// instead of by an organization: organization, application, token and LDAP server.
pub(crate) fn get_admin_id(name: &str) -> String {
    get_id(name, "admin")
}

fn get_id(name: &str, default_owner: &str) -> String {
    if name.contains('/') {
        return name.to_string();
    }

    format!("{default_owner}/{name}")
}

/// `get_owner` returns the owner of an object about to be sent to the server: the one set
/// by the caller, falling back to `default_owner` when the caller left it empty. Callers
/// must apply it before building the request ID, so that the ID in the query string and
/// the owner in the request body always agree.
pub(crate) fn get_owner(owner: &str, default_owner: &str) -> String {
    if owner.is_empty() {
        return default_owner.to_string();
    }

    owner.to_string()
}

async fn read_body(response: reqwest::Response) -> Result<Vec<u8>> {
    let status = response.status();
    let bytes = response.bytes().await?;

    if status != StatusCode::OK && status != StatusCode::FORBIDDEN {
        return Err(CasdoorError::HttpStatus {
            status: status.as_u16(),
            body: String::from_utf8_lossy(&bytes).into_owned(),
        });
    }

    Ok(bytes.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> Client {
        Client::new(
            "http://localhost:8000",
            "client-id",
            "client-secret",
            "",
            "built-in",
            "app-built-in",
        )
    }

    #[test]
    fn get_url_escapes_the_query_parameters() {
        assert_eq!(
            client().get_url("get-user", &[("id", "built-in/admin")]),
            "http://localhost:8000/api/get-user?id=built-in%2Fadmin"
        );

        assert_eq!(
            client().get_url("get-global-users", &[]),
            "http://localhost:8000/api/get-global-users?"
        );
    }

    #[test]
    fn get_id_only_adds_the_owner_of_an_unqualified_name() {
        assert_eq!(client().get_id("admin"), "built-in/admin");
        assert_eq!(client().get_id("other-org/alice"), "other-org/alice");
        assert_eq!(get_admin_id("app-built-in"), "admin/app-built-in");
    }

    #[test]
    fn get_owner_falls_back_to_the_default_owner() {
        assert_eq!(get_owner("", "built-in"), "built-in");
        assert_eq!(get_owner("other-org", "built-in"), "other-org");
    }

    #[test]
    fn with_access_token_keeps_the_original_client_unchanged() {
        let client = client();
        let user_client = client.with_access_token("access-token");

        assert_eq!(user_client.access_token.as_deref(), Some("access-token"));
        assert_eq!(client.access_token, None);
    }
}
