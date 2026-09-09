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

use crate::client::Client;

impl Client {
    /// Return the URL of the Casdoor sign-up page. `redirect_uri` can be empty when
    /// `enable_password` is true, since only the password based sign-up page is needed
    /// then.
    pub fn get_signup_url(&self, enable_password: bool, redirect_uri: &str) -> String {
        if enable_password {
            return format!(
                "{}/signup/{}",
                self.config.endpoint, self.config.application_name
            );
        }

        self.get_signin_url(redirect_uri)
            .replace("/login/oauth/authorize", "/signup/oauth/authorize")
    }

    /// Return the URL of the Casdoor sign-in page, the user is redirected to
    /// `redirect_uri` with an authorization code after signing in.
    pub fn get_signin_url(&self, redirect_uri: &str) -> String {
        let scope = "read";
        let state = &self.config.application_name;

        format!(
            "{}/login/oauth/authorize?client_id={}&response_type=code&redirect_uri={}&scope={}&state={}",
            self.config.endpoint,
            self.config.client_id,
            urlencoding::encode(redirect_uri),
            scope,
            state
        )
    }

    /// Return the URL of a user's profile page.
    pub fn get_user_profile_url(&self, user_name: &str, access_token: Option<&str>) -> String {
        format!(
            "{}/users/{}/{}{}",
            self.config.endpoint,
            self.config.organization_name,
            user_name,
            access_token_param(access_token)
        )
    }

    /// Return the URL of the account page of the user who owns the access token.
    pub fn get_my_profile_url(&self, access_token: Option<&str>) -> String {
        format!(
            "{}/account{}",
            self.config.endpoint,
            access_token_param(access_token)
        )
    }
}

fn access_token_param(access_token: Option<&str>) -> String {
    match access_token {
        Some(token) if !token.is_empty() => format!("?access_token={token}"),
        _ => String::new(),
    }
}
