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

use casdoor_rust_sdk::User;
use common::{random_name, test_client};

/// The whole life cycle of a user, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_user() {
    let client = test_client();
    let name = random_name("User");

    // Add a new object.
    let user = User {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        display_name: name.clone(),
        ..Default::default()
    };
    assert!(client.add_user(&user).await.unwrap(), "failed to add user");

    // Get all the objects, check if our added object is inside the list.
    let users = client.get_users().await.unwrap();
    assert!(users.iter().any(|user| user.name == name));

    // Get the object.
    let mut user = client.get_user(&name).await.unwrap().unwrap();
    assert_eq!(user.name, name);

    // Update the object.
    let updated_display_name = "Updated Display Name";
    user.display_name = updated_display_name.to_string();
    assert!(client.update_user(&user).await.unwrap());

    // Validate the update.
    let user = client.get_user(&name).await.unwrap().unwrap();
    assert_eq!(user.display_name, updated_display_name);

    // Delete the object.
    assert!(client.delete_user(&user).await.unwrap());

    // Validate the deletion.
    assert!(client.get_user(&name).await.unwrap().is_none());
}

#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_get_user_count() {
    let client = test_client();

    let count = client.get_user_count("").await.unwrap();
    assert!(count > 0);
}

#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_get_pagination_users() {
    let client = test_client();

    let (users, count) = client.get_pagination_users(1, 10, &[]).await.unwrap();
    assert!(!users.is_empty());
    assert!(count >= users.len() as i32);
}
