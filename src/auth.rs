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

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{CasdoorError, Result};

/// The OAuth token returned by the Casdoor server.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OAuthToken {
    #[serde(default)]
    pub access_token: String,
    #[serde(default)]
    pub token_type: String,
    #[serde(default)]
    pub refresh_token: String,
    #[serde(default)]
    pub expires_in: i64,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub id_token: String,
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub error_description: String,
}

impl Client {
    /// Get the pivotal and necessary secret to interact with the Casdoor server, by
    /// exchanging the authorization code that Casdoor sent back to the redirect URI.
    pub async fn get_oauth_token(&self, code: &str) -> Result<OAuthToken> {
        self.request_oauth_token(
            "access_token",
            &[("grant_type", "authorization_code"), ("code", code)],
        )
        .await
    }

    /// Refresh the OAuth token with a refresh token.
    pub async fn refresh_oauth_token(&self, refresh_token: &str) -> Result<OAuthToken> {
        self.request_oauth_token(
            "refresh_token",
            &[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
            ],
        )
        .await
    }

    /// Get the OAuth token via the `password` grant type, i.e. the "Resource Owner
    /// Password Credentials Grant" of OAuth 2.0. The `password` grant type must be enabled
    /// in the application's "Grant types" in Casdoor.
    ///
    /// The username is the user's name inside the application's organization, like
    /// `alice` instead of `my-org/alice`.
    pub async fn get_oauth_token_by_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<OAuthToken> {
        self.request_oauth_token(
            "access_token",
            &[
                ("grant_type", "password"),
                ("username", username),
                ("password", password),
            ],
        )
        .await
    }

    /// Get an OAuth token which acts as the given user, so that an admin can call the APIs
    /// on behalf of the user, without knowing the user's own password. It's the SDK
    /// equivalent of the "Impersonation" button in Casdoor's Web UI.
    ///
    /// The `master_password` is the "Master password" of the user's organization, it needs
    /// to be set in Casdoor first: "Organizations" -> Edit the organization -> "Master
    /// password". See <https://casdoor.org/docs/user/impersonation>.
    pub async fn impersonate_user(
        &self,
        username: &str,
        master_password: &str,
    ) -> Result<OAuthToken> {
        self.get_oauth_token_by_password(username, master_password)
            .await
    }

    /// Post the OAuth parameters to `/api/login/oauth/{token_action}`, the client ID and
    /// the client secret are sent in the request body, as Casdoor expects.
    async fn request_oauth_token(
        &self,
        token_action: &str,
        params: &[(&str, &str)],
    ) -> Result<OAuthToken> {
        let url = format!("{}/api/login/oauth/{}", self.config.endpoint, token_action);

        let mut form = params.to_vec();
        form.push(("client_id", self.config.client_id.as_str()));
        form.push(("client_secret", self.config.client_secret.as_str()));

        let mut req = self.http().post(url).form(&form);
        for (key, value) in &self.custom_headers {
            req = req.header(key, value);
        }

        let response = req.send().await?;
        let status = response.status();
        let bytes = response.bytes().await?;

        let token: OAuthToken = serde_json::from_slice(&bytes).map_err(|e| {
            if status.is_success() {
                CasdoorError::Json(e)
            } else {
                CasdoorError::HttpStatus {
                    status: status.as_u16(),
                    body: String::from_utf8_lossy(&bytes).into_owned(),
                }
            }
        })?;

        check_oauth_token(token)
    }
}

/// Convert the `error: xxx` access token returned by the Casdoor server into a real error.
fn check_oauth_token(token: OAuthToken) -> Result<OAuthToken> {
    if !token.error.is_empty() {
        let message = if token.error_description.is_empty() {
            token.error
        } else {
            format!("{}: {}", token.error, token.error_description)
        };

        return Err(CasdoorError::Casdoor(message));
    }

    if let Some(message) = token.access_token.strip_prefix("error:") {
        return Err(CasdoorError::Casdoor(message.trim().to_string()));
    }

    Ok(token)
}
