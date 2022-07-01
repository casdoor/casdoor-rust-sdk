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

///the User struct, the same as that from Go SDK
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    //is all the pub needed here? will there be geeker way to do this?
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,

    pub id: String,
    pub r#type: String,
    pub password: String,
    pub password_salt: String,
    pub display_name: String,
    pub avatar: String,
    pub permanent_avatar: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub address: Vec<String>,
    pub affiliation: String,
    pub title: String,
    pub id_card_type: String,
    pub id_card: String,
    pub homepage: String,
    pub bio: String,
    pub tag: String,
    pub region: String,
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
    pub is_global_admin: bool,
    pub is_forbidden: bool,
    pub is_deleted: bool,
    pub signup_application: String,
    pub hash: String,
    pub pre_hash: String,

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

    pub ldap: String,
    pub properties: HashMap<String, String>,
}

impl User {
    ///return a User, specify important fields, leaving the others as default
    pub fn new() -> Self {
        let mut tmp = User::default();
        //specify some important fields manually
        tmp.is_admin = false;
        tmp.is_deleted = false;
        tmp.is_global_admin = false;
        tmp.is_default_avatar = true;
        tmp.is_online = false;
        tmp.is_forbidden = false;
        tmp
    }
}

///there are two methods to create an user:
///
/// 1. based on an existing User
///
/// ```
///    #[macro_use]
///    extern crate casdoor;
///    use casdoor::user::*;
///    fn main(){
///        let user1 = User::default();
///        let user2 = user!(user1, phone="12345678901", id_card="123456");
///    }
///
/// ```
/// 2. specify the fileds needed, leave others default
///
/// ```
///    #[macro_use]
///    extern crate casdoor;
///    use casdoor::user::*;
///    fn main(){
///        let user3 = user!(phone="12345678901", id_card="123456");
///    }
/// ```
#[macro_export]
macro_rules! user {
    ($ori_user: ident, $($p:ident = $e:expr), *) => {
        $crate::user::User {
            $(
                $p: $e.to_string(),
            )*
            ..$ori_user.clone()
        }
    };
    ($($p:ident = $e:expr), *) => {
        $crate::user::User {
            $(
                $p: $e.to_string(),
            )*
            .. $crate::user::User::new()
        }
    }
}
