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

use casdoor_rust_sdk::{get_current_time, Syncer};
use common::{random_name, test_client};

/// The whole life cycle of a syncer, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_syncer() {
    let client = test_client();
    let name = random_name("Syncer");

    // Add a new object.
    let syncer = Syncer {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        created_time: get_current_time(),
        organization: "casbin".to_string(),
        host: "localhost".to_string(),
        port: 3306,
        user: "root".to_string(),
        password: "123".to_string(),
        database_type: "mysql".to_string(),
        database: "syncer_db".to_string(),
        table: "user_table".to_string(),
        sync_interval: 1,
        ..Default::default()
    };
    assert!(
        client.add_syncer(&syncer).await.unwrap(),
        "failed to add syncer"
    );

    // Get all the objects, check if our added object is inside the list.
    let items = client.get_syncers().await.unwrap();
    assert!(items.iter().any(|item| item.name == name));

    // Get the object.
    let mut syncer = client.get_syncer(&name).await.unwrap().unwrap();
    assert_eq!(syncer.name, name);

    // Update the object.
    let updated_value = "Updated Host";
    syncer.host = updated_value.to_string();
    assert!(client.update_syncer(&syncer).await.unwrap());

    // Validate the update.
    let syncer = client.get_syncer(&name).await.unwrap().unwrap();
    assert_eq!(syncer.host, updated_value);

    // Delete the object.
    assert!(client.delete_syncer(&syncer).await.unwrap());

    // Validate the deletion.
    assert!(client.get_syncer(&name).await.unwrap().is_none());
}
