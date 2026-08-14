//! Tests of /v1/scan

mod common;

use common::assert_api_status;
use gitguardian_api::api::v1::scan::Scan;
use gitguardian_api::models::document::Document;
use gitguardian_mock::MockServer;
use http::StatusCode;

fn document() -> Document {
    Document::new("aws_key = AKIA123").with_filename("intro.py")
}

#[cfg(feature = "ureq")]
mod ureq {
    use super::*;
    use common::ureq::{client, client_preferring};

    #[test]
    /// GIVEN a document
    /// WHEN scanning it
    /// THEN a scan result is returned
    fn scans_document() {
        let server = MockServer::shared();

        let result = client(server)
            .send(&Scan::new(document()))
            .expect("scan should succeed");

        assert!(!result.policies.is_empty());
        assert_eq!(
            result.policy_break_count as usize,
            result.policy_breaks.len()
        );
    }

    #[test]
    /// GIVEN a server that rejects the api key
    /// WHEN scanning a document
    /// THEN the api error reaches the caller instead of a transport error
    fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&Scan::new(document()))
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use super::*;
    use common::reqwest::{client, client_preferring};

    #[tokio::test]
    /// GIVEN a document
    /// WHEN scanning it
    /// THEN a scan result is returned
    async fn scans_document() {
        let server = MockServer::shared();

        let result = client(server)
            .send(&Scan::new(document()))
            .await
            .expect("scan should succeed");

        assert!(!result.policies.is_empty());
        assert_eq!(
            result.policy_break_count as usize,
            result.policy_breaks.len()
        );
    }

    #[tokio::test]
    /// GIVEN a server that rejects the api key
    /// WHEN scanning a document
    /// THEN the api error reaches the caller instead of a transport error
    async fn surfaces_api_error() {
        let server = MockServer::shared();

        let error = client_preferring(server, 401)
            .send(&Scan::new(document()))
            .await
            .unwrap_err();

        assert_api_status(&error, StatusCode::UNAUTHORIZED, "Invalid API key.");
    }
}
