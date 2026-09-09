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
use crate::provider::Provider;
use crate::serde_util::null_to_default;

/// Product has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/product.go>.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Product {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub display_name: String,

    pub image: String,
    pub detail: String,
    pub description: String,
    pub tag: String,
    pub currency: String,
    pub price: f64,
    pub quantity: i32,
    pub sold: i32,
    pub is_recharge: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub recharge_options: Vec<f64>,
    pub disable_custom_recharge: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub providers: Vec<String>,
    pub success_url: String,

    pub state: String,

    #[serde(deserialize_with = "null_to_default")]
    pub properties: HashMap<String, String>,

    #[serde(deserialize_with = "null_to_default")]
    pub provider_objs: Vec<Provider>,
}

impl Client {
    /// Get the products of the client's organization.
    pub async fn get_products(&self) -> Result<Vec<Product>> {
        self.get_list("get-products", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the products of the client's organization, together with the total
    /// number of the products.
    pub async fn get_pagination_products(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Product>, i32)> {
        self.get_pagination(
            "get-products",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one product by name.
    pub async fn get_product(&self, name: &str) -> Result<Option<Product>> {
        self.get_object("get-product", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a product.
    pub async fn add_product(&self, product: &Product) -> Result<bool> {
        Ok(self.modify_product("add-product", product, &[]).await?.1)
    }

    /// Update a product.
    pub async fn update_product(&self, product: &Product) -> Result<bool> {
        Ok(self.modify_product("update-product", product, &[]).await?.1)
    }

    /// Delete a product.
    pub async fn delete_product(&self, product: &Product) -> Result<bool> {
        Ok(self.modify_product("delete-product", product, &[]).await?.1)
    }

    async fn modify_product(
        &self,
        action: &str,
        product: &Product,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut product = product.clone();
        product.owner = get_owner(&product.owner, &self.config.organization_name);

        let id = format!("{}/{}", product.owner, product.name);
        self.modify_object(action, &id, &product, columns).await
    }
}
