use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use serde::Serialize;

use crate::{
    ApiCall, ApiConfig,
    error::{ApiError, BuildError},
    util::json::{expect_no_content, json_body},
};

/// Body of a team perimeter update request
#[derive(Clone, Debug, Serialize)]
struct UpdateTeamSourcesBody<'a> {
    sources_to_add: &'a [u32],
    sources_to_remove: &'a [u32],
}

/// Update a team perimeter.
///
/// This endpoint allows you to add and remove sources from the perimeter of a team.
///
/// If you are using a personal access token, you need to be a workspace manager.
///
/// `POST /v1/teams/{team_id}/sources`, answering `204` Team perimeter updated, `400`
/// Invalid data, `401` Invalid API key, `403` Permission denied, `404` Not found or `503`
/// API under maintenance.
#[derive(Clone, Debug)]
pub struct UpdateTeamSources {
    /// The id of the team.
    pub team_id: u32,
    /// Ids of sources to add to the perimeter.
    pub sources_to_add: Vec<u32>,
    /// Ids of sources to remove from the perimeter.
    pub sources_to_remove: Vec<u32>,
}

impl UpdateTeamSources {
    pub fn new(team_id: u32) -> Self {
        Self {
            team_id,
            sources_to_add: Vec::new(),
            sources_to_remove: Vec::new(),
        }
    }
}

impl ApiCall for UpdateTeamSources {
    type Output = ();

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError> {
        let url = config.endpoint(&format!("teams/{}/sources", self.team_id))?;
        json_body(
            config.request(Method::POST, &url),
            &UpdateTeamSourcesBody {
                sources_to_add: &self.sources_to_add,
                sources_to_remove: &self.sources_to_remove,
            },
        )
    }

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError> {
        expect_no_content(response, StatusCode::NO_CONTENT)
    }
}
