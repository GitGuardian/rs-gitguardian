use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use serde::Serialize;

use crate::{
    ApiCall, ApiConfig,
    error::{ApiError, BuildError},
    models::team::Team,
    util::json::{expect_json, json_body},
};

/// Body of a team creation request
#[derive(Clone, Debug, Serialize)]
struct CreateTeamBody<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

/// Create a team.
///
/// This endpoint allows you to create a team.
///
/// If you are using a personal access token, you need to have an access level superior or
/// equal to `manager`, and the member is automatically added to the created team with
/// permissions `can_manage` and `full_access`.
///
/// `POST /v1/teams`, answering `201` Team created, `400` Invalid data, `401` Invalid API
/// key, `403` Forbidden Call, `409` Data conflict or `503` API under maintenance.
#[derive(Clone, Debug)]
pub struct CreateTeam {
    pub name: String,
    /// Team description.
    pub description: Option<String>,
}

impl CreateTeam {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
        }
    }
}

impl ApiCall for CreateTeam {
    type Output = Team;

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError> {
        let url = config.endpoint("teams")?;
        json_body(
            config.request(Method::POST, &url),
            &CreateTeamBody {
                name: &self.name,
                description: self.description.as_deref(),
            },
        )
    }

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError> {
        expect_json(response, StatusCode::CREATED)
    }
}
