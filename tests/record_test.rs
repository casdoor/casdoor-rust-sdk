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

use casdoor_rust_sdk::{get_current_time, Record};
use common::{random_name, test_client};

/// Add a record and read it back as an admin user, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_record() {
    let client = test_client();
    let name = random_name("Record");

    // Add a new object.
    let record = Record {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        created_time: get_current_time(),
        organization: client.config.organization_name.clone(),
        user: "admin".to_string(),
        action: "test-record".to_string(),
        ..Default::default()
    };
    client.add_record(&record).await.unwrap();

    // Reading the records needs the access token of an admin user.
    let token = client
        .get_oauth_token_by_password("admin", "123")
        .await
        .unwrap();
    let admin_client = client.with_access_token(token.access_token);

    // Get all the objects, check if our added object is inside the list.
    let records = admin_client.get_records().await.unwrap();
    assert!(records.iter().any(|item| item.name == name));

    // Get the object.
    let record = admin_client.get_record(&name).await.unwrap().unwrap();
    assert_eq!(record.name, name);

    // Get an object that doesn't exist.
    let missing = format!("{name}_missing");
    assert!(admin_client.get_record(&missing).await.unwrap().is_none());
}
