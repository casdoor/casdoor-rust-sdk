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
pub struct TableColumn {
    pub name: String,
    pub r#type: String,
    pub casdoor_name: String,
    pub is_key: bool,
    pub is_hashed: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub values: Vec<String>,
}

/// Syncer has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/syncer.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Syncer {
    pub owner: String,
    pub name: String,
    pub created_time: String,

    pub organization: String,
    pub r#type: String,
    pub database_type: String,
    pub ssl_mode: String,
    pub ssh_type: String,

    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub ssh_host: String,
    pub ssh_port: i32,
    pub ssh_user: String,
    pub ssh_password: String,
    pub cert: String,
    pub database: String,
    pub table: String,
    #[serde(deserialize_with = "null_to_default")]
    pub table_columns: Vec<TableColumn>,
    pub affiliation_table: String,
    pub avatar_base_url: String,
    pub error_text: String,
    pub sync_interval: i32,
    pub is_read_only: bool,
    pub is_enabled: bool,
}

impl Client {
    /// Get the syncers of the client's organization.
    pub async fn get_syncers(&self) -> Result<Vec<Syncer>> {
        self.get_list("get-syncers", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the syncers of the client's organization, together with the total
    /// number of the syncers.
    pub async fn get_pagination_syncers(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Syncer>, i32)> {
        self.get_pagination(
            "get-syncers",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one syncer by name.
    pub async fn get_syncer(&self, name: &str) -> Result<Option<Syncer>> {
        self.get_object("get-syncer", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a syncer.
    pub async fn add_syncer(&self, syncer: &Syncer) -> Result<bool> {
        Ok(self.modify_syncer("add-syncer", syncer, &[]).await?.1)
    }

    /// Update a syncer.
    pub async fn update_syncer(&self, syncer: &Syncer) -> Result<bool> {
        Ok(self.modify_syncer("update-syncer", syncer, &[]).await?.1)
    }

    /// Delete a syncer.
    pub async fn delete_syncer(&self, syncer: &Syncer) -> Result<bool> {
        Ok(self.modify_syncer("delete-syncer", syncer, &[]).await?.1)
    }

    async fn modify_syncer(
        &self,
        action: &str,
        syncer: &Syncer,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut syncer = syncer.clone();
        syncer.owner = get_owner(&syncer.owner, &self.config.organization_name);

        let id = format!("{}/{}", syncer.owner, syncer.name);
        self.modify_object(action, &id, &syncer, columns).await
    }
}
