use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use serde::Serialize;

use crate::{
    ApiCall, ApiConfig, Paginated,
    error::{ApiError, BuildError},
    models::{
        page::Page,
        pagination::{Cursor, Pagination},
        team::{
            incident_permission::IncidentPermission, membership::TeamMembership,
            permission::TeamPermission,
        },
    },
    util::{json::expect_json_page, url::set_query},
};

/// Query parameters of a team membership listing request
#[derive(Clone, Debug, Serialize)]
struct ListTeamMembershipsQueryParam<'a> {
    #[serde(flatten)]
    page: &'a Pagination,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_team_leader: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_permission: Option<TeamPermission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    incident_permission: Option<IncidentPermission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_id: Option<u32>,
}

/// List team memberships.
///
/// List all the memberships of a team.
///
/// If you are using a personal access token, you need to be a workspace manager or be part
/// of the team.
///
/// `GET /v1/teams/{team_id}/team_memberships`, answering `200` Team membership list, `400`
/// Invalid data, `401` Invalid API key, `403` Permission denied, `404` Not found or `503`
/// API under maintenance.
#[derive(Clone, Debug)]
pub struct ListTeamMemberships {
    /// The id of the team.
    pub team_id: u32,
    pub page: Pagination,
    /// Filter team memberships that are team leaders.
    pub is_team_leader: Option<bool>,
    /// Filter team memberships with a specific team permission.
    ///
    /// `team_permission` is replaced by `is_team_leader`.
    pub team_permission: Option<TeamPermission>,
    /// Filter team memberships with a specific incident permission.
    pub incident_permission: Option<IncidentPermission>,
    /// Filter team memberships on a specific member.
    pub member_id: Option<u32>,
}

impl ListTeamMemberships {
    pub fn new(team_id: u32) -> Self {
        Self {
            team_id,
            page: Pagination::default(),
            is_team_leader: None,
            team_permission: None,
            incident_permission: None,
            member_id: None,
        }
    }
}

impl ApiCall for ListTeamMemberships {
    type Output = Page<TeamMembership>;

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError> {
        let mut url = config.endpoint(&format!("teams/{}/team_memberships", self.team_id))?;
        set_query(
            &mut url,
            &ListTeamMembershipsQueryParam {
                page: &self.page,
                is_team_leader: self.is_team_leader,
                team_permission: self.team_permission,
                incident_permission: self.incident_permission,
                member_id: self.member_id,
            },
        )?;
        Ok(config.request(Method::GET, &url).body(Bytes::new())?)
    }

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError> {
        expect_json_page(response, StatusCode::OK)
    }
}

impl Paginated for ListTeamMemberships {
    fn set_cursor(&mut self, cursor: Cursor) {
        self.page.cursor = Some(cursor);
    }
}
