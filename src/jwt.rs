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

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use x509_parser::prelude::{FromDer, X509Certificate};

use crate::client::Client;
use crate::error::{CasdoorError, Result};
use crate::user::User;

/// Claims is the payload of the JWT token issued by Casdoor, it embeds the [`User`] the
/// token was issued to.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    #[serde(flatten)]
    pub user: User,

    #[serde(default, alias = "TokenType")]
    pub token_type: String,
    #[serde(default)]
    pub nonce: String,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub scope: String,
    /// The `azp` (Authorized Party) claim. Optional.
    /// See <https://openid.net/specs/openid-connect-core-1_0.html#IDToken>.
    #[serde(default)]
    pub azp: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub signin_method: String,

    // The registered claims of RFC 7519.
    #[serde(default)]
    pub iss: String,
    #[serde(default)]
    pub sub: String,
    /// The audience, it is a string or an array of strings depending on the token format.
    #[serde(default)]
    pub aud: Value,
    #[serde(default)]
    pub exp: Option<i64>,
    #[serde(default)]
    pub nbf: Option<i64>,
    #[serde(default)]
    pub iat: Option<i64>,
    #[serde(default)]
    pub jti: String,
}

impl Claims {
    /// Return true if the token is a refresh token.
    ///
    /// Casdoor emits the claim as `tokenType` for the JWT, JWT-Empty and JWT-Standard
    /// formats, and as `TokenType` for JWT-Custom, both land in `token_type`.
    pub fn is_refresh_token(&self) -> bool {
        self.token_type == "refresh-token"
    }
}

impl Client {
    /// Parse and verify a JWT token issued by Casdoor with the application's certificate,
    /// and return its claims. The RSA (`RS256`, `RS384`, `RS512`, `PS256`, `PS384`,
    /// `PS512`) and the ECDSA (`ES256`, `ES384`) signing methods are supported.
    pub fn parse_jwt_token(&self, token: &str) -> Result<Claims> {
        let header = jsonwebtoken::decode_header(token)?;
        let key = self.decoding_key(header.alg)?;

        let mut validation = Validation::new(header.alg);
        validation.validate_aud = false;

        Ok(jsonwebtoken::decode::<Claims>(token, &key, &validation)?.claims)
    }

    fn decoding_key(&self, alg: Algorithm) -> Result<DecodingKey> {
        let pem = public_key_pem(&self.config.certificate)?;

        let key = match alg {
            Algorithm::RS256
            | Algorithm::RS384
            | Algorithm::RS512
            | Algorithm::PS256
            | Algorithm::PS384
            | Algorithm::PS512 => DecodingKey::from_rsa_pem(pem.as_bytes())?,
            Algorithm::ES256 | Algorithm::ES384 => DecodingKey::from_ec_pem(pem.as_bytes())?,
            Algorithm::EdDSA => DecodingKey::from_ed_pem(pem.as_bytes())?,
            _ => {
                return Err(CasdoorError::Certificate(format!(
                    "unsupported signing method: {alg:?}"
                )))
            }
        };

        Ok(key)
    }
}

/// Turn the certificate of the Casdoor application into a public key in the PEM format,
/// which is what the JWT libraries expect. Both an x509 certificate and a bare public key
/// are accepted, so that a certificate copied from Casdoor's "Certs" page just works.
fn public_key_pem(certificate: &str) -> Result<String> {
    let der = decode_pem_body(certificate)?;

    // A certificate wraps the public key, a "PUBLIC KEY" PEM already is the key itself.
    let public_key_der = match X509Certificate::from_der(&der) {
        Ok((_, cert)) => cert.tbs_certificate.subject_pki.raw.to_vec(),
        Err(_) => der,
    };

    let mut pem = String::from("-----BEGIN PUBLIC KEY-----\n");
    for line in STANDARD.encode(&public_key_der).as_bytes().chunks(64) {
        pem.push_str(&String::from_utf8_lossy(line));
        pem.push('\n');
    }
    pem.push_str("-----END PUBLIC KEY-----\n");

    Ok(pem)
}

/// Decode the base64 body of the first PEM block of `content`.
fn decode_pem_body(content: &str) -> Result<Vec<u8>> {
    let body: String = content
        .lines()
        .map(str::trim)
        .skip_while(|line| !line.starts_with("-----BEGIN"))
        .skip(1)
        .take_while(|line| !line.starts_with("-----END"))
        .collect();

    if body.is_empty() {
        return Err(CasdoorError::Certificate(
            "the certificate is empty or is not in the PEM format".to_string(),
        ));
    }

    STANDARD
        .decode(body)
        .map_err(|e| CasdoorError::Certificate(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A self-signed test certificate and a token signed by its private key, the token
    /// expires in the year 2100.
    const CERTIFICATE: &str = include_str!("../tests/testdata/cert.pem");
    const TOKEN: &str = include_str!("../tests/testdata/token.txt");

    fn client() -> Client {
        Client::new(
            "http://localhost:8000",
            "client-id",
            "client-secret",
            CERTIFICATE,
            "built-in",
            "app-built-in",
        )
    }

    #[test]
    fn parse_jwt_token_reads_the_user_of_the_token() {
        let claims = client().parse_jwt_token(TOKEN.trim()).unwrap();

        assert_eq!(claims.user.owner, "built-in");
        assert_eq!(claims.user.name, "admin");
        assert_eq!(claims.user.display_name, "Admin User");
        assert_eq!(claims.user.email, "admin@example.com");
        assert!(claims.user.is_admin);

        // The Casdoor server sends a null for the empty slices of its Go structs.
        assert!(claims.user.groups.is_empty());
        assert!(claims.user.address.is_empty());

        assert_eq!(claims.token_type, "access-token");
        assert!(!claims.is_refresh_token());
        assert_eq!(claims.tag, "staff");
        assert_eq!(claims.iss, "http://localhost:8000");
        assert_eq!(claims.exp, Some(4102444800));
    }

    #[test]
    fn parse_jwt_token_rejects_a_tampered_token() {
        let token = TOKEN.trim();
        let tampered = format!("{}a", &token[..token.len() - 1]);

        assert!(client().parse_jwt_token(&tampered).is_err());
    }

    #[test]
    fn parse_jwt_token_rejects_a_token_when_the_certificate_is_not_set() {
        let client = Client::new("http://localhost:8000", "", "", "", "built-in", "");

        let error = client.parse_jwt_token(TOKEN.trim()).unwrap_err();
        assert!(matches!(error, CasdoorError::Certificate(_)));
    }

    #[test]
    fn public_key_pem_accepts_a_public_key_as_well_as_a_certificate() {
        let from_certificate = public_key_pem(CERTIFICATE).unwrap();
        let from_public_key = public_key_pem(&from_certificate).unwrap();

        assert!(from_certificate.starts_with("-----BEGIN PUBLIC KEY-----\n"));
        assert_eq!(from_certificate, from_public_key);
    }
}
