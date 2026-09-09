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

/// Role has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/role.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Role {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub display_name: String,
    pub description: String,

    #[serde(deserialize_with = "null_to_default")]
    pub users: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub groups: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub roles: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub domains: Vec<String>,
    pub is_enabled: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub source_groups: Vec<String>,
}

impl Client {
    /// Get the roles of the client's organization.
    pub async fn get_roles(&self) -> Result<Vec<Role>> {
        self.get_list("get-roles", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the roles of the client's organization, together with the total
    /// number of the roles.
    pub async fn get_pagination_roles(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Role>, i32)> {
        self.get_pagination(
            "get-roles",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one role by name.
    pub async fn get_role(&self, name: &str) -> Result<Option<Role>> {
        self.get_object("get-role", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a role.
    pub async fn add_role(&self, role: &Role) -> Result<bool> {
        Ok(self.modify_role("add-role", role, &[]).await?.1)
    }

    /// Update a role.
    pub async fn update_role(&self, role: &Role) -> Result<bool> {
        Ok(self.modify_role("update-role", role, &[]).await?.1)
    }

    /// Update only the given columns of a role.
    pub async fn update_role_for_columns(&self, role: &Role, columns: &[&str]) -> Result<bool> {
        Ok(self.modify_role("update-role", role, columns).await?.1)
    }

    /// Delete a role.
    pub async fn delete_role(&self, role: &Role) -> Result<bool> {
        Ok(self.modify_role("delete-role", role, &[]).await?.1)
    }

    async fn modify_role(
        &self,
        action: &str,
        role: &Role,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut role = role.clone();
        role.owner = get_owner(&role.owner, &self.config.organization_name);

        let id = format!("{}/{}", role.owner, role.name);
        self.modify_object(action, &id, &role, columns).await
    }
}
