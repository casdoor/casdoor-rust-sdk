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

/// Cert has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/cert.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Cert {
    pub owner: String,
    pub name: String,
    pub created_time: String,

    pub display_name: String,
    pub scope: String,
    pub r#type: String,
    pub crypto_algorithm: String,
    pub bit_size: i32,
    pub expire_in_years: i32,

    pub expire_time: String,
    pub domain_expire_time: String,
    pub provider: String,
    pub account: String,
    pub access_key: String,
    pub access_secret: String,

    pub certificate: String,
    pub private_key: String,
}

impl Client {
    /// Get the certs shared by all the organizations.
    pub async fn get_global_certs(&self) -> Result<Vec<Cert>> {
        self.get_list("get-global-certs", &[]).await
    }

    /// Get the certs of the client's organization.
    pub async fn get_certs(&self) -> Result<Vec<Cert>> {
        self.get_list("get-certs", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get one cert by name.
    pub async fn get_cert(&self, name: &str) -> Result<Option<Cert>> {
        self.get_object("get-cert", &[("id", &self.get_id(name))])
            .await
    }

    /// Add a cert.
    pub async fn add_cert(&self, cert: &Cert) -> Result<bool> {
        Ok(self.modify_cert("add-cert", cert, &[]).await?.1)
    }

    /// Update a cert.
    pub async fn update_cert(&self, cert: &Cert) -> Result<bool> {
        Ok(self.modify_cert("update-cert", cert, &[]).await?.1)
    }

    /// Delete a cert.
    pub async fn delete_cert(&self, cert: &Cert) -> Result<bool> {
        Ok(self.modify_cert("delete-cert", cert, &[]).await?.1)
    }

    async fn modify_cert(
        &self,
        action: &str,
        cert: &Cert,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut cert = cert.clone();
        cert.owner = get_owner(&cert.owner, &self.config.organization_name);

        let id = format!("{}/{}", cert.owner, cert.name);
        self.modify_object(action, &id, &cert, columns).await
    }
}
