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

/// Adapter has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/adapter.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Adapter {
    pub owner: String,
    pub name: String,
    pub created_time: String,

    pub table: String,
    pub use_same_db: bool,
    pub r#type: String,
    pub database_type: String,
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl Client {
    /// Get the adapters of the client's organization.
    pub async fn get_adapters(&self) -> Result<Vec<Adapter>> {
        self.get_list("get-adapters", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the adapters of the client's organization, together with the total
    /// number of the adapters.
    pub async fn get_pagination_adapters(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Adapter>, i32)> {
        self.get_pagination(
            "get-adapters",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one adapter by name.
    pub async fn get_adapter(&self, name: &str) -> Result<Option<Adapter>> {
        self.get_object("get-adapter", &[("id", &self.get_id(name))])
            .await
    }

    /// Add an adapter.
    pub async fn add_adapter(&self, adapter: &Adapter) -> Result<bool> {
        Ok(self.modify_adapter("add-adapter", adapter, &[]).await?.1)
    }

    /// Update an adapter.
    pub async fn update_adapter(&self, adapter: &Adapter) -> Result<bool> {
        Ok(self.modify_adapter("update-adapter", adapter, &[]).await?.1)
    }

    /// Delete an adapter.
    pub async fn delete_adapter(&self, adapter: &Adapter) -> Result<bool> {
        Ok(self.modify_adapter("delete-adapter", adapter, &[]).await?.1)
    }

    async fn modify_adapter(
        &self,
        action: &str,
        adapter: &Adapter,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut adapter = adapter.clone();
        adapter.owner = get_owner(&adapter.owner, &self.config.organization_name);

        let id = format!("{}/{}", adapter.owner, adapter.name);
        self.modify_object(action, &id, &adapter, columns).await
    }
}
