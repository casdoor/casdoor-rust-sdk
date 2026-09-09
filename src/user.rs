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
use serde_json::Value;

use crate::client::{get_owner, Client, PostBody, Response};
use crate::error::Result;
use crate::order::ProductInfo;
use crate::organization::MfaItem;
use crate::permission::Permission;
use crate::role::Role;
use crate::serde_util::null_to_default;

/// The name of the session that holds the MFA recovery codes.
pub const MFA_RECOVERY_CODES_SESSION: &str = "mfa_recovery_codes";

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ManagedAccount {
    pub application: String,
    pub username: String,
    pub password: String,
    pub signin_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MfaAccount {
    pub account_name: String,
    pub issuer: String,
    pub secret_key: String,
    pub origin: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Address {
    pub tag: String,
    pub line1: String,
    pub line2: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub region: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FaceId {
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub face_id_data: Vec<f64>,
    pub image_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MfaProps {
    pub enabled: bool,
    pub is_preferred: bool,
    pub mfa_type: String,
    pub secret: String,
    pub country_code: String,
    pub url: String,
    #[serde(deserialize_with = "null_to_default")]
    pub recovery_codes: Vec<String>,
    pub mfa_remember_in_hours: i32,
}

/// The user info returned by the OIDC `/api/userinfo` API.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Userinfo {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    #[serde(rename = "preferred_username")]
    pub name: String,
    #[serde(rename = "name")]
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    #[serde(rename = "picture")]
    pub avatar: String,
    pub address: String,
    pub phone: String,
    pub real_name: String,
    pub is_verified: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub groups: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub roles: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ConsentRecord {
    /// The `owner/name` ID of the application.
    pub application: String,
    #[serde(deserialize_with = "null_to_default")]
    pub granted_scopes: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ThirdPartyLink {
    pub owner: String,
    pub user_name: String,
    pub provider_name: String,
    pub provider_id: String,
    pub created_time: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ScopeDescription {
    pub scope: String,
    pub display_name: String,
    pub description: String,
}

/// User has the same definition as
/// <https://github.com/casdoor/casdoor/blob/master/object/user.go>.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct User {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub deleted_time: String,

    pub id: String,
    pub external_id: String,
    pub r#type: String,
    pub password: String,
    pub password_salt: String,
    pub password_type: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar: String,
    pub avatar_type: String,
    pub permanent_avatar: String,
    pub email: String,
    pub email_verified: bool,
    pub phone: String,
    pub country_code: String,
    pub region: String,
    pub location: String,
    #[serde(deserialize_with = "null_to_default")]
    pub address: Vec<String>,
    #[serde(deserialize_with = "null_to_default")]
    pub addresses: Vec<Address>,
    pub affiliation: String,
    pub title: String,
    pub id_card_type: String,
    pub id_card: String,
    pub real_name: String,
    pub is_verified: bool,
    pub homepage: String,
    pub bio: String,
    pub tag: String,
    pub language: String,
    pub gender: String,
    pub birthday: String,
    pub education: String,
    pub score: i32,
    pub karma: i32,
    pub ranking: i32,
    pub balance: f64,
    pub balance_credit: f64,
    pub currency: String,
    pub balance_currency: String,
    pub is_default_avatar: bool,
    pub is_online: bool,
    pub is_admin: bool,
    pub is_forbidden: bool,
    pub is_deleted: bool,
    pub signup_application: String,
    pub hash: String,
    pub pre_hash: String,
    pub register_type: String,
    pub register_source: String,
    pub access_token: String,
    pub original_token: String,
    pub original_refresh_token: String,

    pub created_ip: String,
    pub last_signin_time: String,
    pub last_signin_ip: String,

    pub github: String,
    pub google: String,
    pub qq: String,
    pub wechat: String,
    pub facebook: String,
    pub dingtalk: String,
    pub weibo: String,
    pub gitee: String,
    pub linkedin: String,
    pub wecom: String,
    pub lark: String,
    pub gitlab: String,
    pub adfs: String,
    pub baidu: String,
    pub alipay: String,
    pub casdoor: String,
    pub infoflow: String,
    pub apple: String,
    pub azuread: String,
    pub azureadb2c: String,
    pub slack: String,
    pub steam: String,
    pub bilibili: String,
    pub okta: String,
    pub douyin: String,
    pub kwai: String,
    pub line: String,
    pub amazon: String,
    pub auth0: String,
    pub battlenet: String,
    pub bitbucket: String,
    #[serde(rename = "box")]
    pub r#box: String,
    pub cloudfoundry: String,
    pub dailymotion: String,
    pub deezer: String,
    pub digitalocean: String,
    pub discord: String,
    pub dropbox: String,
    pub eveonline: String,
    pub fitbit: String,
    pub gitea: String,
    pub heroku: String,
    pub influxcloud: String,
    pub instagram: String,
    pub intercom: String,
    pub kakao: String,
    pub lastfm: String,
    pub mailru: String,
    pub meetup: String,
    pub microsoftonline: String,
    pub naver: String,
    pub nextcloud: String,
    pub onedrive: String,
    pub oura: String,
    pub patreon: String,
    pub paypal: String,
    pub salesforce: String,
    pub shopify: String,
    pub soundcloud: String,
    pub spotify: String,
    pub strava: String,
    pub stripe: String,
    pub telegram: String,
    pub tiktok: String,
    pub tumblr: String,
    pub twitch: String,
    pub twitter: String,
    pub typetalk: String,
    pub uber: String,
    pub vk: String,
    pub wepay: String,
    pub xero: String,
    pub yahoo: String,
    pub yammer: String,
    pub yandex: String,
    pub zoom: String,
    pub metamask: String,
    pub web3onboard: String,
    pub oidc: String,
    pub custom: String,
    pub custom2: String,
    pub custom3: String,
    pub custom4: String,
    pub custom5: String,
    pub custom6: String,
    pub custom7: String,
    pub custom8: String,
    pub custom9: String,
    pub custom10: String,

    /// The WebAuthn credentials of the user, kept as raw JSON so that the value is
    /// round-tripped untouched on `get_user()` and `update_user()`.
    pub webauthn_credentials: Value,
    pub preferred_mfa_type: String,
    #[serde(deserialize_with = "null_to_default")]
    pub recovery_codes: Vec<String>,
    pub totp_secret: String,
    pub mfa_phone_enabled: bool,
    pub mfa_email_enabled: bool,
    pub mfa_radius_enabled: bool,
    pub mfa_radius_username: String,
    pub mfa_radius_provider: String,
    pub mfa_push_enabled: bool,
    pub mfa_push_receiver: String,
    pub mfa_push_provider: String,
    #[serde(deserialize_with = "null_to_default")]
    pub multi_factor_auths: Vec<MfaProps>,
    pub invitation: String,
    pub invitation_code: String,
    #[serde(deserialize_with = "null_to_default")]
    pub face_ids: Vec<FaceId>,
    #[serde(deserialize_with = "null_to_default")]
    pub cart: Vec<ProductInfo>,

    pub ldap: String,
    pub uid_number: i32,
    #[serde(deserialize_with = "null_to_default")]
    pub properties: HashMap<String, String>,

    #[serde(deserialize_with = "null_to_default")]
    pub third_party_links: Vec<ThirdPartyLink>,

    #[serde(deserialize_with = "null_to_default")]
    pub roles: Vec<Role>,
    #[serde(deserialize_with = "null_to_default")]
    pub permissions: Vec<Permission>,
    #[serde(deserialize_with = "null_to_default")]
    pub groups: Vec<String>,

    pub last_change_password_time: String,
    pub last_signin_wrong_time: String,
    pub signin_wrong_times: i32,

    #[serde(deserialize_with = "null_to_default")]
    pub managed_accounts: Vec<ManagedAccount>,
    #[serde(deserialize_with = "null_to_default")]
    pub mfa_accounts: Vec<MfaAccount>,
    #[serde(deserialize_with = "null_to_default")]
    pub mfa_items: Vec<MfaItem>,
    pub mfa_remember_deadline: String,
    pub need_update_password: bool,
    pub ip_whitelist: String,
    #[serde(deserialize_with = "null_to_default")]
    pub application_scopes: Vec<ConsentRecord>,
}

impl User {
    /// Return the Casdoor object ID (`"owner/name"`) of the user.
    pub fn get_id(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

impl Client {
    /// Get the users of all the organizations.
    pub async fn get_global_users(&self) -> Result<Vec<User>> {
        self.get_list("get-global-users", &[]).await
    }

    /// Get the users of the client's organization.
    pub async fn get_users(&self) -> Result<Vec<User>> {
        self.get_list("get-users", &[("owner", &self.config.organization_name)])
            .await
    }

    /// Get the users of the client's organization, sorted by `sorter` and limited to
    /// `limit` results.
    pub async fn get_sorted_users(&self, sorter: &str, limit: i32) -> Result<Vec<User>> {
        self.get_list(
            "get-sorted-users",
            &[
                ("owner", &self.config.organization_name),
                ("sorter", sorter),
                ("limit", &limit.to_string()),
            ],
        )
        .await
    }

    /// Get one page of the users of the client's organization, together with the total
    /// number of the users.
    pub async fn get_pagination_users(
        &self,
        p: i32,
        page_size: i32,
        query: &[(&str, &str)],
    ) -> Result<(Vec<User>, i32)> {
        self.get_pagination(
            "get-users",
            &self.config.organization_name,
            p,
            page_size,
            query,
        )
        .await
    }

    /// Get the number of the users of the client's organization. `is_online` can be `""`
    /// for all the users, `"1"` for the online ones and `"0"` for the offline ones.
    pub async fn get_user_count(&self, is_online: &str) -> Result<i32> {
        let url = self.get_url(
            "get-user-count",
            &[
                ("owner", &self.config.organization_name),
                ("isOnline", is_online),
            ],
        );

        let response = self.do_get_response(&url).await?;
        Ok(serde_json::from_value(response.data)?)
    }

    /// Get one user by name, or by the `owner/name` ID for a user of another
    /// organization.
    pub async fn get_user(&self, name: &str) -> Result<Option<User>> {
        self.get_object("get-user", &[("id", &self.get_id(name))])
            .await
    }

    /// Get the user that the client is authenticated as, by calling the
    /// `/api/get-account` API. It's meant to be used with a client returned by
    /// [`Client::with_access_token()`], so that the owner of an access token can be
    /// retrieved.
    pub async fn get_account(&self) -> Result<Option<User>> {
        self.get_object("get-account", &[]).await
    }

    /// Get one user of the client's organization by email.
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        self.get_object(
            "get-user",
            &[("owner", &self.config.organization_name), ("email", email)],
        )
        .await
    }

    /// Get one user of the client's organization by phone.
    pub async fn get_user_by_phone(&self, phone: &str) -> Result<Option<User>> {
        self.get_object(
            "get-user",
            &[("owner", &self.config.organization_name), ("phone", phone)],
        )
        .await
    }

    /// Get one user of the client's organization by the user's `id` field.
    pub async fn get_user_by_user_id(&self, user_id: &str) -> Result<Option<User>> {
        self.get_object(
            "get-user",
            &[
                ("owner", &self.config.organization_name),
                ("userId", user_id),
            ],
        )
        .await
    }

    /// Set the password of a user. `old_password` is not required, just pass an empty
    /// string when you don't need it.
    pub async fn set_password(
        &self,
        owner: &str,
        name: &str,
        old_password: &str,
        new_password: &str,
    ) -> Result<bool> {
        let fields = HashMap::from([
            ("userOwner".to_string(), owner.to_string()),
            ("userName".to_string(), name.to_string()),
            ("oldPassword".to_string(), old_password.to_string()),
            ("newPassword".to_string(), new_password.to_string()),
        ]);

        let response = self
            .do_post("set-password", &[], PostBody::Form(fields))
            .await?;

        Ok(response.status == "ok")
    }

    /// Add a user.
    pub async fn add_user(&self, user: &User) -> Result<bool> {
        Ok(self.modify_user("add-user", user, &[]).await?.1)
    }

    /// Update a user.
    pub async fn update_user(&self, user: &User) -> Result<bool> {
        Ok(self.modify_user("update-user", user, &[]).await?.1)
    }

    /// Update only the given columns of a user.
    pub async fn update_user_for_columns(&self, user: &User, columns: &[&str]) -> Result<bool> {
        Ok(self.modify_user("update-user", user, columns).await?.1)
    }

    /// Update the user addressed by the given `owner/name` ID.
    pub async fn update_user_by_id(&self, id: &str, user: &User) -> Result<bool> {
        Ok(self
            .modify_user_by_id("update-user", id, user, &[])
            .await?
            .1)
    }

    /// Update the user of `owner` whose `id` field is `user_id`.
    pub async fn update_user_by_user_id(
        &self,
        owner: &str,
        user_id: &str,
        user: &User,
    ) -> Result<bool> {
        let post_bytes = serde_json::to_vec(user)?;
        let response = self
            .do_post(
                "update-user",
                &[("owner", owner), ("userId", user_id)],
                PostBody::Raw(post_bytes),
            )
            .await?;

        Ok(response.data.as_str() == Some("Affected"))
    }

    /// Delete a user.
    pub async fn delete_user(&self, user: &User) -> Result<bool> {
        Ok(self.modify_user("delete-user", user, &[]).await?.1)
    }

    /// Check whether the `password` field of the given user is the user's password.
    pub async fn check_user_password(&self, user: &User) -> Result<bool> {
        let (response, _) = self.modify_user("check-user-password", user, &[]).await?;
        Ok(response.status == "ok")
    }

    /// The encapsulation of the user CUD (Create, Update, Delete) operations, the possible
    /// actions are `add-user`, `update-user`, `delete-user` and `check-user-password`.
    async fn modify_user(
        &self,
        action: &str,
        user: &User,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let owner = get_owner(&user.owner, &self.config.organization_name);
        let id = format!("{}/{}", owner, user.name);

        self.modify_user_by_id(action, &id, user, columns).await
    }

    async fn modify_user_by_id(
        &self,
        action: &str,
        id: &str,
        user: &User,
        columns: &[&str],
    ) -> Result<(Response, bool)> {
        let mut user = user.clone();
        user.owner = get_owner(&user.owner, &self.config.organization_name);

        self.modify_object(action, id, &user, columns).await
    }
}
