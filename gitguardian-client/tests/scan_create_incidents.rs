//! Tests of /v1/scan/create-incidents

mod common;

use common::assert_api_status;
use common::fixture::scan_create_incidents;
use gitguardian_mock::MockServer;
use http::StatusCode;

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::{client, client_preferring};

    #[test]
    /// GIVEN a document and a custom source
    /// WHEN scanning it to create incidents on that source
    /// THEN one scan result is returned per document
    fn scans_documents_and_creates_incidents() {
        let server = MockServer::shared();

        let results = client(server)
            .send(&scan_create_incidents())
            .expect("scan and incident creation should succeed");

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
    /// WHEN scanning a document to create incidents
    /// THEN the api error reaches the caller instead of a transport error
    fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&scan_create_incidents())
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::{client, client_preferring};

    #[tokio::test]
    /// GIVEN a document and a custom source
    /// WHEN scanning it to create incidents on that source
    /// THEN one scan result is returned per document
    async fn scans_documents_and_creates_incidents() {
        let server = MockServer::shared();

        let results = client(server)
            .send(&scan_create_incidents())
            .await
            .expect("scan and incident creation should succeed");

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
    /// WHEN scanning a document to create incidents
    /// THEN the api error reaches the caller instead of a transport error
    async fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&scan_create_incidents())
            .await
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}
