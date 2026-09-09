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

use std::fs;

use serde::{Deserialize, Serialize};

use crate::error::{CasdoorError, Result};

/// AuthConfig is the core configuration of the SDK, it has the same fields as the
/// `AuthConfig` struct of the Casdoor Go SDK.
///
/// The `organization_name` and `application_name` fields can also be written as
/// `org_name` and `app_name` in a TOML file, so that the config files of the 1.x
/// versions of this SDK keep working.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Casdoor server URL, such as `http://localhost:8000`.
    pub endpoint: String,
    /// Client ID of the Casdoor application.
    pub client_id: String,
    /// Client secret of the Casdoor application.
    pub client_secret: String,
    /// The x509 certificate content of the Casdoor application, it's only used by
    /// [`crate::Client::parse_jwt_token()`].
    #[serde(default)]
    pub certificate: String,
    /// The name of the Casdoor organization.
    #[serde(alias = "org_name")]
    pub organization_name: String,
    /// The name of the Casdoor application.
    #[serde(default, alias = "app_name")]
    pub application_name: String,
}

impl AuthConfig {
    /// Create a new AuthConfig.
    pub fn new(
        endpoint: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        certificate: impl Into<String>,
        organization_name: impl Into<String>,
        application_name: impl Into<String>,
    ) -> Self {
        Self {
            endpoint: endpoint.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            certificate: certificate.into(),
            organization_name: organization_name.into(),
            application_name: application_name.into(),
        }
    }

    /// Create a new AuthConfig from a TOML file, the path should be an absolute path.
    ///
    /// ```toml
    /// endpoint = "http://localhost:8000"
    /// client_id = "0e5b3fdbc2ba7b04a000"
    /// client_secret = "1b5b3fdbc2ba7b04a0000f5b3fdbc2ba7b04a000"
    /// certificate = """
    /// -----BEGIN CERTIFICATE-----
    /// ...
    /// -----END CERTIFICATE-----
    /// """
    /// organization_name = "built-in"
    /// application_name = "app-built-in"
    /// ```
    pub fn from_toml(path: &str) -> Result<Self> {
        let content =
            fs::read_to_string(path).map_err(|e| CasdoorError::Config(format!("{path}: {e}")))?;

        toml::from_str(&content).map_err(|e| CasdoorError::Config(format!("{path}: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn write_config(name: &str, content: &str) -> String {
        let path = std::env::temp_dir().join(name);
        fs::write(&path, content).unwrap();

        path.to_str().unwrap().to_string()
    }

    #[test]
    fn from_toml_reads_the_config_file() {
        let path = write_config(
            "casdoor-rust-sdk-config.toml",
            r#"
            endpoint = "http://localhost:8000"
            client_id = "client-id"
            client_secret = "client-secret"
            certificate = "certificate"
            organization_name = "built-in"
            application_name = "app-built-in"
            "#,
        );

        let config = AuthConfig::from_toml(&path).unwrap();

        assert_eq!(config.endpoint, "http://localhost:8000");
        assert_eq!(config.organization_name, "built-in");
        assert_eq!(config.application_name, "app-built-in");
    }

    #[test]
    fn from_toml_still_reads_the_config_file_of_the_1_x_versions() {
        let path = write_config(
            "casdoor-rust-sdk-legacy-config.toml",
            r#"
            endpoint = "http://localhost:8000"
            client_id = "client-id"
            client_secret = "client-secret"
            certificate = "certificate"
            org_name = "built-in"
            app_name = "app-built-in"
            "#,
        );

        let config = AuthConfig::from_toml(&path).unwrap();

        assert_eq!(config.organization_name, "built-in");
        assert_eq!(config.application_name, "app-built-in");
    }

    #[test]
    fn from_toml_reports_the_path_of_a_missing_config_file() {
        let error = AuthConfig::from_toml("no-such-conf.toml").unwrap_err();
        assert!(error.to_string().contains("no-such-conf.toml"));
    }
}
