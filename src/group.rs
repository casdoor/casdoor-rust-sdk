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

/// Group has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/group.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Group {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,

    pub display_name: String,
    pub manager: String,
    pub contact_email: String,
    pub r#type: String,
    pub parent_id: String,
    pub parent_name: String,
    pub is_top_group: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub users: Vec<String>,

    pub title: String,
    pub key: String,
    pub have_children: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub children: Vec<Group>,

    pub is_enabled: bool,
    pub gid_number: i32,
    #[serde(deserialize_with = "null_to_default")]
    pub properties: HashMap<String, String>,
}

impl Client {
    /// Get the groups of the client's organization.
    pub async fn get_groups(&self) -> Result<Vec<Group>> {
        self.get_list("get-groups", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the groups of the client's organization, together with the total
    /// number of the groups.
    pub async fn get_pagination_groups(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Group>, i32)> {
        self.get_pagination(
            "get-groups",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one group by name.
    pub async fn get_group(&self, name: &str) -> Result<Option<Group>> {
        self.get_object("get-group", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a group.
    pub async fn add_group(&self, group: &Group) -> Result<bool> {
        Ok(self.modify_group("add-group", group, &[]).await?.1)
    }

    /// Update a group.
    pub async fn update_group(&self, group: &Group) -> Result<bool> {
        Ok(self.modify_group("update-group", group, &[]).await?.1)
    }

    /// Delete a group.
    pub async fn delete_group(&self, group: &Group) -> Result<bool> {
        Ok(self.modify_group("delete-group", group, &[]).await?.1)
    }

    async fn modify_group(
        &self,
        action: &str,
        group: &Group,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut group = group.clone();
        group.owner = get_owner(&group.owner, &self.config.organization_name);

        let id = format!("{}/{}", group.owner, group.name);
        self.modify_object(action, &id, &group, columns).await
    }
}
