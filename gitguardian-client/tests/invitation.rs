//! Tests of /v1/invitations

mod common;

use common::assert_api_status;
use common::fixture::create_invitation;
use gitguardian_mock::MockServer;
use http::StatusCode;

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::{client, client_preferring};

    #[test]
    /// GIVEN an email address
    /// WHEN creating an invitation for it
    /// THEN the created invitation is returned
    fn creates_invitation() {
        let server = MockServer::shared();

        let invitation = client(server)
            .send(&create_invitation())
            .expect("invitation creation should succeed");

        assert!(!invitation.email.is_empty());
        assert!(!invitation.access_level.is_empty());
    }

    #[test]
    /// GIVEN a server that rejects the api key
    /// WHEN creating an invitation
    /// THEN the api error reaches the caller instead of a transport error
    fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&create_invitation())
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::{client, client_preferring};

    #[tokio::test]
    /// GIVEN an email address
    /// WHEN creating an invitation for it
    /// THEN the created invitation is returned
    async fn creates_invitation() {
        let server = MockServer::shared();

        let invitation = client(server)
            .send(&create_invitation())
            .await
            .expect("invitation creation should succeed");

        assert!(!invitation.email.is_empty());
        assert!(!invitation.access_level.is_empty());
    }

    #[tokio::test]
    /// GIVEN a server that rejects the api key
    /// WHEN creating an invitation
    /// THEN the api error reaches the caller instead of a transport error
    async fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&create_invitation())
            .await
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}
