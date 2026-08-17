//! Tests of /v1/teams/{team_id}/team_invitations

mod common;

use common::assert_api_status;
use common::fixture::list_team_invitations;
use gitguardian_mock::MockServer;
use http::StatusCode;

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::{client, client_preferring};

    #[test]
    /// GIVEN a team id and filter parameters
    /// WHEN listing the team invitations
    /// THEN a page of team invitations is returned
    fn lists_team_invitations() {
        let server = MockServer::shared();

        let page = client(server)
            .send(&list_team_invitations())
            .expect("team invitation listing should succeed");

        assert!(!page.items.is_empty());
        assert!(
            page.items
                .iter()
                .all(|invitation| invitation.invitation_id > 0)
        );
        assert!(page.items.iter().all(|invitation| invitation.team_id > 0));
    }

    #[test]
    /// GIVEN a mock that always reports a next cursor
    /// WHEN taking two pages from the paginated call
    /// THEN both pages are fetched
    fn paginates_team_invitations() {
        let server = MockServer::shared();
        let client = client(server);

        let pages = client
            .paginate(list_team_invitations())
            .take(2)
            .collect::<Result<Vec<_>, _>>()
            .expect("team invitation pagination should succeed");

        assert_eq!(pages.len(), 2);
    }

    #[test]
    /// GIVEN a server that rejects the api key
    /// WHEN listing the team invitations
    /// THEN the api error reaches the caller instead of a transport error
    fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&list_team_invitations())
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::{client, client_preferring};
    use futures_util::{StreamExt, TryStreamExt};

    #[tokio::test]
    /// GIVEN a team id and filter parameters
    /// WHEN listing the team invitations
    /// THEN a page of team invitations is returned
    async fn lists_team_invitations() {
        let server = MockServer::shared();

        let page = client(server)
            .send(&list_team_invitations())
            .await
            .expect("team invitation listing should succeed");

        assert!(!page.items.is_empty());
        assert!(
            page.items
                .iter()
                .all(|invitation| invitation.invitation_id > 0)
        );
        assert!(page.items.iter().all(|invitation| invitation.team_id > 0));
    }

    #[tokio::test]
    /// GIVEN a mock that always reports a next cursor
    /// WHEN taking two pages from the paginated call
    /// THEN both pages are fetched
    async fn paginates_team_invitations() {
        let server = MockServer::shared();
        let client = client(server);

        let pages: Vec<_> = client
            .paginate(list_team_invitations())
            .take(2)
            .try_collect()
            .await
            .expect("team invitation pagination should succeed");

        assert_eq!(pages.len(), 2);
    }

    #[tokio::test]
    /// GIVEN a server that rejects the api key
    /// WHEN listing the team invitations
    /// THEN the api error reaches the caller instead of a transport error
    async fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&list_team_invitations())
            .await
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}
