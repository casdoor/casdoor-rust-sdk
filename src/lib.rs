// Copyright 2022 The Casdoor Authors. All Rights Reserved.
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

//! Casdoor's SDK for Rust, it lets an application use [Casdoor](https://casdoor.org) as
//! its authentication and authorization system, without having to implement one from
//! scratch.
//!
//! Everything is done through the [`Client`], which is created from the config of a
//! Casdoor application:
//!
//! ```no_run
//! use casdoor_rust_sdk::Client;
//!
//! # async fn f() -> casdoor_rust_sdk::Result<()> {
//! let client = Client::new(
//!     "http://localhost:8000",
//!     "<client-id>",
//!     "<client-secret>",
//!     "<certificate>",
//!     "built-in",
//!     "app-built-in",
//! );
//!
//! // Redirect the user to this URL to sign in.
//! let signin_url = client.get_signin_url("http://localhost:9000/callback");
//!
//! // In the callback, exchange the code for an access token and read the user.
//! let token = client.get_oauth_token("<code>").await?;
//! let claims = client.parse_jwt_token(&token.access_token)?;
//! println!("signed in as {}", claims.user.name);
//!
//! // Call the APIs as the application itself.
//! let users = client.get_users().await?;
//! # Ok(())
//! # }
//! ```
//!
//! The SDK is kept in sync with the [Go SDK](https://github.com/casdoor/casdoor-go-sdk),
//! one Rust module per Go file, so that the two are easy to compare.

mod adapter;
mod application;
mod auth;
mod cert;
mod client;
mod config;
mod email;
mod enforce;
mod enforcer;
mod error;
mod group;
mod invitation;
mod jwt;
mod ldap;
mod logout;
mod mfa;
mod model;
mod notification;
mod order;
mod order_pay;
mod organization;
mod payment;
mod permission;
mod plan;
mod policy;
mod pricing;
mod product;
mod provider;
mod record;
mod resource;
mod role;
mod serde_util;
mod session;
mod sms;
mod subscription;
mod syncer;
mod token;
mod transaction;
mod url;
mod user;
mod webhook;

pub use adapter::*;
pub use application::*;
pub use auth::*;
pub use cert::*;
pub use client::*;
pub use config::*;
pub use enforce::*;
pub use enforcer::*;
pub use error::*;
pub use group::*;
pub use invitation::*;
pub use jwt::*;
pub use ldap::*;
pub use mfa::*;
pub use model::*;
pub use order::*;
pub use organization::*;
pub use payment::*;
pub use permission::*;
pub use plan::*;
pub use policy::*;
pub use pricing::*;
pub use product::*;
pub use provider::*;
pub use record::*;
pub use resource::*;
pub use role::*;
pub use session::*;
pub use subscription::*;
pub use syncer::*;
pub use token::*;
pub use transaction::*;
pub use user::*;
pub use webhook::*;

/// The old name of [`AuthConfig`], kept to ease the migration from the 1.x versions.
#[deprecated(since = "2.0.0", note = "renamed to `AuthConfig`")]
pub type CasdoorConfig = AuthConfig;

/// The old name of [`User`], kept to ease the migration from the 1.x versions.
#[deprecated(since = "2.0.0", note = "renamed to `User`")]
pub type CasdoorUser = User;
