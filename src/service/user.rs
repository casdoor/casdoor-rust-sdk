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

use std::fmt::Display;

use http::StatusCode;

use crate::entity::{CasdoorConfig, CasdoorUser};

pub struct UserService<'a> {
    config: &'a CasdoorConfig,
}

enum Op {
    Add,
    Delete,
    Update,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "add-user"),
            Op::Delete => write!(f, "delete-user"),
            Op::Update => write!(f, "update-user"),
        }
    }
}

#[allow(dead_code)]
impl<'a> UserService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_users(&self) -> Result<Vec<CasdoorUser>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-users?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_sorted_users(
        &self,
        sorter: String,
        limit: i32,
    ) -> Result<Vec<CasdoorUser>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-sorted-users?owner={}&clientId={}&clientSecret={}&sorter={}&limit={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret,
            sorter,
            limit
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_user_count(
        &self,
        is_online: String,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-user-count?owner={}&clientId={}&clientSecret={}&isOnline={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret,
            is_online
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_user(&self, name: String) -> Result<CasdoorUser, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-user?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_user_with_email(
        &self,
        name: String,
        email: String,
    ) -> Result<CasdoorUser, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-user?id={}/{}&owner={}&clientId={}&clientSecret={}&email={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret,
            email
        );

        println!("{}", url);

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    async fn modify_user(
        &self,
        op: Op,
        user: CasdoorUser,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            user.owner,
            user.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(&user).send().await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn add_user(
        &self,
        user: CasdoorUser,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_user(Op::Add, user).await
    }

    pub async fn delete_user(
        &self,
        user: CasdoorUser,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_user(Op::Delete, user).await
    }

    pub async fn update_user(
        &self,
        user: CasdoorUser,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_user(Op::Update, user).await
    }
}
