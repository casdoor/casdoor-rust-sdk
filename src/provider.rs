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

use crate::client::{get_owner, Client, Response};
use crate::error::Result;
use crate::serde_util::null_to_default;

/// Provider has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/provider.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Provider {
    pub owner: String,
    pub name: String,
    pub created_time: String,

    pub display_name: String,
    pub category: String,
    pub r#type: String,
    pub sub_type: String,
    pub method: String,
    pub client_id: String,
    pub client_secret: String,
    pub client_id2: String,
    pub client_secret2: String,
    pub cert: String,
    pub custom_auth_url: String,
    pub custom_token_url: String,
    pub custom_user_info_url: String,
    pub custom_logout_url: String,
    pub custom_logo: String,
    pub scopes: String,
    #[serde(deserialize_with = "null_to_default")]
    pub user_mapping: HashMap<String, String>,
    #[serde(deserialize_with = "null_to_default")]
    pub http_headers: HashMap<String, String>,

    pub host: String,
    pub port: i32,
    /// Deprecated: use `ssl_mode` instead. If the provider type is WeChat, `disable_ssl`
    /// means `enableQRCode`, if the type is Google, it means "sync phone number".
    pub disable_ssl: bool,
    /// `"Auto"` (an empty value means `"Auto"`), `"Enable"` or `"Disable"`.
    pub ssl_mode: String,
    pub title: String,
    /// If the provider type is WeChat, `content` is the QR code string in Base64.
    pub content: String,
    pub receiver: String,

    pub region_id: String,
    pub sign_name: String,
    pub template_code: String,
    pub app_id: String,

    pub endpoint: String,
    pub intranet_endpoint: String,
    pub domain: String,
    pub bucket: String,
    pub path_prefix: String,

    pub metadata: String,
    pub id_p: String,
    pub issuer_url: String,
    pub enable_sign_authn_request: bool,
    pub email_regex: String,

    pub provider_url: String,
    pub enable_proxy: bool,
    pub enable_pkce: bool,

    pub state: String,
}

impl Client {
    /// Get the providers of the client's organization.
    pub async fn get_providers(&self) -> Result<Vec<Provider>> {
        self.get_list(
            "get-providers",
            &[("owner", &self.config.organization_name)],
        )
        .await
    }

    /// Get one provider by name.
    pub async fn get_provider(&self, name: &str) -> Result<Option<Provider>> {
        self.get_object("get-provider", &[("id", &self.get_id(name))])
            .await
    }

    /// Get one page of the providers of the client's organization, together with the total
    /// number of the providers.
    pub async fn get_pagination_providers(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Provider>, i32)> {
        self.get_pagination(
            "get-providers",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Add a provider.
    pub async fn add_provider(&self, provider: &Provider) -> Result<bool> {
        Ok(self.modify_provider("add-provider", provider, &[]).await?.1)
    }

    /// Update a provider.
    pub async fn update_provider(&self, provider: &Provider) -> Result<bool> {
        Ok(self
            .modify_provider("update-provider", provider, &[])
            .await?
            .1)
    }

    /// Delete a provider.
    pub async fn delete_provider(&self, provider: &Provider) -> Result<bool> {
        Ok(self
            .modify_provider("delete-provider", provider, &[])
            .await?
            .1)
    }

    async fn modify_provider(
        &self,
        action: &str,
        provider: &Provider,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut provider = provider.clone();
        provider.owner = get_owner(&provider.owner, &self.config.organization_name);

        let id = format!("{}/{}", provider.owner, provider.name);
        self.modify_object(action, &id, &provider, columns).await
    }
}
