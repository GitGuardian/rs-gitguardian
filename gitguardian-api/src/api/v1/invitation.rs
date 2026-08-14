use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use serde::Serialize;

use crate::{
    ApiCall, ApiConfig,
    error::{ApiError, BuildError},
    models::invitation::Invitation,
    util::{
        json::{expect_json, json_body},
        url::set_query,
    },
};

/// Query parameters for an invitation creation request
#[derive(Clone, Debug, Serialize)]
struct CreateInvitationQueryParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email: Option<bool>,
}

/// Body of an invitation creation request
#[derive(Clone, Debug, Serialize)]
struct CreateInvitationBody<'a> {
    email: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<&'a str>,
}

/// Create an invitation.
///
/// This endpoint allows you to send an invitation to a user.
///
/// If you are using a personal access token, you need to have an access level superior or
/// equal to `member`.
///
/// `POST /v1/invitations`, answering `201` Invitation sent, `400` Invalid data, `401`
/// Invalid API key, `403` Forbidden Call, `409` Data conflict or `503` API under
/// maintenance.
#[derive(Clone, Debug)]
pub struct CreateInvitation {
    pub email: String,
    pub access_level: Option<String>,
    /// Whether to send an email to the invitee with a link to accept the invitation.
    pub send_email: Option<bool>,
}

impl CreateInvitation {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            access_level: None,
            send_email: None,
        }
    }
}

impl ApiCall for CreateInvitation {
    type Output = Invitation;

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError> {
        let mut url = config.endpoint("invitations")?;
        set_query(
            &mut url,
            &CreateInvitationQueryParam {
                send_email: self.send_email,
            },
        )?;
        json_body(
            config.request(Method::POST, &url),
            &CreateInvitationBody {
                email: &self.email,
                access_level: self.access_level.as_deref(),
            },
        )
    }

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError> {
        expect_json(response, StatusCode::CREATED)
    }
}
