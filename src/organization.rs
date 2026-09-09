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

use crate::client::{get_admin_id, get_owner, Client, Response};
use crate::error::Result;
use crate::serde_util::null_to_default;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AccountItem {
    pub name: String,
    pub visible: bool,
    pub view_rule: String,
    pub modify_rule: String,
    pub regex: String,
    pub tab: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ThemeData {
    pub theme_type: String,
    pub color_primary: String,
    pub border_radius: i32,
    pub is_compact: bool,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MfaItem {
    pub name: String,
    pub rule: String,
}

/// Organization has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/organization.go>.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Organization {
    pub owner: String,
    pub name: String,
    pub created_time: String,

    pub display_name: String,
    pub website_url: String,
    pub logo: String,
    pub logo_dark: String,
    pub favicon: String,
    pub has_privilege_consent: bool,
    pub password_type: String,
    pub password_salt: String,
    #[serde(deserialize_with = "null_to_default")]
    pub password_options: Vec<String>,
    pub password_obfuscator_type: String,
    pub password_obfuscator_key: String,
    pub password_expire_days: i32,
    pub token_retention_days: i32,
    pub record_retention_days: i32,
    #[serde(deserialize_with = "null_to_default")]
    pub country_codes: Vec<String>,
    pub default_avatar: String,
    pub use_permanent_avatar: bool,
    pub default_application: String,
    pub default_token_format: String,
    #[serde(deserialize_with = "null_to_default")]
    pub user_types: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub tags: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub languages: Vec<String>,
    pub theme_data: Option<ThemeData>,
    pub master_password: String,
    pub default_password: String,
    pub master_verification_code: String,
    pub ip_whitelist: String,
    pub init_score: i32,
    pub enable_soft_deletion: bool,
    pub is_profile_public: bool,
    pub use_email_as_username: bool,
    pub enable_tour: bool,
    pub disable_signin: bool,
    pub disable_console: bool,
    pub ip_restriction: String,
    #[serde(deserialize_with = "null_to_default")]
    pub nav_items: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub user_nav_items: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub widget_items: Vec<String>,

    #[serde(deserialize_with = "null_to_default")]
    pub mfa_items: Vec<MfaItem>,
    pub mfa_remember_in_hours: i32,
    pub account_menu: String,
    #[serde(deserialize_with = "null_to_default")]
    pub account_items: Vec<AccountItem>,

    pub dcr_policy: String,

    #[serde(deserialize_with = "null_to_default")]
    pub ldap_attributes: Vec<String>,

    pub kerberos_realm: String,
    pub kerberos_kdc_host: String,
    pub kerberos_keytab: String,
    pub kerberos_service_name: String,

    pub org_balance: f64,
    pub user_balance: f64,
    pub balance_credit: f64,
    pub balance_currency: String,
}

impl Client {
    /// Get one organization by name.
    pub async fn get_organization(&self, name: &str) -> Result<Option<Organization>> {
        self.get_object("get-organization", &[("id", &get_admin_id(name))])
            .await
    }

    /// Get all the organizations.
    pub async fn get_organizations(&self) -> Result<Vec<Organization>> {
        self.get_list(
            "get-organizations",
            &[("owner", &self.config.organization_name)],
        )
        .await
    }

    /// Get all the organizations, with only their names filled in.
    pub async fn get_organization_names(&self) -> Result<Vec<Organization>> {
        self.get_list(
            "get-organization-names",
            &[("owner", &self.config.organization_name)],
        )
        .await
    }

    /// Add an organization.
    pub async fn add_organization(&self, organization: &Organization) -> Result<bool> {
        Ok(self
            .modify_organization("add-organization", organization, &[])
            .await?
            .1)
    }

    /// Update an organization.
    pub async fn update_organization(&self, organization: &Organization) -> Result<bool> {
        Ok(self
            .modify_organization("update-organization", organization, &[])
            .await?
            .1)
    }

    /// Delete an organization.
    pub async fn delete_organization(&self, organization: &Organization) -> Result<bool> {
        Ok(self
            .modify_organization("delete-organization", organization, &[])
            .await?
            .1)
    }

    async fn modify_organization(
        &self,
        action: &str,
        organization: &Organization,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut organization = organization.clone();
        organization.owner = get_owner(&organization.owner, "admin");

        let id = format!("{}/{}", organization.owner, organization.name);
        self.modify_object(action, &id, &organization, columns)
            .await
    }
}
