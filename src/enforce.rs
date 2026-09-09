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
use serde_json::Value;

use crate::client::{Client, PostBody, Response};
use crate::error::{CasdoorError, Result};

/// PermissionRule has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/permission_rule.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PermissionRule {
    pub ptype: String,
    pub v0: String,
    pub v1: String,
    pub v2: String,
    pub v3: String,
    pub v4: String,
    pub v5: String,
    pub id: String,
}

/// A Casbin request, such as `["alice", "data1", "read"]`.
pub type CasbinRequest = Vec<Value>;

impl Client {
    /// Check whether a Casbin request is allowed by a permission, a model, a resource or
    /// an enforcer. Only one of `permission_id`, `model_id`, `resource_id` and
    /// `enforcer_id` needs to be given, the others can be empty.
    pub async fn enforce(
        &self,
        permission_id: &str,
        model_id: &str,
        resource_id: &str,
        enforcer_id: &str,
        owner: &str,
        casbin_request: &CasbinRequest,
    ) -> Result<bool> {
        let post_bytes = serde_json::to_vec(casbin_request)?;

        let response = self
            .do_enforce(
                "enforce",
                permission_id,
                model_id,
                resource_id,
                enforcer_id,
                owner,
                post_bytes,
            )
            .await?;

        let results = response.data.as_array().ok_or(CasdoorError::InvalidData)?;

        for result in results {
            let is_allowed = result.as_bool().ok_or(CasdoorError::InvalidData)?;
            if is_allowed {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Check a batch of Casbin requests at once, the returned matrix has one row per
    /// permission and one column per request.
    pub async fn batch_enforce(
        &self,
        permission_id: &str,
        model_id: &str,
        resource_id: &str,
        enforcer_id: &str,
        owner: &str,
        casbin_requests: &[CasbinRequest],
    ) -> Result<Vec<Vec<bool>>> {
        let post_bytes = serde_json::to_vec(casbin_requests)?;

        let response = self
            .do_enforce(
                "batch-enforce",
                permission_id,
                model_id,
                resource_id,
                enforcer_id,
                owner,
                post_bytes,
            )
            .await?;

        serde_json::from_value(response.data).map_err(|_| CasdoorError::InvalidData)
    }

    #[allow(clippy::too_many_arguments)]
    async fn do_enforce(
        &self,
        action: &str,
        permission_id: &str,
        model_id: &str,
        resource_id: &str,
        enforcer_id: &str,
        owner: &str,
        post_bytes: Vec<u8>,
    ) -> Result<Response> {
        self.do_post(
            action,
            &[
                ("permissionId", permission_id),
                ("modelId", model_id),
                ("resourceId", resource_id),
                ("enforcerId", enforcer_id),
                ("owner", owner),
            ],
            PostBody::Raw(post_bytes),
        )
        .await
    }
}
