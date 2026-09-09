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

/// Enforcer has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/enforcer.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Enforcer {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub display_name: String,
    pub description: String,

    pub model: String,
    pub adapter: String,

    #[serde(deserialize_with = "null_to_default")]
    pub model_cfg: HashMap<String, String>,
}

impl Client {
    /// Get the enforcers of the client's organization.
    pub async fn get_enforcers(&self) -> Result<Vec<Enforcer>> {
        self.get_list(
            "get-enforcers",
            &[("owner", &self.config.organization_name)],
        )
        .await
    }

    /// Get one page of the enforcers of the client's organization, together with the total
    /// number of the enforcers.
    pub async fn get_pagination_enforcers(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Enforcer>, i32)> {
        self.get_pagination(
            "get-enforcers",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one enforcer by name.
    pub async fn get_enforcer(&self, name: &str) -> Result<Option<Enforcer>> {
        self.get_object("get-enforcer", &[("id", &self.get_id(name))])
            .await
    }

    /// Add an enforcer.
    pub async fn add_enforcer(&self, enforcer: &Enforcer) -> Result<bool> {
        Ok(self.modify_enforcer("add-enforcer", enforcer, &[]).await?.1)
    }

    /// Update an enforcer.
    pub async fn update_enforcer(&self, enforcer: &Enforcer) -> Result<bool> {
        Ok(self
            .modify_enforcer("update-enforcer", enforcer, &[])
            .await?
            .1)
    }

    /// Delete an enforcer.
    pub async fn delete_enforcer(&self, enforcer: &Enforcer) -> Result<bool> {
        Ok(self
            .modify_enforcer("delete-enforcer", enforcer, &[])
            .await?
            .1)
    }

    async fn modify_enforcer(
        &self,
        action: &str,
        enforcer: &Enforcer,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut enforcer = enforcer.clone();
        enforcer.owner = get_owner(&enforcer.owner, &self.config.organization_name);

        let id = format!("{}/{}", enforcer.owner, enforcer.name);
        self.modify_object(action, &id, &enforcer, columns).await
    }
}
