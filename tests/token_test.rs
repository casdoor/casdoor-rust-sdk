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

mod common;

use casdoor_rust_sdk::{get_current_time, Token};
use common::{random_name, test_client};

/// The whole life cycle of a token, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_token() {
    let client = test_client();
    let name = random_name("Token");

    // Add a new object.
    let token = Token {
        owner: "admin".to_string(),
        name: name.clone(),
        created_time: get_current_time(),
        organization: "casbin".to_string(),
        code: "abc".to_string(),
        access_token: "123456".to_string(),
        ..Default::default()
    };
    assert!(client.add_token(&token).await.unwrap());

    // Get all the objects, check if our added object is inside the list.
    let tokens = client.get_tokens().await.unwrap();
    assert!(tokens.iter().any(|item| item.name == name));

    // Get the object.
    let mut token = client.get_token(&name).await.unwrap().unwrap();
    assert_eq!(token.name, name);

    // Update the object.
    let updated_code = "Updated Code";
    token.code = updated_code.to_string();
    assert!(client.update_token(&token).await.unwrap());

    // Validate the update.
    let token = client.get_token(&name).await.unwrap().unwrap();
    assert_eq!(token.code, updated_code);

    // Delete the object.
    assert!(client.delete_token(&token).await.unwrap());

    // Validate the deletion, the server may report the deleted token as not existing.
    match client.get_token(&name).await {
        Ok(token) => assert!(token.is_none()),
        Err(err) => assert!(err.to_string().contains("does not exist"), "{err}"),
    }
}
