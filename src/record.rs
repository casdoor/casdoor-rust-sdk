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

use crate::client::{Client, PostBody};
use crate::error::Result;

/// Record has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/record.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Record {
    pub id: i32,

    pub owner: String,
    pub name: String,
    pub created_time: String,

    pub organization: String,
    pub client_ip: String,
    pub user: String,
    pub method: String,
    pub request_uri: String,
    pub action: String,
    pub language: String,

    pub object: String,
    pub response: String,
    pub status_code: i32,

    pub detail: String,

    pub is_triggered: bool,
}

impl Client {
    /// Get the records of the client's organization.
    pub async fn get_records(&self) -> Result<Vec<Record>> {
        self.get_list("get-records", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the records of the client's organization, together with the total
    /// number of the records.
    pub async fn get_pagination_records(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Record>, i32)> {
        self.get_pagination(
            "get-records",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one record by name.
    pub async fn get_record(&self, name: &str) -> Result<Option<Record>> {
        self.get_object("get-record", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a record.
    pub async fn add_record(&self, record: &Record) -> Result<bool> {
        let mut record = record.clone();
        if record.owner.is_empty() {
            record.owner = self.config.organization_name.clone();
        }
        if record.organization.is_empty() {
            record.organization = self.config.organization_name.clone();
        }

        let post_bytes = serde_json::to_vec(&record)?;
        let response = self
            .do_post("add-record", &[], PostBody::Raw(post_bytes))
            .await?;

        Ok(response.data.as_str() == Some("Affected"))
    }
}
