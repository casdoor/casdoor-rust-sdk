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
use crate::order::Order;
use crate::serde_util::null_to_default;

/// Payment has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/payment.go>.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Payment {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub display_name: String,

    // Payment provider info.
    pub provider: String,
    pub r#type: String,

    // Product info.
    #[serde(deserialize_with = "null_to_default")]
    pub products: Vec<String>,
    pub products_display_name: String,
    pub product_name: String,
    pub product_display_name: String,
    pub detail: String,
    pub currency: String,
    pub price: f64,

    // Payer info.
    pub user: String,
    pub person_name: String,
    pub person_id_card: String,
    pub person_email: String,
    pub person_phone: String,

    // Invoice info.
    pub invoice_type: String,
    pub invoice_title: String,
    pub invoice_tax_id: String,
    pub invoice_remark: String,
    pub invoice_url: String,

    // Order info.
    /// The internal order name.
    pub order: String,
    pub order_obj: Option<Order>,
    /// The order ID of the external payment provider.
    pub out_order_id: String,
    pub pay_url: String,
    /// `success_url` is redirected from `pay_url` after a successful payment.
    pub success_url: String,
    pub state: String,
    pub message: String,
}

impl Client {
    /// Get the payments of the client's organization.
    pub async fn get_payments(&self) -> Result<Vec<Payment>> {
        self.get_list("get-payments", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the payments of the client's organization, together with the total
    /// number of the payments.
    pub async fn get_pagination_payments(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Payment>, i32)> {
        self.get_pagination(
            "get-payments",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one payment by name.
    pub async fn get_payment(&self, name: &str) -> Result<Option<Payment>> {
        self.get_object("get-payment", &[("id", &self.get_id(name))])
            .await
    }

    /// Get the payments of one user.
    pub async fn get_user_payments(&self, user_name: &str) -> Result<Vec<Payment>> {
        self.get_list(
            "get-user-payments",
            &[
                ("owner", &self.config.organization_name),
                ("organization", &self.config.organization_name),
                ("user", user_name),
            ],
        )
        .await
    }

    /// Add a payment.
    pub async fn add_payment(&self, payment: &Payment) -> Result<bool> {
        Ok(self.modify_payment("add-payment", payment, &[]).await?.1)
    }

    /// Update a payment.
    pub async fn update_payment(&self, payment: &Payment) -> Result<bool> {
        Ok(self.modify_payment("update-payment", payment, &[]).await?.1)
    }

    /// Delete a payment.
    pub async fn delete_payment(&self, payment: &Payment) -> Result<bool> {
        Ok(self.modify_payment("delete-payment", payment, &[]).await?.1)
    }

    /// Notify Casdoor that a payment was made, so that it queries the payment provider and
    /// updates the state of the payment.
    pub async fn notify_payment(&self, payment: &Payment) -> Result<bool> {
        Ok(self.modify_payment("notify-payment", payment, &[]).await?.1)
    }

    /// Ask the payment provider to issue the invoice of a payment.
    pub async fn invoice_payment(&self, payment: &Payment) -> Result<bool> {
        Ok(self
            .modify_payment("invoice-payment", payment, &[])
            .await?
            .1)
    }

    async fn modify_payment(
        &self,
        action: &str,
        payment: &Payment,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut payment = payment.clone();
        payment.owner = get_owner(&payment.owner, &self.config.organization_name);

        let id = format!("{}/{}", payment.owner, payment.name);
        self.modify_object(action, &id, &payment, columns).await
    }
}
