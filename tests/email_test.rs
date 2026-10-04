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

use common::test_client;

/// Send an email through the SMTP provider of the application, it needs a running Casdoor
/// server. The CI server has no real SMTP server, so a refused connection is expected.
#[tokio::test]
#[ignore = "needs a running Casdoor server"]
async fn test_email() {
    let client = test_client();

    let result = client
        .send_email(
            "casbin",
            "casdoor-rust-sdk website test",
            "admin",
            &["TestSmtpServer"],
        )
        .await;
    if let Err(err) = result {
        let message = err.to_string();
        assert!(
            ["535 Error", "i/o timeout", "refused"]
                .iter()
                .any(|expected| message.contains(expected)),
            "failed to send email: {message}"
        );
    }
}
