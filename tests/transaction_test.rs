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

use casdoor_rust_sdk::{get_current_time, Transaction};
use common::test_client;

/// The whole life cycle of a transaction, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_transaction() {
    let client = test_client();

    // Add a new object, the server generates its name.
    let transaction = Transaction {
        owner: client.config.organization_name.clone(),
        created_time: get_current_time(),
        state: "Paid".to_string(),
        ..Default::default()
    };
    let (_, name) = client.add_transaction(&transaction).await.unwrap();

    // Get all the objects, check if our added object is inside the list.
    let transactions = client.get_transactions().await.unwrap();
    assert!(transactions.iter().any(|item| item.name == name));

    // Get the object.
    let mut transaction = client.get_transaction(&name).await.unwrap().unwrap();
    assert_eq!(transaction.name, name);

    // Update the object.
    let updated_display_name = "Organization";
    transaction.display_name = updated_display_name.to_string();
    assert!(client.update_transaction(&transaction).await.unwrap());

    // Validate the update.
    let transaction = client.get_transaction(&name).await.unwrap().unwrap();
    assert_eq!(transaction.display_name, updated_display_name);

    // Delete the object.
    assert!(client.delete_transaction(&transaction).await.unwrap());

    // Validate the deletion.
    assert!(client.get_transaction(&name).await.unwrap().is_none());
}
