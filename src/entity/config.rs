// Copyright 2022 The Casdoor Authors. All Rights Reserved.
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

use serde_derive::Deserialize;
use std::{fs::File, io::Read};

/// CasdoorConfig is the core configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct CasdoorConfig {
    pub(crate) endpoint: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) jwt_pub_key: String,
    pub(crate) org_name: String,
    pub(crate) app_name: Option<String>,
}

impl CasdoorConfig {
    /// Create a new CasdoorConfig.
    #[allow(dead_code)]
    pub fn new(
        endpoint: String,
        client_id: String,
        client_secret: String,
        jwt_pub_key: String,
        org_name: String,
        app_name: Option<String>,
    ) -> Self {
        CasdoorConfig {
            endpoint,
            client_id,
            client_secret,
            jwt_pub_key,
            org_name,
            app_name,
        }
    }

    /// Create a new CasdoorConfig from a Toml file.
    #[allow(dead_code)]
    pub fn from_toml(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // read path file content
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(toml::from_str(&content)?)
    }
}
