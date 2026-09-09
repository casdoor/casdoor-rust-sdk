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

use serde::Serialize;

use crate::client::{Client, PostBody};
use crate::error::Result;
use crate::order::{Order, ProductInfo};
use crate::payment::Payment;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PlaceOrderRequest<'a> {
    product_infos: &'a [ProductInfo],
}

impl Client {
    /// Place an order for the given products. `user_name` can be empty to place the order
    /// for the user that the client is authenticated as.
    pub async fn place_order(
        &self,
        product_infos: &[ProductInfo],
        user_name: &str,
    ) -> Result<Order> {
        let mut query = vec![("owner", self.config.organization_name.as_str())];
        if !user_name.is_empty() {
            query.push(("userName", user_name));
        }

        let post_bytes = serde_json::to_vec(&PlaceOrderRequest { product_infos })?;
        let response = self
            .do_post("place-order", &query, PostBody::Raw(post_bytes))
            .await?;

        Ok(serde_json::from_value(response.data)?)
    }

    /// Pay an order with a payment provider, the returned payment holds the `pay_url` the
    /// user has to be redirected to.
    pub async fn pay_order(&self, order_name: &str, provider_name: &str) -> Result<Payment> {
        let response = self
            .do_post(
                "pay-order",
                &[
                    ("id", &self.get_id(order_name)),
                    ("providerName", provider_name),
                ],
                PostBody::Raw(Vec::new()),
            )
            .await?;

        Ok(serde_json::from_value(response.data)?)
    }

    /// Place an order of one product, kept for compatibility with the old `buy-product`
    /// API.
    pub async fn buy_product(&self, name: &str, user_name: &str) -> Result<Order> {
        let product_info = ProductInfo {
            name: name.to_string(),
            quantity: 1,
            ..Default::default()
        };

        self.place_order(&[product_info], user_name).await
    }
}
