use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use serde::Serialize;

use crate::{
    ApiCall, ApiConfig, Paginated,
    error::{ApiError, BuildError},
    models::{
        member::{Member, access_level::AccessLevel, ordering::MemberOrdering},
        page::Page,
        pagination::{Cursor, Pagination},
    },
    util::{
        json::{expect_json, expect_json_page},
        url::set_query,
    },
};

/// Query parameters of a member listing request
#[derive(Clone, Debug, Serialize)]
struct ListMembersQueryParam<'a> {
    #[serde(flatten)]
    page: &'a Pagination,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<AccessLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<AccessLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordering: Option<MemberOrdering>,
}

/// List members.
///
/// List members of the workspace.
///
/// `GET /v1/members`, answering `200` List members, `400` Invalid data, `401` Invalid API
/// key or `503` API under maintenance.
#[derive(Clone, Debug, Default)]
pub struct ListMembers {
    pub page: Pagination,
    /// Filter members based on their access level. Use `access_level` instead.
    pub role: Option<AccessLevel>,
    /// Filter members based on their access level.
    pub access_level: Option<AccessLevel>,
    /// Filter members based on their active status.
    pub active: Option<bool>,
    /// Search members based on their name or email.
    pub search: Option<String>,
    /// Sort the results by their field value. The default sort is ASC, DESC if the field
    /// is preceded by a `-`.
    pub ordering: Option<MemberOrdering>,
}

impl ApiCall for ListMembers {
    type Output = Page<Member>;

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError> {
        let mut url = config.endpoint("members")?;
        set_query(
            &mut url,
            &ListMembersQueryParam {
                page: &self.page,
                role: self.role,
                access_level: self.access_level,
                active: self.active,
                search: self.search.as_deref(),
                ordering: self.ordering,
            },
        )?;
        Ok(config.request(Method::GET, &url).body(Bytes::new())?)
    }

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError> {
        expect_json_page(response, StatusCode::OK)
    }
}

impl Paginated for ListMembers {
    fn set_cursor(&mut self, cursor: Cursor) {
        self.page.cursor = Some(cursor);
    }
}

/// Retrieve a member.
///
/// Retrieve an existing workspace member.
///
/// If you are using a personal access token, you need to have an access level greater or
/// equal to `member`.
///
/// `GET /v1/members/{member_id}`, answering `200` Workspace member details, `400` Invalid
/// data, `401` Invalid API key, `403` Forbidden Call, `404` Member not found or `503` API
/// under maintenance.
#[derive(Clone, Debug)]
pub struct RetrieveMember {
    /// The id of the workspace member.
    pub member_id: u32,
}

impl RetrieveMember {
    pub fn new(member_id: u32) -> Self {
        Self { member_id }
    }
}

impl ApiCall for RetrieveMember {
    type Output = Member;

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError> {
        let url = config.endpoint(&format!("members/{}", self.member_id))?;
        Ok(config.request(Method::GET, &url).body(Bytes::new())?)
    }

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError> {
        expect_json(response, StatusCode::OK)
    }
}
