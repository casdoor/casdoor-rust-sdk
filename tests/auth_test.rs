// Copyright 2021 The Casdoor Authors. All Rights Reserved.
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

use casdoor_rust_sdk::{AuthService, CasdoorConfig};

fn abs_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let absolute_path = std::env::current_dir()?.join("tests").join(path);
    Ok(absolute_path.to_str().unwrap().to_string())
}

#[tokio::main]
#[test]
async fn test_get_signin_url() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let auth_src = AuthService::new(&conf);

    let url = auth_src.get_user_profile_url("admin".to_string(), None);
    assert!(!url.is_empty());
}

#[tokio::main]
#[test]
async fn test_get_signup_url() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let auth_src = AuthService::new(&conf);

    let url = auth_src.get_signup_url_enable_password();
    assert!(!url.is_empty());
}

#[tokio::main]
#[test]
async fn test_get_user_profile_url() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let auth_src = AuthService::new(&conf);

    let url = auth_src.get_user_profile_url("admin".to_string(), None);
    assert!(!url.is_empty());
}

#[tokio::main]
#[test]
async fn test_get_my_profile_url() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let auth_src = AuthService::new(&conf);

    let url = auth_src.get_my_profile_url(None);
    assert!(!url.is_empty());
}

#[tokio::main]
#[test]
async fn test_parse_auth_tokens() {
    let conf = CasdoorConfig::from_toml(abs_path("./conf.toml").unwrap().as_str()).unwrap();
    let auth_src = AuthService::new(&conf);

    let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJ1aWx0LWluIiwibmFtZSI6ImFkbWluIiwiY3JlYXRlZFRpbWUiOiIyMDIyLTA3LTA1VDEwOjA4OjExWiIsInVwZGF0ZWRUaW1lIjoiIiwiaWQiOiI5Y2UwNDdmNC0xOTc0LTQ4MDMtYWNlZC02Yzc5NjU0ZDkzOTciLCJ0eXBlIjoibm9ybWFsLXVzZXIiLCJwYXNzd29yZCI6IiIsInBhc3N3b3JkU2FsdCI6IiIsImRpc3BsYXlOYW1lIjoiQWRtaW4iLCJmaXJzdE5hbWUiOiIiLCJsYXN0TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vY2FzYmluLm9yZy9pbWcvY2FzYmluLnN2ZyIsInBlcm1hbmVudEF2YXRhciI6IiIsImVtYWlsIjoiYWRtaW5AZXhhbXBsZS5jb20iLCJlbWFpbFZlcmlmaWVkIjpmYWxzZSwicGhvbmUiOiIxMjM0NTY3ODkxMCIsImxvY2F0aW9uIjoiIiwiYWRkcmVzcyI6W10sImFmZmlsaWF0aW9uIjoiRXhhbXBsZSBJbmMuIiwidGl0bGUiOiIiLCJpZENhcmRUeXBlIjoiIiwiaWRDYXJkIjoiIiwiaG9tZXBhZ2UiOiIiLCJiaW8iOiIiLCJyZWdpb24iOiIiLCJsYW5ndWFnZSI6IiIsImdlbmRlciI6IiIsImJpcnRoZGF5IjoiIiwiZWR1Y2F0aW9uIjoiIiwic2NvcmUiOjIwMDAsImthcm1hIjowLCJyYW5raW5nIjoxLCJpc0RlZmF1bHRBdmF0YXIiOmZhbHNlLCJpc09ubGluZSI6ZmFsc2UsImlzQWRtaW4iOnRydWUsImlzR2xvYmFsQWRtaW4iOnRydWUsImlzRm9yYmlkZGVuIjpmYWxzZSwiaXNEZWxldGVkIjpmYWxzZSwic2lnbnVwQXBwbGljYXRpb24iOiJidWlsdC1pbi1hcHAiLCJoYXNoIjoiIiwicHJlSGFzaCI6IiIsImNyZWF0ZWRJcCI6IjEyNy4wLjAuMSIsImxhc3RTaWduaW5UaW1lIjoiIiwibGFzdFNpZ25pbklwIjoiIiwiZ2l0aHViIjoiIiwiZ29vZ2xlIjoiIiwicXEiOiIiLCJ3ZWNoYXQiOiIiLCJ1bmlvbklkIjoiIiwiZmFjZWJvb2siOiIiLCJkaW5ndGFsayI6IiIsIndlaWJvIjoiIiwiZ2l0ZWUiOiIiLCJsaW5rZWRpbiI6IiIsIndlY29tIjoiIiwibGFyayI6IiIsImdpdGxhYiI6IiIsImFkZnMiOiIiLCJiYWlkdSI6IiIsImFsaXBheSI6IiIsImNhc2Rvb3IiOiIiLCJpbmZvZmxvdyI6IiIsImFwcGxlIjoiIiwiYXp1cmVhZCI6IiIsInNsYWNrIjoiIiwic3RlYW0iOiIiLCJiaWxpYmlsaSI6IiIsIm9rdGEiOiIiLCJkb3V5aW4iOiIiLCJjdXN0b20iOiIiLCJsZGFwIjoiIiwicHJvcGVydGllcyI6e30sInRhZyI6InN0YWZmIiwic2NvcGUiOiJyZWFkIiwiaXNzIjoiaHR0cDovL2xvY2FsaG9zdDo4MDAwIiwic3ViIjoiOWNlMDQ3ZjQtMTk3NC00ODAzLWFjZWQtNmM3OTY1NGQ5Mzk3IiwiYXVkIjpbIjI3MGM0MzJkZmY5YjQwOWJhYzY5Il0sImV4cCI6MTY1NzgwMTk4MSwibmJmIjoxNjU3MTk3MTgxLCJpYXQiOjE2NTcxOTcxODF9.pJi0D3SlyVPiv4ZKCFw2clTiirmlU6YARo1xa22Z3ZIX5VKl7EdU1T-e4n5YIkvVEM8r9Eso8w_6k-sK9ZCtaC-AQ03SSQx9ZgIzUtFq7wUTpeEL9FpK9_rIGnktefpL6tzPzH57kV0zQwEq2XNVFGKKCwYUshDjwft_nXkjlP9NS8f-mwI9MlOej87JE1lgSyHgKFvKA8WMC5FrJ6cG3mXiIbbxka5wVUKf8hlRky4vJ5A0e0Ype6WvGlR3bHSAWLUvIElJE1ubX4fdW6u7MN98bR2Aijt5qb5_6h5pt_nJ1N7SUqKjjOikvDXoSWcWf2EYZTwB_aO6I-ysnFL8ipbylyb2UYyhr4AHKe9ECFrlfdTWzAuhf0IGKLPUqk5GbZZikleBlPanwK_OjzRVEFRILtkUZ68MJ2x75lzva_6Rvbf0NsK-nII2-wGhtZtz6Tm9KiFfUEgbXG_rLx4EvDS8iOkQjiDOAq5zN7bwmpWDDzBbr7z0pug_RoGgpUf55hkaOhcuGHU7nrr4EflhGIiw_Cd6SeP6s3Z4PjDCxP7ktR8jLdy8S_fv5eMejOLeYMspQWPOGdqmWfYacF8he6Q2Xu6zD7yVHdQ9uPDPSGBHnw1-xopJoyfZr85J4Och8Yx9F1r4pjofrLvx4l_OGMNGOSoUkK3hHcxHJ-O58HU";

    let user = auth_src.parse_jwt_token(token.to_string()).unwrap();
    println!("{:?}", user);
}
