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
use crate::error::{CasdoorError, Result};

/// Transaction has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/transaction.go>.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Transaction {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub display_name: String,

    pub application: String,
    pub domain: String,
    pub category: String,
    pub r#type: String,
    pub subtype: String,
    pub provider: String,
    pub user: String,
    pub tag: String,

    pub amount: f64,
    pub currency: String,

    pub payment: String,

    pub state: String,
}

impl Client {
    /// Get the transactions of the client's organization.
    pub async fn get_transactions(&self) -> Result<Vec<Transaction>> {
        self.get_list(
            "get-transactions",
            &[("owner", &self.config.organization_name)],
        )
        .await
    }

    /// Get one page of the transactions of the client's organization, together with the
    /// total number of the transactions.
    pub async fn get_pagination_transactions(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Transaction>, i32)> {
        self.get_pagination(
            "get-transactions",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one transaction by name.
    pub async fn get_transaction(&self, name: &str) -> Result<Option<Transaction>> {
        self.get_object("get-transaction", &[("id", &self.get_id(name))])
            .await
    }

    /// Get the transactions of one user.
    pub async fn get_user_transactions(&self, user_name: &str) -> Result<Vec<Transaction>> {
        self.get_list(
            "get-user-transactions",
            &[
                ("owner", &self.config.organization_name),
                ("user", user_name),
            ],
        )
        .await
    }

    /// Add a transaction, the returned string is the ID of the created transaction.
    pub async fn add_transaction(&self, transaction: &Transaction) -> Result<(bool, String)> {
        self.add_transaction_with_dry_run(transaction, false).await
    }

    /// Add a transaction, or only check whether it would succeed when `dry_run` is true.
    pub async fn add_transaction_with_dry_run(
        &self,
        transaction: &Transaction,
        dry_run: bool,
    ) -> Result<(bool, String)> {
        let (response, affected) = self
            .modify_transaction_with_dry_run("add-transaction", transaction, &[], dry_run)
            .await?;

        let transaction_id = response
            .data
            .as_str()
            .ok_or(CasdoorError::Casdoor(
                "failed to parse transaction id from response".to_string(),
            ))?
            .to_string();

        Ok((affected, transaction_id))
    }

    /// Update a transaction.
    pub async fn update_transaction(&self, transaction: &Transaction) -> Result<bool> {
        Ok(self
            .modify_transaction("update-transaction", transaction, &[])
            .await?
            .1)
    }

    /// Delete a transaction.
    pub async fn delete_transaction(&self, transaction: &Transaction) -> Result<bool> {
        Ok(self
            .modify_transaction("delete-transaction", transaction, &[])
            .await?
            .1)
    }

    async fn modify_transaction(
        &self,
        action: &str,
        transaction: &Transaction,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        self.modify_transaction_with_dry_run(action, transaction, columns, false)
            .await
    }

    /// `dry_run` is only applicable to the `add-transaction` action.
    async fn modify_transaction_with_dry_run(
        &self,
        action: &str,
        transaction: &Transaction,
        columns: &[&str],
        dry_run: bool,
    ) -> Result<(Response, bool)> {
        let mut transaction = transaction.clone();
        transaction.owner = get_owner(&transaction.owner, &self.config.organization_name);

        let id = format!("{}/{}", transaction.owner, transaction.name);
        let columns = columns.join(",");

        let mut query = vec![("id", id.as_str())];
        if !columns.is_empty() {
            query.push(("columns", columns.as_str()));
        }
        if dry_run && action == "add-transaction" {
            query.push(("dryRun", "1"));
        }

        let post_bytes = serde_json::to_vec(&transaction)?;
        let response = self
            .do_post(action, &query, PostBody::Raw(post_bytes))
            .await?;

        let affected = response.data.as_str() == Some("Affected");
        Ok((response, affected))
    }
}
