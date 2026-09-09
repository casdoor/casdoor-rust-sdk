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

use crate::client::{get_owner, Client, PostBody, Response};
use crate::error::Result;
use crate::serde_util::null_to_default;

/// Order has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/order.go>.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Order {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub update_time: String,
    pub display_name: String,

    // Product info.
    #[serde(deserialize_with = "null_to_default")]
    pub products: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub product_infos: Vec<ProductInfo>,

    // User info.
    pub user: String,

    // Payment info.
    pub payment: String,
    pub price: f64,
    pub currency: String,

    // Order state.
    pub state: String,
    pub message: String,

    // Coupon info.
    pub coupon_name: String,
    /// The discount amount applied by the coupon.
    pub coupon_discount: f64,
}

/// One product of an [`Order`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProductInfo {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub display_name: String,
    pub image: String,
    pub detail: String,
    pub price: f64,
    pub currency: String,
    pub is_recharge: bool,
    pub quantity: i32,
    pub pricing_name: String,
    pub plan_name: String,
}

impl Order {
    /// Return the Casdoor object ID (`"owner/name"`) of the order.
    pub fn get_id(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

impl Client {
    /// Get the orders of the client's organization.
    pub async fn get_orders(&self) -> Result<Vec<Order>> {
        self.get_list("get-orders", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the orders of the client's organization, together with the total
    /// number of the orders.
    pub async fn get_pagination_orders(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Order>, i32)> {
        self.get_pagination(
            "get-orders",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get the orders of one user.
    pub async fn get_user_orders(&self, user_name: &str) -> Result<Vec<Order>> {
        self.get_list(
            "get-user-orders",
            &[
                ("owner", &self.config.organization_name),
                ("user", user_name),
            ],
        )
        .await
    }

    /// Get one order by name.
    pub async fn get_order(&self, name: &str) -> Result<Option<Order>> {
        self.get_object("get-order", &[("id", &self.get_id(name))])
            .await
    }

    /// Add an order.
    pub async fn add_order(&self, order: &Order) -> Result<bool> {
        Ok(self.modify_order("add-order", order, &[]).await?.1)
    }

    /// Update an order.
    pub async fn update_order(&self, order: &Order) -> Result<bool> {
        Ok(self.modify_order("update-order", order, &[]).await?.1)
    }

    /// Delete an order.
    pub async fn delete_order(&self, order: &Order) -> Result<bool> {
        Ok(self.modify_order("delete-order", order, &[]).await?.1)
    }

    /// Cancel an order by name.
    pub async fn cancel_order(&self, name: &str) -> Result<bool> {
        let response = self
            .do_post(
                "cancel-order",
                &[("id", &self.get_id(name))],
                PostBody::Raw(Vec::new()),
            )
            .await?;

        Ok(response.data.as_str() == Some("Affected"))
    }

    async fn modify_order(
        &self,
        action: &str,
        order: &Order,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut order = order.clone();
        order.owner = get_owner(&order.owner, &self.config.organization_name);

        let id = format!("{}/{}", order.owner, order.name);
        self.modify_object(action, &id, &order, columns).await
    }
}
