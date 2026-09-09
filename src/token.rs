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

use serde::{Deserialize, Serialize};

use crate::client::{get_admin_id, get_owner, Client, PostBody, Response};
use crate::error::Result;
use crate::serde_util::null_to_default;

/// Token has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/token.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Token {
    pub owner: String,
    pub name: String,
    pub created_time: String,

    pub application: String,
    pub organization: String,
    pub user: String,

    pub code: String,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_hash: String,
    pub refresh_token_hash: String,
    pub expires_in: i32,
    pub scope: String,
    pub token_type: String,
    pub grant_type: String,
    pub code_challenge: String,
    pub code_is_used: bool,
    pub code_expire_in: i64,
    /// RFC 8707 Resource Indicator.
    pub resource: String,
    /// RFC 9449 DPoP JWK thumbprint binding.
    #[serde(rename = "dPoPJkt")]
    pub dpop_jkt: String,
    pub session_id: String,
}

/// The result of the RFC 7662 token introspection.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct IntrospectTokenResult {
    pub active: bool,
    pub client_id: String,
    pub username: String,
    pub token_type: String,
    pub exp: u64,
    pub iat: u64,
    pub nbf: u64,
    pub sub: String,
    #[serde(deserialize_with = "null_to_default")]
    pub aud: Vec<String>,
    pub iss: String,
    pub jti: String,
}

impl Client {
    /// Get all the tokens.
    pub async fn get_tokens(&self) -> Result<Vec<Token>> {
        self.get_list("get-tokens", &[("owner", "admin")]).await
    }

    /// Get one page of the tokens, together with the total number of the tokens.
    pub async fn get_pagination_tokens(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Token>, i32)> {
        self.get_pagination("get-tokens", "admin", p, page_size, query)
            .await
    }

    /// Get one token by name.
    pub async fn get_token(&self, name: &str) -> Result<Option<Token>> {
        self.get_object("get-token", &[("id", &get_admin_id(name))])
            .await
    }

    /// Add a token.
    pub async fn add_token(&self, token: &Token) -> Result<bool> {
        Ok(self.modify_token("add-token", token, &[]).await?.1)
    }

    /// Update a token.
    pub async fn update_token(&self, token: &Token) -> Result<bool> {
        Ok(self.modify_token("update-token", token, &[]).await?.1)
    }

    /// Update only the given columns of a token.
    pub async fn update_token_for_columns(&self, token: &Token, columns: &[&str]) -> Result<bool> {
        Ok(self.modify_token("update-token", token, columns).await?.1)
    }

    /// Delete a token.
    pub async fn delete_token(&self, token: &Token) -> Result<bool> {
        Ok(self.modify_token("delete-token", token, &[]).await?.1)
    }

    /// Introspect an access token or a refresh token, `token_type_hint` is
    /// `"access_token"` or `"refresh_token"`.
    pub async fn introspect_token(
        &self,
        token: &str,
        token_type_hint: &str,
    ) -> Result<IntrospectTokenResult> {
        let fields = HashMap::from([
            ("token".to_string(), token.to_string()),
            ("token_type_hint".to_string(), token_type_hint.to_string()),
        ]);

        let url = self.get_url("login/oauth/introspect", &[]);
        let bytes = self.do_post_bytes_raw(&url, PostBody::Form(fields)).await?;

        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn modify_token(
        &self,
        action: &str,
        token: &Token,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut token = token.clone();
        token.owner = get_owner(&token.owner, "admin");

        let id = format!("{}/{}", token.owner, token.name);
        self.modify_object(action, &id, &token, columns).await
    }
}
