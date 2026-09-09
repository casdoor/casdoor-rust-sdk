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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{get_admin_id, get_owner, Client, PostBody, Response};
use crate::error::Result;
use crate::serde_util::null_to_default;

/// Ldap has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/ldap.go>.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Ldap {
    pub id: String,
    pub owner: String,
    pub created_time: String,

    pub server_name: String,
    pub host: String,
    pub port: i32,
    pub enable_ssl: bool,
    pub allow_self_signed_cert: bool,
    pub username: String,
    pub password: String,
    pub base_dn: String,
    pub filter: String,
    #[serde(deserialize_with = "null_to_default")]
    pub filter_fields: Vec<String>,
    pub default_group: String,
    #[serde(deserialize_with = "null_to_default")]
    pub default_groups: Vec<String>,
    pub password_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub custom_attributes: HashMap<String, String>,

    pub auto_sync: i32,
    pub last_sync: String,
    pub enable_groups: bool,
    pub enable_password_reset: bool,
}

/// One user read from an LDAP server.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LdapUser {
    pub uid_number: String,
    pub uid: String,
    pub cn: String,
    pub gid_number: String,
    pub uuid: String,
    pub user_principal_name: String,
    pub display_name: String,
    #[serde(rename = "Mail")]
    pub mail: String,
    pub email: String,
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    #[serde(rename = "TelephoneNumber")]
    pub telephone_number: String,
    pub mobile: String,
    #[serde(rename = "MobileTelephoneNumber")]
    pub mobile_telephone_number: String,
    #[serde(rename = "RegisteredAddress")]
    pub registered_address: String,
    #[serde(rename = "PostalAddress")]
    pub postal_address: String,
    pub country: String,
    pub country_name: String,

    pub group_id: String,
    pub address: String,
    #[serde(deserialize_with = "null_to_default")]
    pub member_of: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub attributes: HashMap<String, String>,
}

/// The response of [`Client::get_ldap_users()`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LdapUsersResponse {
    #[serde(deserialize_with = "null_to_default")]
    pub exist_uuids: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub users: Vec<LdapUser>,
}

/// The response of [`Client::sync_ldap_users()`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SyncLdapUsersResponse {
    #[serde(deserialize_with = "null_to_default")]
    pub exist: Vec<LdapUser>,
    #[serde(deserialize_with = "null_to_default")]
    pub failed: Vec<LdapUser>,
}

impl Client {
    /// Get all the LDAP servers.
    pub async fn get_ldaps(&self) -> Result<Vec<Ldap>> {
        self.get_list("get-ldaps", &[("owner", "admin")]).await
    }

    /// Get one LDAP server by ID.
    pub async fn get_ldap(&self, id: &str) -> Result<Option<Ldap>> {
        self.get_object("get-ldap", &[("id", &get_admin_id(id))])
            .await
    }

    /// Add an LDAP server.
    pub async fn add_ldap(&self, ldap: &Ldap) -> Result<bool> {
        Ok(self.modify_ldap("add-ldap", ldap, &[]).await?.1)
    }

    /// Update an LDAP server.
    pub async fn update_ldap(&self, ldap: &Ldap) -> Result<bool> {
        Ok(self.modify_ldap("update-ldap", ldap, &[]).await?.1)
    }

    /// Delete an LDAP server.
    pub async fn delete_ldap(&self, ldap: &Ldap) -> Result<bool> {
        Ok(self.modify_ldap("delete-ldap", ldap, &[]).await?.1)
    }

    /// Get the users of an LDAP server, together with the UUIDs that already exist in
    /// Casdoor.
    pub async fn get_ldap_users(&self, id: &str) -> Result<Option<LdapUsersResponse>> {
        self.get_object("get-ldap-users", &[("id", &self.get_id(id))])
            .await
    }

    /// Sync the given LDAP users into Casdoor.
    pub async fn sync_ldap_users(
        &self,
        id: &str,
        users: &[LdapUser],
    ) -> Result<Option<SyncLdapUsersResponse>> {
        let post_bytes = serde_json::to_vec(users)?;

        let response = self
            .do_post(
                "sync-ldap-users",
                &[("id", &self.get_id(id))],
                PostBody::Raw(post_bytes),
            )
            .await?;

        if response.data.is_null() {
            return Ok(None);
        }

        Ok(Some(serde_json::from_value(response.data)?))
    }

    /// Read the users of an LDAP server and sync them all into Casdoor.
    pub async fn sync_ldap_users_from_server(
        &self,
        id: &str,
    ) -> Result<Option<SyncLdapUsersResponse>> {
        let users = self
            .get_ldap_users(id)
            .await?
            .map(|response| response.users)
            .unwrap_or_default();

        self.sync_ldap_users(id, &users).await
    }

    async fn modify_ldap(
        &self,
        action: &str,
        ldap: &Ldap,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut ldap = ldap.clone();
        ldap.owner = get_owner(&ldap.owner, "admin");

        let id = format!("{}/{}", ldap.owner, ldap.id);
        self.modify_object(action, &id, &ldap, columns).await
    }
}
