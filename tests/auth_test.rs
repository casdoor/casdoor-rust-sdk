// Copyright 2021 The Casdoor Authors. All Rights Reserved.
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

use casdoor_rust_sdk::{AuthService, CasdoorConfig};

fn abs_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let absolute_path = std::env::current_dir()?.join("tests").join(path);
    Ok(absolute_path.to_str().unwrap().to_string())
}

#[tokio::main]
#[test]
async fn test_get_signin_url() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let auth_src = AuthService::new(&conf);

    let url = auth_src.get_user_profile_url("admin".to_string(), None);
    assert!(!url.is_empty());
}

#[tokio::main]
#[test]
async fn test_get_signup_url() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let auth_src = AuthService::new(&conf);

    let url = auth_src.get_signup_url_enable_password();
    assert!(!url.is_empty());
}

#[tokio::main]
#[test]
async fn test_get_user_profile_url() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let auth_src = AuthService::new(&conf);

    let url = auth_src.get_user_profile_url("admin".to_string(), None);
    assert!(!url.is_empty());
}

#[tokio::main]
#[test]
async fn test_get_my_profile_url() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let auth_src = AuthService::new(&conf);

    let url = auth_src.get_my_profile_url(None);
    assert!(!url.is_empty());
}

#[tokio::main]
#[test]
async fn test_get_and_parse_auth_tokens() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let auth_src = AuthService::new(&conf);

    let token = auth_src
        .get_auth_token("71b645e73381caeb2c66".to_string())
        .unwrap();

    let user = auth_src.parse_jwt_token(token).unwrap();
    println!("{:?}", user);
}
