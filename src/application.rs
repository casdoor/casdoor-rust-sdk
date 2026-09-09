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

use crate::cert::Cert;
use crate::client::{get_admin_id, get_owner, Client, Response};
use crate::error::Result;
use crate::organization::{Organization, ThemeData};
use crate::provider::Provider;
use crate::serde_util::null_to_default;
use crate::user::ScopeDescription;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProviderItem {
    pub owner: String,
    pub name: String,

    pub can_sign_up: bool,
    pub can_sign_in: bool,
    pub can_unlink: bool,
    pub binding_rule: Option<Vec<String>>,
    #[serde(deserialize_with = "null_to_default")]
    pub country_codes: Vec<String>,
    pub prompted: bool,
    pub signup_group: String,
    pub rule: String,
    pub provider: Option<Provider>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ScopeItem {
    pub name: String,
    pub display_name: String,
    pub description: String,
    /// The MCP tools allowed by this scope.
    #[serde(deserialize_with = "null_to_default")]
    pub tools: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SignupItem {
    pub name: String,
    pub visible: bool,
    pub required: bool,
    pub prompted: bool,
    pub r#type: String,
    pub custom_css: String,
    pub label: String,
    pub placeholder: String,
    #[serde(deserialize_with = "null_to_default")]
    pub options: Vec<String>,
    pub regex: String,
    pub rule: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SigninMethod {
    pub name: String,
    pub display_name: String,
    pub rule: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SigninItem {
    pub name: String,
    pub visible: bool,
    pub label: String,
    pub custom_css: String,
    pub placeholder: String,
    pub rule: String,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SamlItem {
    pub name: String,
    pub name_format: String,
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct JwtItem {
    pub name: String,
    pub category: String,
    pub value: String,
    pub r#type: String,
}

/// Application has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/application.go>.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Application {
    pub owner: String,
    pub name: String,
    pub created_time: String,

    pub display_name: String,
    pub category: String,
    pub r#type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub scopes: Vec<ScopeItem>,
    pub logo: String,
    pub title: String,
    pub favicon: String,
    pub order: i32,
    pub homepage_url: String,
    pub description: String,
    pub organization: String,
    pub cert: String,
    pub default_group: String,
    pub default_tag: String,
    pub header_html: String,
    pub page_html: String,
    pub enable_password: bool,
    pub enable_sign_up: bool,
    pub enable_guest_signin: bool,
    pub disable_signin: bool,
    pub enable_signin_session: bool,
    pub enable_auto_signin: bool,
    pub enable_code_signin: bool,
    pub enable_exclusive_signin: bool,
    pub enable_saml_compress: bool,
    pub enable_saml_c14n10: bool,
    pub enable_saml_post_binding: bool,
    pub disable_saml_attributes: bool,
    pub enable_saml_assertion_signature: bool,
    pub use_email_as_saml_name_id: bool,
    pub enable_web_authn: bool,
    pub enable_link_with_email: bool,
    pub org_choice_mode: String,
    pub saml_reply_url: String,
    #[serde(deserialize_with = "null_to_default")]
    pub providers: Vec<ProviderItem>,
    #[serde(deserialize_with = "null_to_default")]
    pub signin_methods: Vec<SigninMethod>,
    #[serde(deserialize_with = "null_to_default")]
    pub signup_items: Vec<SignupItem>,
    #[serde(deserialize_with = "null_to_default")]
    pub signin_items: Vec<SigninItem>,
    #[serde(deserialize_with = "null_to_default")]
    pub grant_types: Vec<String>,
    pub organization_obj: Option<Organization>,
    pub cert_public_key: String,
    #[serde(deserialize_with = "null_to_default")]
    pub tags: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub saml_attributes: Vec<SamlItem>,
    pub saml_hash_algorithm: String,
    pub saml_c14n_prefix: String,
    pub is_shared: bool,
    pub ip_restriction: String,

    pub client_id: String,
    pub client_secret: String,
    pub client_cert: String,
    #[serde(deserialize_with = "null_to_default")]
    pub redirect_uris: Vec<String>,
    pub backchannel_logout_uri: String,
    pub forced_redirect_origin: String,
    pub token_format: String,
    pub token_signing_method: String,
    #[serde(deserialize_with = "null_to_default")]
    pub token_fields: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub token_attributes: Vec<JwtItem>,
    pub expire_in_hours: f64,
    pub refresh_expire_in_hours: f64,
    pub cookie_expire_in_hours: i64,
    pub signup_url: String,
    pub signin_url: String,
    pub forget_url: String,
    pub affiliation_url: String,
    pub ip_whitelist: String,
    pub terms_of_use: String,
    pub signup_html: String,
    pub signin_html: String,
    pub theme_data: Option<ThemeData>,
    pub footer_html: String,
    pub form_css: String,
    pub form_css_mobile: String,
    pub form_offset: i32,
    pub form_side_html: String,
    pub form_background_url: String,
    pub form_background_url_mobile: String,

    pub failed_signin_limit: i32,
    pub failed_signin_frozen_time: i32,
    pub code_resend_timeout: i32,

    #[serde(deserialize_with = "null_to_default")]
    pub custom_scopes: Vec<ScopeDescription>,

    // Reverse proxy fields.
    pub domain: String,
    #[serde(deserialize_with = "null_to_default")]
    pub other_domains: Vec<String>,
    pub upstream_host: String,
    pub ssl_mode: String,
    pub ssl_cert: String,

    #[serde(rename = "CertObj")]
    pub cert_obj: Option<Cert>,

    pub registration_access_token: String,
}

impl Client {
    /// Get all the applications.
    pub async fn get_applications(&self) -> Result<Vec<Application>> {
        self.get_list("get-applications", &[("owner", "admin")])
            .await
    }

    /// Get the applications of the client's organization.
    pub async fn get_organization_applications(&self) -> Result<Vec<Application>> {
        self.get_list(
            "get-organization-applications",
            &[
                ("owner", "admin"),
                ("organization", &self.config.organization_name),
            ],
        )
        .await
    }

    /// Get one application by name.
    pub async fn get_application(&self, name: &str) -> Result<Option<Application>> {
        self.get_object("get-application", &[("id", &get_admin_id(name))])
            .await
    }

    /// Add an application.
    pub async fn add_application(&self, application: &Application) -> Result<bool> {
        Ok(self
            .modify_application("add-application", application, &[])
            .await?
            .1)
    }

    /// Update an application.
    pub async fn update_application(&self, application: &Application) -> Result<bool> {
        Ok(self
            .modify_application("update-application", application, &[])
            .await?
            .1)
    }

    /// Delete an application.
    pub async fn delete_application(&self, application: &Application) -> Result<bool> {
        Ok(self
            .modify_application("delete-application", application, &[])
            .await?
            .1)
    }

    async fn modify_application(
        &self,
        action: &str,
        application: &Application,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut application = application.clone();
        application.owner = get_owner(&application.owner, "admin");

        let id = format!("{}/{}", application.owner, application.name);
        self.modify_object(action, &id, &application, columns).await
    }
}
