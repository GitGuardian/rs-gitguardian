//! Tests of /v1/multiscan

mod common;

use common::assert_api_status;
use common::fixture::multiscan;
use gitguardian_mock::MockServer;
use http::StatusCode;

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::{client, client_preferring};

    #[test]
    /// GIVEN several documents
    /// WHEN scanning them
    /// THEN one scan result is returned per document
    fn scans_documents() {
        let server = MockServer::shared();

        let results = client(server)
            .send(&multiscan())
            .expect("multiscan should succeed");

        assert!(!results.is_empty());
        for result in &results {
            assert!(!result.policies.is_empty());
            assert_eq!(
                result.policy_break_count as usize,
                result.policy_breaks.len()
            );
        }
    }

    #[test]
    /// GIVEN a server that rejects the api key
    /// WHEN scanning several documents
    /// THEN the api error reaches the caller instead of a transport error
    fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&multiscan())
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::{client, client_preferring};

    #[tokio::test]
    /// GIVEN several documents
    /// WHEN scanning them
    /// THEN one scan result is returned per document
    async fn scans_documents() {
        let server = MockServer::shared();

        let results = client(server)
            .send(&multiscan())
            .await
            .expect("multiscan should succeed");

        assert!(!results.is_empty());
        for result in &results {
            assert!(!result.policies.is_empty());
            assert_eq!(
                result.policy_break_count as usize,
                result.policy_breaks.len()
            );
        }
    }

    #[tokio::test]
    /// GIVEN a server that rejects the api key
    /// WHEN scanning several documents
    /// THEN the api error reaches the caller instead of a transport error
    async fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&multiscan())
            .await
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}
