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

use casdoor_rust_sdk::Client;

fn client() -> Client {
    Client::new(
        "http://localhost:8000",
        "client-id",
        "client-secret",
        "",
        "built-in",
        "app-built-in",
    )
}

#[test]
fn test_get_signin_url() {
    assert_eq!(
        client().get_signin_url("http://localhost:9000/callback"),
        "http://localhost:8000/login/oauth/authorize?client_id=client-id&response_type=code\
         &redirect_uri=http%3A%2F%2Flocalhost%3A9000%2Fcallback&scope=read&state=app-built-in"
    );
}

#[test]
fn test_get_signup_url() {
    assert_eq!(
        client().get_signup_url(true, ""),
        "http://localhost:8000/signup/app-built-in"
    );

    assert!(client()
        .get_signup_url(false, "http://localhost:9000/callback")
        .starts_with("http://localhost:8000/signup/oauth/authorize?"));
}

#[test]
fn test_get_user_profile_url() {
    assert_eq!(
        client().get_user_profile_url("admin", None),
        "http://localhost:8000/users/built-in/admin"
    );

    assert_eq!(
        client().get_user_profile_url("admin", Some("token")),
        "http://localhost:8000/users/built-in/admin?access_token=token"
    );
}

#[test]
fn test_get_my_profile_url() {
    assert_eq!(
        client().get_my_profile_url(None),
        "http://localhost:8000/account"
    );

    assert_eq!(
        client().get_my_profile_url(Some("token")),
        "http://localhost:8000/account?access_token=token"
    );
}
