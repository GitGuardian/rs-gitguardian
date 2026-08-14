//! Tests of /v1/honeytokens

mod common;

use common::assert_api_status;
use common::fixture::create_honeytoken;
use gitguardian_api::models::honeytoken_status::HoneytokenStatus;
use gitguardian_api::models::honeytoken_type::HoneytokenType;
use gitguardian_mock::MockServer;
use http::StatusCode;

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::{client, client_preferring};

    #[test]
    /// GIVEN a honeytoken name and type
    /// WHEN creating the honeytoken
    /// THEN the created honeytoken is returned
    fn creates_honeytoken() {
        let server = MockServer::shared();

        let honeytoken = client(server)
            .send(&create_honeytoken())
            .expect("honeytoken creation should succeed");

        assert!(!honeytoken.id.is_nil());
        assert_eq!(honeytoken.honeytoken_type, HoneytokenType::Aws);
        assert_eq!(honeytoken.status, HoneytokenStatus::Active);
        assert!(!honeytoken.token.is_empty());
    }

    #[test]
    /// GIVEN a server that rejects the api key
    /// WHEN creating a honeytoken
    /// THEN the api error reaches the caller instead of a transport error
    fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&create_honeytoken())
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::{client, client_preferring};

    #[tokio::test]
    /// GIVEN a honeytoken name and type
    /// WHEN creating the honeytoken
    /// THEN the created honeytoken is returned
    async fn creates_honeytoken() {
        let server = MockServer::shared();

        let honeytoken = client(server)
            .send(&create_honeytoken())
            .await
            .expect("honeytoken creation should succeed");

        assert!(!honeytoken.id.is_nil());
        assert_eq!(honeytoken.honeytoken_type, HoneytokenType::Aws);
        assert_eq!(honeytoken.status, HoneytokenStatus::Active);
        assert!(!honeytoken.token.is_empty());
    }

    #[tokio::test]
    /// GIVEN a server that rejects the api key
    /// WHEN creating a honeytoken
    /// THEN the api error reaches the caller instead of a transport error
    async fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&create_honeytoken())
            .await
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}
