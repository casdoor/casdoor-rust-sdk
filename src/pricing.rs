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

/// Pricing has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/pricing.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Pricing {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub display_name: String,
    pub description: String,

    #[serde(deserialize_with = "null_to_default")]
    pub plans: Vec<String>,
    pub is_enabled: bool,
    pub trial_duration: i32,
    pub application: String,
}

impl Client {
    /// Get the pricings of the client's organization.
    pub async fn get_pricings(&self) -> Result<Vec<Pricing>> {
        self.get_list("get-pricings", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the pricings of the client's organization, together with the total
    /// number of the pricings.
    pub async fn get_pagination_pricings(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Pricing>, i32)> {
        self.get_pagination(
            "get-pricings",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one pricing by name.
    pub async fn get_pricing(&self, name: &str) -> Result<Option<Pricing>> {
        self.get_object("get-pricing", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a pricing.
    pub async fn add_pricing(&self, pricing: &Pricing) -> Result<bool> {
        Ok(self.modify_pricing("add-pricing", pricing, &[]).await?.1)
    }

    /// Update a pricing.
    pub async fn update_pricing(&self, pricing: &Pricing) -> Result<bool> {
        Ok(self.modify_pricing("update-pricing", pricing, &[]).await?.1)
    }

    /// Delete a pricing.
    pub async fn delete_pricing(&self, pricing: &Pricing) -> Result<bool> {
        Ok(self.modify_pricing("delete-pricing", pricing, &[]).await?.1)
    }

    async fn modify_pricing(
        &self,
        action: &str,
        pricing: &Pricing,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut pricing = pricing.clone();
        pricing.owner = get_owner(&pricing.owner, &self.config.organization_name);

        let id = format!("{}/{}", pricing.owner, pricing.name);
        self.modify_object(action, &id, &pricing, columns).await
    }
}
