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
use crate::error::{CasdoorError, Result};

/// Resource has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/resource.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Resource {
    pub owner: String,
    pub name: String,
    pub created_time: String,

    pub user: String,
    pub provider: String,
    pub application: String,
    pub tag: String,
    pub parent: String,
    pub file_name: String,
    pub file_type: String,
    pub file_format: String,
    pub file_size: i32,
    pub url: String,
    pub description: String,
}

impl Client {
    /// Get one resource by its `owner/name` ID.
    pub async fn get_resource(&self, id: &str) -> Result<Option<Resource>> {
        self.get_object(
            "get-resource",
            &[("owner", &self.config.organization_name), ("id", id)],
        )
        .await
    }

    /// Get one resource by its owner and name.
    pub async fn get_resource_ex(&self, owner: &str, name: &str) -> Result<Option<Resource>> {
        self.get_resource(&format!("{owner}/{name}")).await
    }

    /// Get the resources that match the given filters.
    pub async fn get_resources(
        &self,
        owner: &str,
        user: &str,
        field: &str,
        value: &str,
        sort_field: &str,
        sort_order: &str,
    ) -> Result<Vec<Resource>> {
        self.get_list(
            "get-resources",
            &[
                ("owner", owner),
                ("user", user),
                ("field", field),
                ("value", value),
                ("sortField", sort_field),
                ("sortOrder", sort_order),
            ],
        )
        .await
    }

    /// Get one page of the resources that match the given filters.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_pagination_resources(
        &self,
        owner: &str,
        user: &str,
        field: &str,
        value: &str,
        page_size: i32,
        page: i32,
        sort_field: &str,
        sort_order: &str,
    ) -> Result<Vec<Resource>> {
        self.get_list(
            "get-resources",
            &[
                ("owner", owner),
                ("user", user),
                ("field", field),
                ("value", value),
                ("p", &page.to_string()),
                ("pageSize", &page_size.to_string()),
                ("sortField", sort_field),
                ("sortOrder", sort_order),
            ],
        )
        .await
    }

    /// Upload a file to Casdoor, the returned values are the URL and the name of the
    /// created resource.
    pub async fn upload_resource(
        &self,
        user: &str,
        tag: &str,
        parent: &str,
        full_file_path: &str,
        file_bytes: &[u8],
    ) -> Result<(String, String)> {
        self.do_upload_resource(
            &[
                ("owner", &self.config.organization_name),
                ("user", user),
                ("application", &self.config.application_name),
                ("tag", tag),
                ("parent", parent),
                ("fullFilePath", full_file_path),
            ],
            file_bytes,
        )
        .await
    }

    /// Like [`Client::upload_resource()`], but also sets the created time and the
    /// description of the resource.
    #[allow(clippy::too_many_arguments)]
    pub async fn upload_resource_ex(
        &self,
        user: &str,
        tag: &str,
        parent: &str,
        full_file_path: &str,
        file_bytes: &[u8],
        created_time: &str,
        description: &str,
    ) -> Result<(String, String)> {
        self.do_upload_resource(
            &[
                ("owner", &self.config.organization_name),
                ("user", user),
                ("application", &self.config.application_name),
                ("tag", tag),
                ("parent", parent),
                ("fullFilePath", full_file_path),
                ("createdTime", created_time),
                ("description", description),
            ],
            file_bytes,
        )
        .await
    }

    /// Delete a resource.
    pub async fn delete_resource(&self, resource: &Resource) -> Result<bool> {
        self.delete_resource_with_tag(resource, "").await
    }

    /// Delete a resource that has the given tag.
    pub async fn delete_resource_with_tag(&self, resource: &Resource, tag: &str) -> Result<bool> {
        let mut resource = resource.clone();
        if resource.owner.is_empty() {
            resource.owner = self.config.organization_name.clone();
        }

        let post_bytes = serde_json::to_vec(&resource)?;
        let response = self
            .do_post(
                "delete-resource",
                &[("tag", tag)],
                PostBody::Raw(post_bytes),
            )
            .await?;

        Ok(response.data.as_str() == Some("Affected"))
    }

    async fn do_upload_resource(
        &self,
        query: &[(&str, &str)],
        file_bytes: &[u8],
    ) -> Result<(String, String)> {
        let response = self
            .do_post("upload-resource", query, PostBody::File(file_bytes))
            .await?;

        let file_url = response
            .data
            .as_str()
            .ok_or(CasdoorError::InvalidData)?
            .to_string();
        let name = response
            .data2
            .as_str()
            .ok_or(CasdoorError::InvalidData)?
            .to_string();

        Ok((file_url, name))
    }
}
