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

//! The helpers shared by the integration tests. The tests marked with `#[ignore]` need a
//! running Casdoor server, they are run with:
//!
//! ```shell
//! cargo test -- --ignored
//! ```

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use casdoor_rust_sdk::Client;

/// The client used by the tests that need a running Casdoor server, it is configured with
/// the same environment variables as the Casdoor Go SDK.
pub fn test_client() -> Client {
    Client::new(
        get_env("CASDOOR_TEST_ENDPOINT", "http://localhost:8000"),
        get_env("CASDOOR_TEST_CLIENT_ID", "casdoor-rust-sdk-ci-client"),
        get_env("CASDOOR_TEST_CLIENT_SECRET", "casdoor-rust-sdk-ci-secret"),
        "",
        get_env("CASDOOR_TEST_ORGANIZATION", "casbin"),
        get_env("CASDOOR_TEST_APPLICATION", "app-casibase"),
    )
}

/// A name that no other object of the Casdoor server has.
pub fn random_name(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    format!("{prefix}_{nanos}")
}

fn get_env(key: &str, default_value: &str) -> String {
    match env::var(key) {
        Ok(value) if !value.is_empty() => value,
        _ => default_value.to_string(),
    }
}
