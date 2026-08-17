//! Tests of /v1/teams/{team_id}/sources

mod common;

use common::assert_api_status;
use common::fixture::update_team_sources;
use gitguardian_mock::MockServer;
use http::StatusCode;

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::{client, client_preferring};

    #[test]
    /// GIVEN sources to add to a team perimeter
    /// WHEN updating the perimeter
    /// THEN the empty response is accepted
    fn updates_team_sources() {
        client(MockServer::shared())
            .send(&update_team_sources())
            .expect("team perimeter update should succeed");
    }

    #[test]
    /// GIVEN a server that rejects the api key
    /// WHEN updating the perimeter
    /// THEN the api error reaches the caller instead of a transport error
    fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&update_team_sources())
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::{client, client_preferring};

    #[tokio::test]
    /// GIVEN sources to add to a team perimeter
    /// WHEN updating the perimeter
    /// THEN the empty response is accepted
    async fn updates_team_sources() {
        client(MockServer::shared())
            .send(&update_team_sources())
            .await
            .expect("team perimeter update should succeed");
    }

    #[tokio::test]
    /// GIVEN a server that rejects the api key
    /// WHEN updating the perimeter
    /// THEN the api error reaches the caller instead of a transport error
    async fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&update_team_sources())
            .await
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}
