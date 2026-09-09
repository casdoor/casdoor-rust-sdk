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

use crate::client::{get_owner, Client, PostBody};
use crate::enforcer::Enforcer;
use crate::error::Result;
use crate::serde_util::null_to_default;

/// CasbinRule is one policy rule stored by a Casbin adapter. Its JSON fields are
/// capitalized, as the Casbin xorm adapter defines them without JSON tags.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct CasbinRule {
    pub id: i64,
    pub ptype: String,
    pub v0: String,
    pub v1: String,
    pub v2: String,
    pub v3: String,
    pub v4: String,
    pub v5: String,
}

/// A filter used by [`Client::get_filtered_policies()`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PolicyFilter {
    pub ptype: String,
    pub field_index: Option<i32>,
    #[serde(deserialize_with = "null_to_default")]
    pub field_values: Vec<String>,
}

impl Client {
    /// Get the policies of an enforcer. `adapter_id` can be empty to use the adapter of
    /// the enforcer.
    pub async fn get_policies(
        &self,
        enforcer_name: &str,
        adapter_id: &str,
    ) -> Result<Vec<CasbinRule>> {
        self.get_list(
            "get-policies",
            &[
                ("id", &self.get_id(enforcer_name)),
                ("adapterId", adapter_id),
            ],
        )
        .await
    }

    /// Get the policies of an enforcer, filtered by the field index and the field values.
    pub async fn get_filtered_policies(
        &self,
        enforcer_id: &str,
        filters: &[PolicyFilter],
    ) -> Result<Vec<CasbinRule>> {
        let post_bytes = serde_json::to_vec(filters)?;

        let response = self
            .do_post(
                "get-filtered-policies",
                &[("id", enforcer_id)],
                PostBody::Raw(post_bytes),
            )
            .await?;

        if response.data.is_null() {
            return Ok(Vec::new());
        }

        Ok(serde_json::from_value(response.data)?)
    }

    /// Add one policy to an enforcer.
    pub async fn add_policy(&self, enforcer: &Enforcer, policy: &CasbinRule) -> Result<bool> {
        self.modify_policy("add-policy", enforcer, policy).await
    }

    /// Update one policy of an enforcer.
    pub async fn update_policy(
        &self,
        enforcer: &Enforcer,
        old_policy: &CasbinRule,
        new_policy: &CasbinRule,
    ) -> Result<bool> {
        self.modify_policy(
            "update-policy",
            enforcer,
            &[old_policy.clone(), new_policy.clone()],
        )
        .await
    }

    /// Remove one policy from an enforcer.
    pub async fn remove_policy(&self, enforcer: &Enforcer, policy: &CasbinRule) -> Result<bool> {
        self.modify_policy("remove-policy", enforcer, policy).await
    }

    async fn modify_policy<T: Serialize>(
        &self,
        action: &str,
        enforcer: &Enforcer,
        policies: &T,
    ) -> Result<bool> {
        let owner = get_owner(&enforcer.owner, &self.config.organization_name);
        let id = format!("{}/{}", owner, enforcer.name);

        Ok(self.modify_object(action, &id, policies, &[]).await?.1)
    }
}
