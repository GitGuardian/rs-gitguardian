//! Tests of /v1/api_tokens

mod common;

use common::fixture::{retrieve_api_token, retrieve_current_api_token};
use gitguardian_mock::MockServer;

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::client;

    #[test]
    /// GIVEN a valid api key
    /// WHEN retrieving the details of the current token
    /// THEN the token is returned
    fn retrieves_current_api_token() {
        let token = client(MockServer::shared())
            .send(&retrieve_current_api_token())
            .expect("current token retrieval should succeed");

        assert!(!token.name.is_empty());
    }

    #[test]
    /// GIVEN a token id
    /// WHEN retrieving the details of that token
    /// THEN the token is returned
    fn retrieves_api_token() {
        let token = client(MockServer::shared())
            .send(&retrieve_api_token())
            .expect("token retrieval should succeed");

        assert!(!token.name.is_empty());
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::client;

    #[tokio::test]
    /// GIVEN a valid api key
    /// WHEN retrieving the details of the current token
    /// THEN the token is returned
    async fn retrieves_current_api_token() {
        let token = client(MockServer::shared())
            .send(&retrieve_current_api_token())
            .await
            .expect("current token retrieval should succeed");

        assert!(!token.name.is_empty());
    }

    #[tokio::test]
    /// GIVEN a token id
    /// WHEN retrieving the details of that token
    /// THEN the token is returned
    async fn retrieves_api_token() {
        let token = client(MockServer::shared())
            .send(&retrieve_api_token())
            .await
            .expect("token retrieval should succeed");

        assert!(!token.name.is_empty());
    }
}
