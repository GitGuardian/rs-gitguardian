use gitguardian_api::api::v1::health::CheckHealth;
use gitguardian_api::api::v1::honeytoken::CreateHoneytoken;
use gitguardian_api::api::v1::honeytoken_with_context::CreateHoneytokenWithContext;
use gitguardian_api::api::v1::invitation::CreateInvitation;
use gitguardian_api::api::v1::invitation::ListInvitations;
use gitguardian_api::api::v1::jwt::CreateJwt;
use gitguardian_api::api::v1::member::{ListMembers, RetrieveMember};
use gitguardian_api::api::v1::multiscan::MultiScan;
use gitguardian_api::api::v1::quota::RetrieveQuotas;
use gitguardian_api::api::v1::scan::Scan;
use gitguardian_api::api::v1::scan_create_incidents::ScanCreateIncidents;
use gitguardian_api::api::v1::team::{CreateTeam, ListTeams, RetrieveTeam};
use gitguardian_api::api::v1::team_invitation::{CreateTeamInvitation, ListTeamInvitations};
use gitguardian_api::api::v1::team_membership::{CreateTeamMembership, ListTeamMemberships};
use gitguardian_api::api::v1::team_source::UpdateTeamSources;
use gitguardian_api::models::document::Document;
use gitguardian_api::models::document_location::DocumentLocation;
use gitguardian_api::models::honeytoken::r#type::HoneytokenType;
use gitguardian_api::models::invitation::ordering::InvitationOrdering;
use gitguardian_api::models::member::access_level::AccessLevel;
use gitguardian_api::models::member::ordering::MemberOrdering;
use gitguardian_api::models::pagination::Pagination;
use gitguardian_api::models::team::incident_permission::IncidentPermission;
use gitguardian_api::models::team::permission::TeamPermission;
use gitguardian_api::uuid::Uuid;

pub const SOURCE_UUID: Uuid = Uuid::from_u128(0x550e8400_e29b_41d4_a716_446655440000);

pub const TEAM_ID: u32 = 3252;

pub const MEMBER_ID: u32 = 3252;

pub const INVITATION_ID: u32 = 4851;

pub const SOURCE_ID: u32 = 6531;

pub fn document() -> Document {
    Document::new("aws_key = AKIA123").with_filename("intro.py")
}

pub fn documents() -> Vec<Document> {
    vec![
        document(),
        Document::new("__version__ = \"1.0.0\"").with_filename("tasks.py"),
    ]
}

pub fn scan() -> Scan {
    Scan::new(document())
}

pub fn multiscan() -> MultiScan {
    MultiScan::new(documents())
}

pub fn scan_create_incidents() -> ScanCreateIncidents {
    ScanCreateIncidents::new(
        SOURCE_UUID,
        vec![document().with_location(DocumentLocation::new(
            "https://wiki.example.com/my-config-page",
        ))],
    )
}

pub fn create_honeytoken_with_context() -> CreateHoneytokenWithContext {
    let mut call = CreateHoneytokenWithContext::new("honeytoken A", HoneytokenType::Aws);
    call.language = Some("python".to_owned());
    call.project_extensions = vec![".py".to_owned(), ".c".to_owned()];
    call
}

pub fn create_jwt() -> CreateJwt {
    CreateJwt::new("https://api.hasmysecretleaked.com")
}

pub fn create_honeytoken() -> CreateHoneytoken {
    CreateHoneytoken::new("honeytoken A", HoneytokenType::Aws)
}

pub fn create_team() -> CreateTeam {
    CreateTeam::new("feature team A")
}

pub fn list_teams() -> ListTeams {
    ListTeams {
        page: Pagination {
            per_page: Some(1),
            ..Default::default()
        },
        is_global: Some(false),
        ..Default::default()
    }
}

pub fn retrieve_team() -> RetrieveTeam {
    RetrieveTeam::new(TEAM_ID)
}

pub fn list_invitations() -> ListInvitations {
    ListInvitations {
        page: Pagination {
            per_page: Some(1),
            ..Default::default()
        },
        ordering: Some(InvitationOrdering::DateDesc),
        ..Default::default()
    }
}

pub fn list_members() -> ListMembers {
    ListMembers {
        page: Pagination {
            per_page: Some(1),
            ..Default::default()
        },
        access_level: Some(AccessLevel::Manager),
        active: Some(true),
        ordering: Some(MemberOrdering::CreatedAtDesc),
        ..Default::default()
    }
}

pub fn retrieve_member() -> RetrieveMember {
    RetrieveMember::new(MEMBER_ID)
}

pub fn list_team_invitations() -> ListTeamInvitations {
    let mut call = ListTeamInvitations::new(TEAM_ID);
    call.page.per_page = Some(1);
    call.incident_permission = Some(IncidentPermission::FullAccess);
    call
}

pub fn list_team_memberships() -> ListTeamMemberships {
    let mut call = ListTeamMemberships::new(TEAM_ID);
    call.page.per_page = Some(1);
    call.team_permission = Some(TeamPermission::CanManage);
    call
}

pub fn create_invitation() -> CreateInvitation {
    CreateInvitation::new("someone@example.com")
}

pub fn create_team_invitation() -> CreateTeamInvitation {
    let mut call = CreateTeamInvitation::new(TEAM_ID, INVITATION_ID);
    call.is_team_leader = Some(false);
    call.incident_permission = Some(IncidentPermission::CanEdit);
    call
}

pub fn create_team_membership() -> CreateTeamMembership {
    let mut call = CreateTeamMembership::new(TEAM_ID, MEMBER_ID);
    call.is_team_leader = Some(false);
    call.incident_permission = Some(IncidentPermission::CanEdit);
    call.send_email = Some(false);
    call
}

pub fn update_team_sources() -> UpdateTeamSources {
    let mut call = UpdateTeamSources::new(TEAM_ID);
    call.sources_to_add = vec![SOURCE_ID];
    call
}

pub fn check_health() -> CheckHealth {
    CheckHealth
}

pub fn retrieve_quotas() -> RetrieveQuotas {
    RetrieveQuotas
}
