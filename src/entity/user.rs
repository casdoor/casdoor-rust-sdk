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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// User info struct, defined in the SDK.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CasdoorUser {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,

    pub id: String,
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
    pub address: Vec<String>,
    pub affiliation: String,
    pub title: String,
    pub id_card_type: String,
    pub id_card: String,
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
    pub is_default_avatar: bool,
    pub is_online: bool,
    pub is_admin: bool,
    pub is_forbidden: bool,
    pub is_deleted: bool,
    pub signup_application: String,
    pub hash: String,
    pub pre_hash: String,

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
    pub ldap: String,

    pub properties: HashMap<String, String>,
    pub groups: Vec<String>,
    pub last_signin_wrong_time: String,
    pub signin_wrong_times: i32,
}
