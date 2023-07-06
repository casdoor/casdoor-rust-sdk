# casdoor-rust-sdk

[![GitHub last commit](https://img.shields.io/github/last-commit/casdoor/casdoor-rust-sdk)](https://github.com/casdoor/casdoor-rust-sdk/commits/master)
[![Crates.io](https://img.shields.io/crates/v/casdoor-rust-sdk.svg)](https://crates.io/crates/casdoor-rust-sdk)
[![Docs](https://docs.rs/casdoor-rust-sdk/badge.svg)](https://docs.rs/casdoor-rust-sdk)
[![CI](https://github.com/casdoor/casdoor-rust-sdk/workflows/CI/badge.svg)](https://github.com/casdoor/casdoor-rust-sdk/actions)
[![Discord](https://img.shields.io/discord/1022748306096537660?logo=discord&label=discord&color=5865F2)](https://discord.gg/5rPsrAzK7S)

This is Casdoor's SDK for Rust, which will allow you to easily connect your application to the Casdoor authentication system without having to implement it from scratch.

Casdoor SDK is very simple to use. We will show you the steps below.

```toml
[dependencies]
casdoor-rust-sdk = <latest-version>
```

## Step1. Init SDK

Initialization requires 6 parameters, which are all string type:

| Name (in order) | Must | Description                                         |
| --------------- | ---- | --------------------------------------------------- |
| endpoint        | Yes  | Casdoor Server Url, such as `http://localhost:8000` |
| client_id       | Yes  | Client ID for the Casdoor application               |
| client_secret   | Yes  | Client secret for the Casdoor application           |
| certificate     | Yes  | x509 certificate content of Application.cert        |
| org_name        | Yes  | The name for the Casdoor organization               |
| app_name        | No   | The name for the Casdoor application                |

```rust
// init from params.
let app = CasdoorConfig::new(endpoint, client_id, client_secret, certificate, org_name);

// init from toml file, file_path should be absolute path. (recommend)
let conf = CasdoorConfig::from_toml(file_path).unwrap().as_str()).unwrap();
```

## Step2. Get service and use

Now provide two services: `CasdoorUserService`, `CasdoorAuthService`

You can create them like:

```rust
let user_service = UserService::new(&conf);
let auth_src = AuthService::new(&conf);
```

## Step3. Interact with the sdk services

The SDK support basic operations.

**user**

- `get_user(name)`, get one user by user name.
- `get_users()`, get all users.
- `get_sorted_users(sorter, limit)`, get sorted user and limit number of result.
- `get_user_count(is_online)`, get user count.
- `add_user(User)`, write user to database.
- `update_user(User)`, update user info
- `delete_user(User)`, delete user

**auth**

- `get_auth_token(code)`, get the auth token.
- `parse_jwt_token(token)`, parse jwt token.
