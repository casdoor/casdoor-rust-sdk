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

/// Model has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/model.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Model {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub display_name: String,
    pub description: String,

    pub model_text: String,
}

impl Client {
    /// Get the models of the client's organization.
    pub async fn get_models(&self) -> Result<Vec<Model>> {
        self.get_list("get-models", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one page of the models of the client's organization, together with the total
    /// number of the models.
    pub async fn get_pagination_models(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<Model>, i32)> {
        self.get_pagination(
            "get-models",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get one model by name.
    pub async fn get_model(&self, name: &str) -> Result<Option<Model>> {
        self.get_object("get-model", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a model.
    pub async fn add_model(&self, model: &Model) -> Result<bool> {
        Ok(self.modify_model("add-model", model, &[]).await?.1)
    }

    /// Update a model.
    pub async fn update_model(&self, model: &Model) -> Result<bool> {
        Ok(self.modify_model("update-model", model, &[]).await?.1)
    }

    /// Delete a model.
    pub async fn delete_model(&self, model: &Model) -> Result<bool> {
        Ok(self.modify_model("delete-model", model, &[]).await?.1)
    }

    async fn modify_model(
        &self,
        action: &str,
        model: &Model,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut model = model.clone();
        model.owner = get_owner(&model.owner, &self.config.organization_name);

        let id = format!("{}/{}", model.owner, model.name);
        self.modify_object(action, &id, &model, columns).await
    }
}
