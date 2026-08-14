//! Tests of /v1/teams

mod common;

use common::assert_api_status;
use common::fixture::create_team;
use gitguardian_mock::MockServer;
use http::StatusCode;

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::{client, client_preferring};

    #[test]
    /// GIVEN a team name
    /// WHEN creating the team
    /// THEN the created team is returned
    fn creates_team() {
        let server = MockServer::shared();

        let team = client(server)
            .send(&create_team())
            .expect("team creation should succeed");

        assert!(!team.name.is_empty());
        assert!(!team.gitguardian_url.is_empty());
    }

    #[test]
    /// GIVEN a server that rejects the api key
    /// WHEN creating a team
    /// THEN the api error reaches the caller instead of a transport error
    fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&create_team())
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::{client, client_preferring};

    #[tokio::test]
    /// GIVEN a team name
    /// WHEN creating the team
    /// THEN the created team is returned
    async fn creates_team() {
        let server = MockServer::shared();

        let team = client(server)
            .send(&create_team())
            .await
            .expect("team creation should succeed");

        assert!(!team.name.is_empty());
        assert!(!team.gitguardian_url.is_empty());
    }

    #[tokio::test]
    /// GIVEN a server that rejects the api key
    /// WHEN creating a team
    /// THEN the api error reaches the caller instead of a transport error
    async fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&create_team())
            .await
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}
