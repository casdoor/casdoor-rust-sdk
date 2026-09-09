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

/// Permission has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/permission.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Permission {
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

    #[serde(deserialize_with = "null_to_default")]
    pub source_groups: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub source_roles: Vec<String>,

    pub model: String,
    pub adapter: String,
    pub resource_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub resources: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub actions: Vec<String>,
    pub effect: String,
    pub is_enabled: bool,

    /// An optional RFC3339 timestamp. When set and reached, the permission is
    /// automatically revoked (its Casbin policies are removed and it is disabled) by the
    /// permission expiration job, providing time-limited access as required by standards
    /// such as ISO/IEC 27001 control 5.18. An empty value means the permission never
    /// expires.
    pub expire_time: String,

    pub submitter: String,
    pub approver: String,
    pub approve_time: String,
    pub state: String,
}

impl Client {
    /// Get the permissions of the client's organization.
    pub async fn get_permissions(&self) -> Result<Vec<Permission>> {
        self.get_list(
            "get-permissions",
            &[("owner", &self.config.organization_name)],
        )
        .await
    }

    /// Get the permissions of a role.
    pub async fn get_permissions_by_role(&self, name: &str) -> Result<Vec<Permission>> {
        self.get_list("get-permissions-by-role", &[("id", &self.get_id(name))])
            .await
    }

    /// Get one page of the permissions of the client's organization, together with the
    /// total number of the permissions.
    pub async fn get_pagination_permissions(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Permission>, i32)> {
        self.get_pagination(
            "get-permissions",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one permission by name.
    pub async fn get_permission(&self, name: &str) -> Result<Option<Permission>> {
        self.get_object("get-permission", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a permission.
    pub async fn add_permission(&self, permission: &Permission) -> Result<bool> {
        Ok(self
            .modify_permission("add-permission", permission, &[])
            .await?
            .1)
    }

    /// Update a permission.
    pub async fn update_permission(&self, permission: &Permission) -> Result<bool> {
        Ok(self
            .modify_permission("update-permission", permission, &[])
            .await?
            .1)
    }

    /// Update only the given columns of a permission.
    pub async fn update_permission_for_columns(
        &self,
        permission: &Permission,
        columns: &[&str],
    ) -> Result<bool> {
        Ok(self
            .modify_permission("update-permission", permission, columns)
            .await?
            .1)
    }

    /// Delete a permission.
    pub async fn delete_permission(&self, permission: &Permission) -> Result<bool> {
        Ok(self
            .modify_permission("delete-permission", permission, &[])
            .await?
            .1)
    }

    async fn modify_permission(
        &self,
        action: &str,
        permission: &Permission,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut permission = permission.clone();
        permission.owner = get_owner(&permission.owner, &self.config.organization_name);

        let id = format!("{}/{}", permission.owner, permission.name);
        self.modify_object(action, &id, &permission, columns).await
    }
}
