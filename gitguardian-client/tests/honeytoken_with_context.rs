//! Tests of /v1/honeytokens/with-context

mod common;

use common::assert_api_status;
use common::fixture::create_honeytoken_with_context;
use gitguardian_mock::MockServer;
use http::StatusCode;

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::{client, client_preferring};

    #[test]
    /// GIVEN a honeytoken name, type and context hints
    /// WHEN creating the honeytoken within a context
    /// THEN the file holding the honeytoken is returned
    fn creates_honeytoken_with_context() {
        let server = MockServer::shared();

        let context = client(server)
            .send(&create_honeytoken_with_context())
            .expect("honeytoken with context creation should succeed");

        assert!(!context.content.is_empty());
        assert!(!context.filename.is_empty());
        assert!(!context.honeytoken_id.is_nil());
    }

    #[test]
    /// GIVEN a server that rejects the api key
    /// WHEN creating a honeytoken within a context
    /// THEN the api error reaches the caller instead of a transport error
    fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&create_honeytoken_with_context())
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::{client, client_preferring};

    #[tokio::test]
    /// GIVEN a honeytoken name, type and context hints
    /// WHEN creating the honeytoken within a context
    /// THEN the file holding the honeytoken is returned
    async fn creates_honeytoken_with_context() {
        let server = MockServer::shared();

        let context = client(server)
            .send(&create_honeytoken_with_context())
            .await
            .expect("honeytoken with context creation should succeed");

        assert!(!context.content.is_empty());
        assert!(!context.filename.is_empty());
        assert!(!context.honeytoken_id.is_nil());
    }

    #[tokio::test]
    /// GIVEN a server that rejects the api key
    /// WHEN creating a honeytoken within a context
    /// THEN the api error reaches the caller instead of a transport error
    async fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&create_honeytoken_with_context())
            .await
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}
