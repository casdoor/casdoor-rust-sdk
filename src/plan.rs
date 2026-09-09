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

/// Plan has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/plan.go>.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Plan {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub display_name: String,
    pub description: String,

    pub price: f64,
    pub currency: String,
    pub period: String,
    pub product: String,
    /// The payment providers of the related product.
    #[serde(deserialize_with = "null_to_default")]
    pub payment_providers: Vec<String>,
    pub is_enabled: bool,
    /// If true, a user can only have at most one subscription of this plan.
    pub is_exclusive: bool,

    pub role: String,
    #[serde(deserialize_with = "null_to_default")]
    pub options: Vec<String>,
}

impl Client {
    /// Get the plans of the client's organization.
    pub async fn get_plans(&self) -> Result<Vec<Plan>> {
        self.get_list("get-plans", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the plans of the client's organization, together with the total
    /// number of the plans.
    pub async fn get_pagination_plans(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Plan>, i32)> {
        self.get_pagination(
            "get-plans",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one plan by name.
    pub async fn get_plan(&self, name: &str) -> Result<Option<Plan>> {
        self.get_object("get-plan", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a plan.
    pub async fn add_plan(&self, plan: &Plan) -> Result<bool> {
        Ok(self.modify_plan("add-plan", plan, &[]).await?.1)
    }

    /// Update a plan.
    pub async fn update_plan(&self, plan: &Plan) -> Result<bool> {
        Ok(self.modify_plan("update-plan", plan, &[]).await?.1)
    }

    /// Delete a plan.
    pub async fn delete_plan(&self, plan: &Plan) -> Result<bool> {
        Ok(self.modify_plan("delete-plan", plan, &[]).await?.1)
    }

    async fn modify_plan(
        &self,
        action: &str,
        plan: &Plan,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut plan = plan.clone();
        plan.owner = get_owner(&plan.owner, &self.config.organization_name);

        let id = format!("{}/{}", plan.owner, plan.name);
        self.modify_object(action, &id, &plan, columns).await
    }
}
