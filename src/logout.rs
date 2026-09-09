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

use crate::client::{Client, Response};
use crate::error::{CasdoorError, Result};

impl Client {
    /// Log out the user that owns the `access_token` by calling the `/api/sso-logout` API.
    /// It performs a single sign-out: all the sessions of the user across all the
    /// applications of the organization are deleted and all the access tokens issued to
    /// the user are expired.
    ///
    /// The `access_token` is the user's access token returned by
    /// [`Client::get_oauth_token()`].
    pub async fn logout(&self, access_token: &str) -> Result<()> {
        self.sso_logout(access_token, true).await
    }

    /// Like [`Client::logout()`], but only ends the session that the `access_token`
    /// belongs to, so the user stays signed in on their other devices and browsers.
    pub async fn logout_current_session(&self, access_token: &str) -> Result<()> {
        self.sso_logout(access_token, false).await
    }

    async fn sso_logout(&self, access_token: &str, logout_all: bool) -> Result<()> {
        if access_token.is_empty() {
            return Err(CasdoorError::Casdoor(
                "logout() error: the accessToken should not be empty".to_string(),
            ));
        }

        let url = self.get_url("sso-logout", &[("logoutAll", &logout_all.to_string())]);

        // The "/api/sso-logout" API identifies the user by their own access token, so the
        // Bearer token is used here instead of the application's Basic Auth.
        let mut req = self.http().post(url).bearer_auth(access_token);
        for (key, value) in &self.custom_headers {
            req = req.header(key, value);
        }

        let response = req.send().await?;
        let status = response.status();
        let bytes = response.bytes().await?;

        if status != reqwest::StatusCode::OK && status != reqwest::StatusCode::FORBIDDEN {
            return Err(CasdoorError::HttpStatus {
                status: status.as_u16(),
                body: String::from_utf8_lossy(&bytes).into_owned(),
            });
        }

        let response: Response = serde_json::from_slice(&bytes)?;
        if response.status != "ok" {
            return Err(CasdoorError::Casdoor(response.msg));
        }

        Ok(())
    }
}
