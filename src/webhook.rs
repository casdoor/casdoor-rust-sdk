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

use crate::client::{get_owner, Client, Response};
use crate::error::Result;
use crate::serde_util::null_to_default;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Header {
    pub name: String,
    pub value: String,
}

/// Webhook has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/webhook.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Webhook {
    pub owner: String,
    pub name: String,
    pub created_time: String,

    pub organization: String,

    pub url: String,
    pub method: String,
    pub content_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub headers: Vec<Header>,
    #[serde(deserialize_with = "null_to_default")]
    pub events: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub token_fields: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub object_fields: Vec<String>,
    pub is_user_extended: bool,
    pub single_org_only: bool,
    pub is_enabled: bool,

    // Retry configuration.
    pub max_retries: i32,
    /// The retry interval in seconds.
    pub retry_interval: i32,
    pub use_exponential_backoff: bool,
}

impl Client {
    /// Get the webhooks of the client's organization.
    pub async fn get_webhooks(&self) -> Result<Vec<Webhook>> {
        self.get_list("get-webhooks", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the webhooks of the client's organization, together with the total
    /// number of the webhooks.
    pub async fn get_pagination_webhooks(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Webhook>, i32)> {
        self.get_pagination(
            "get-webhooks",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one webhook by name.
    pub async fn get_webhook(&self, name: &str) -> Result<Option<Webhook>> {
        self.get_object("get-webhook", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a webhook.
    pub async fn add_webhook(&self, webhook: &Webhook) -> Result<bool> {
        Ok(self.modify_webhook("add-webhook", webhook, &[]).await?.1)
    }

    /// Update a webhook.
    pub async fn update_webhook(&self, webhook: &Webhook) -> Result<bool> {
        Ok(self.modify_webhook("update-webhook", webhook, &[]).await?.1)
    }

    /// Delete a webhook.
    pub async fn delete_webhook(&self, webhook: &Webhook) -> Result<bool> {
        Ok(self.modify_webhook("delete-webhook", webhook, &[]).await?.1)
    }

    async fn modify_webhook(
        &self,
        action: &str,
        webhook: &Webhook,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut webhook = webhook.clone();
        webhook.owner = get_owner(&webhook.owner, &self.config.organization_name);

        let id = format!("{}/{}", webhook.owner, webhook.name);
        self.modify_object(action, &id, &webhook, columns).await
    }
}
