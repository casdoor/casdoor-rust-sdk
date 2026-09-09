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

/// The subscription is waiting for its payment.
pub const SUB_STATE_PENDING: &str = "Pending";
/// The subscription failed.
pub const SUB_STATE_ERROR: &str = "Error";
/// The subscription was suspended by the admin.
pub const SUB_STATE_SUSPENDED: &str = "Suspended";
/// The subscription is running.
pub const SUB_STATE_ACTIVE: &str = "Active";
/// The subscription has not started yet.
pub const SUB_STATE_UPCOMING: &str = "Upcoming";
/// The subscription has ended.
pub const SUB_STATE_EXPIRED: &str = "Expired";

/// Subscription has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/subscription.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Subscription {
    pub owner: String,
    pub name: String,
    pub display_name: String,
    pub created_time: String,
    pub description: String,

    pub user: String,
    pub pricing: String,
    pub plan: String,
    pub payment: String,

    pub start_time: String,
    pub end_time: String,
    pub period: String,
    /// One of the `SUB_STATE_*` constants of this module.
    pub state: String,
}

impl Client {
    /// Get the subscriptions of the client's organization.
    pub async fn get_subscriptions(&self) -> Result<Vec<Subscription>> {
        self.get_list(
            "get-subscriptions",
            &[("owner", &self.config.organization_name)],
        )
        .await
    }

    /// Get one page of the subscriptions of the client's organization, together with the
    /// total number of the subscriptions.
    pub async fn get_pagination_subscriptions(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Subscription>, i32)> {
        self.get_pagination(
            "get-subscriptions",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one subscription by name.
    pub async fn get_subscription(&self, name: &str) -> Result<Option<Subscription>> {
        self.get_object("get-subscription", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a subscription.
    pub async fn add_subscription(&self, subscription: &Subscription) -> Result<bool> {
        Ok(self
            .modify_subscription("add-subscription", subscription, &[])
            .await?
            .1)
    }

    /// Update a subscription.
    pub async fn update_subscription(&self, subscription: &Subscription) -> Result<bool> {
        Ok(self
            .modify_subscription("update-subscription", subscription, &[])
            .await?
            .1)
    }

    /// Delete a subscription.
    pub async fn delete_subscription(&self, subscription: &Subscription) -> Result<bool> {
        Ok(self
            .modify_subscription("delete-subscription", subscription, &[])
            .await?
            .1)
    }

    async fn modify_subscription(
        &self,
        action: &str,
        subscription: &Subscription,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut subscription = subscription.clone();
        subscription.owner = get_owner(&subscription.owner, &self.config.organization_name);

        let id = format!("{}/{}", subscription.owner, subscription.name);
        self.modify_object(action, &id, &subscription, columns)
            .await
    }
}
