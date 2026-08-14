//! Tests of /v1/auth/jwt

mod common;

use common::assert_api_status;
use common::fixture::create_jwt;
use gitguardian_mock::MockServer;
use http::StatusCode;

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::{client, client_preferring};

    #[test]
    /// GIVEN an audience
    /// WHEN creating a jwt for it
    /// THEN a token is returned
    fn creates_jwt() {
        let server = MockServer::shared();

        let jwt = client(server)
            .send(&create_jwt())
            .expect("jwt creation should succeed");

        assert!(!jwt.token.is_empty());
    }

    #[test]
    /// GIVEN a server that rejects the api key
    /// WHEN creating a jwt
    /// THEN the api error reaches the caller instead of a transport error
    fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&create_jwt())
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::{client, client_preferring};

    #[tokio::test]
    /// GIVEN an audience
    /// WHEN creating a jwt for it
    /// THEN a token is returned
    async fn creates_jwt() {
        let server = MockServer::shared();

        let jwt = client(server)
            .send(&create_jwt())
            .await
            .expect("jwt creation should succeed");

        assert!(!jwt.token.is_empty());
    }

    #[tokio::test]
    /// GIVEN a server that rejects the api key
    /// WHEN creating a jwt
    /// THEN the api error reaches the caller instead of a transport error
    async fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&create_jwt())
            .await
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}
