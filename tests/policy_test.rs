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

use casdoor_rust_sdk::{get_current_time, CasbinRule, Client, Enforcer, PolicyFilter};
use common::{random_name, test_client};

fn rule(ptype: &str, v0: &str, v1: &str, v2: &str) -> CasbinRule {
    CasbinRule {
        ptype: ptype.to_string(),
        v0: v0.to_string(),
        v1: v1.to_string(),
        v2: v2.to_string(),
        ..Default::default()
    }
}

async fn add_enforcer(client: &Client) -> Enforcer {
    let name = random_name("Enforcer");
    let enforcer = Enforcer {
        owner: client.config.organization_name.clone(),
        name: name.clone(),
        created_time: get_current_time(),
        display_name: name,
        model: "built-in/user-model-built-in".to_string(),
        adapter: "built-in/user-adapter-built-in".to_string(),
        description: "Casdoor Website".to_string(),
        ..Default::default()
    };
    assert!(
        client.add_enforcer(&enforcer).await.unwrap(),
        "failed to add enforcer"
    );
    enforcer
}

/// Add, update and remove a policy of an enforcer, it needs a running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_policy() {
    let client = test_client();
    let enforcer = add_enforcer(&client).await;

    // Add a new policy.
    let policy = rule("p", "1", "2", "4");
    client.add_policy(&enforcer, &policy).await.unwrap();

    // Get all the policies, check if our added policy is inside the list.
    let policies = client.get_policies(&enforcer.name, "").await.unwrap();
    assert!(policies
        .iter()
        .any(|item| item.ptype == "p" && item.v2 == "4"));

    // Update the policy.
    let new_policy = rule("p", "1", "2", "5");
    client
        .update_policy(&enforcer, &policy, &new_policy)
        .await
        .unwrap();

    // Validate the update.
    let policies = client.get_policies(&enforcer.name, "").await.unwrap();
    assert!(policies
        .iter()
        .any(|item| item.ptype == "p" && item.v2 == "5"));

    // Remove the policy.
    client.remove_policy(&enforcer, &new_policy).await.unwrap();

    // Validate the removal.
    let policies = client.get_policies(&enforcer.name, "").await.unwrap();
    assert!(!policies
        .iter()
        .any(|item| item.ptype == "p" && item.v2 == "5"));
}

/// Get the policies of an enforcer that match the given filters, it needs a running Casdoor
/// server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_get_filtered_policies() {
    let client = test_client();
    let enforcer = add_enforcer(&client).await;

    for policy in [
        rule("g", "built-in/Test1", "group:built-in/Test1", ""),
        rule("g", "built-in/Test2", "group:built-in/Test2", ""),
        // Unlike test_policy, use values of its own as both tests share the built-in adapter.
        rule("p", "filter_1", "filter_2", "4"),
    ] {
        client.add_policy(&enforcer, &policy).await.unwrap();
    }

    let enforcer_id = format!("{}/{}", client.config.organization_name, enforcer.name);
    let filter = |ptype: &str, field_index: Option<i32>, field_values: &[&str]| PolicyFilter {
        ptype: ptype.to_string(),
        field_index,
        field_values: field_values.iter().map(|value| value.to_string()).collect(),
    };

    // Filter by the first field.
    let policies = client
        .get_filtered_policies(&enforcer_id, &[filter("g", Some(0), &["built-in/Test1"])])
        .await
        .unwrap();
    assert!(policies
        .iter()
        .any(|item| item.ptype == "g" && item.v0 == "built-in/Test1"));

    // Filter by the first field with multiple values.
    let policies = client
        .get_filtered_policies(
            &enforcer_id,
            &[filter("g", Some(0), &["built-in/Test1", "built-in/Test2"])],
        )
        .await
        .unwrap();
    assert!(policies.len() >= 2);

    // Filter by the second field.
    let policies = client
        .get_filtered_policies(
            &enforcer_id,
            &[filter("g", Some(1), &["group:built-in/Test1"])],
        )
        .await
        .unwrap();
    assert!(policies
        .iter()
        .any(|item| item.ptype == "g" && item.v1 == "group:built-in/Test1"));

    // Filter by the policy type only.
    let policies = client
        .get_filtered_policies(&enforcer_id, &[filter("g", None, &[])])
        .await
        .unwrap();
    assert!(policies.len() >= 2);

    // Filter by multiple fields.
    let policies = client
        .get_filtered_policies(
            &enforcer_id,
            &[
                filter("p", Some(0), &["filter_1"]),
                filter("p", Some(1), &["filter_2"]),
            ],
        )
        .await
        .unwrap();
    assert!(policies
        .iter()
        .any(|item| item.ptype == "p" && item.v0 == "filter_1" && item.v1 == "filter_2"));
}
