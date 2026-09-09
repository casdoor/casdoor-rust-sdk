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

use thiserror::Error;

/// The error type returned by all the SDK APIs.
#[derive(Debug, Error)]
pub enum CasdoorError {
    /// The Casdoor server returned a response whose `status` is not `"ok"`,
    /// the message is the `msg` field of the response.
    #[error("{0}")]
    Casdoor(String),

    /// The Casdoor server returned an unexpected HTTP status code.
    #[error("status code: {status}, body: {body}")]
    HttpStatus { status: u16, body: String },

    /// The response data can't be converted to the expected type.
    #[error("response data format is incorrect")]
    InvalidData,

    /// The HTTP request failed.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// The JSON serialization or deserialization failed.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// The config file can't be read or parsed.
    #[error("config error: {0}")]
    Config(String),

    /// The certificate of the application is invalid.
    #[error("certificate error: {0}")]
    Certificate(String),

    /// The JWT token is invalid or can't be verified by the certificate.
    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

/// The result type returned by all the SDK APIs.
pub type Result<T> = std::result::Result<T, CasdoorError>;
