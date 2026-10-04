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

use casdoor_rust_sdk::{Adapter, CasbinRule, Enforcer, Model};
use common::{random_name, test_client};
use serde_json::json;

const MODEL_TEXT: &str = r#"[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = r.sub == p.sub && r.obj == p.obj && r.act == p.act"#;

/// Enforce and batch enforce against an enforcer with its own model and adapter, it needs a
/// running Casdoor server.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_enforce() {
    let client = test_client();
    let owner = client.config.organization_name.clone();

    let model_name = random_name("enforceModel");
    let model = Model {
        owner: owner.clone(),
        name: model_name.clone(),
        display_name: model_name.clone(),
        model_text: MODEL_TEXT.to_string(),
        ..Default::default()
    };
    assert!(
        client.add_model(&model).await.unwrap(),
        "failed to add model"
    );

    let adapter_name = random_name("enforceAdapter");
    let adapter = Adapter {
        owner: owner.clone(),
        name: adapter_name.clone(),
        table: format!("{adapter_name}_policy"),
        use_same_db: true,
        ..Default::default()
    };
    assert!(
        client.add_adapter(&adapter).await.unwrap(),
        "failed to add adapter"
    );

    let enforcer_name = random_name("enforceEnforcer");
    let enforcer = Enforcer {
        owner: owner.clone(),
        name: enforcer_name.clone(),
        display_name: enforcer_name.clone(),
        model: format!("{owner}/{model_name}"),
        adapter: format!("{owner}/{adapter_name}"),
        ..Default::default()
    };
    assert!(
        client.add_enforcer(&enforcer).await.unwrap(),
        "failed to add enforcer"
    );

    for (sub, obj, act) in [("alice", "data1", "read"), ("bob", "data2", "write")] {
        let policy = CasbinRule {
            ptype: "p".to_string(),
            v0: sub.to_string(),
            v1: obj.to_string(),
            v2: act.to_string(),
            ..Default::default()
        };
        assert!(
            client.add_policy(&enforcer, &policy).await.unwrap(),
            "failed to add policy"
        );
    }

    let enforcer_id = format!("{owner}/{enforcer_name}");
    let request1 = vec![json!("alice"), json!("data1"), json!("read")];
    let request2 = vec![json!("bob"), json!("data2"), json!("write")];
    let request_fail = vec![json!("alice"), json!("data1"), json!("write")];

    assert!(client
        .enforce("", "", "", &enforcer_id, "", &request1)
        .await
        .unwrap());
    assert!(client
        .enforce("", "", "", &enforcer_id, "", &request2)
        .await
        .unwrap());
    assert!(!client
        .enforce("", "", "", &enforcer_id, "", &request_fail)
        .await
        .unwrap());

    let results = client
        .batch_enforce("", "", "", &enforcer_id, "", &[request1, request_fail])
        .await
        .unwrap();
    assert!(results[0][0]);
    assert!(!results[0][1]);
}
