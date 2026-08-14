use gitguardian_api::api::v1::honeytoken::CreateHoneytoken;
use gitguardian_api::api::v1::honeytoken_with_context::CreateHoneytokenWithContext;
use gitguardian_api::api::v1::invitation::CreateInvitation;
use gitguardian_api::api::v1::jwt::CreateJwt;
use gitguardian_api::api::v1::multiscan::MultiScan;
use gitguardian_api::api::v1::scan::Scan;
use gitguardian_api::api::v1::scan_create_incidents::ScanCreateIncidents;
use gitguardian_api::api::v1::team::CreateTeam;
use gitguardian_api::models::document::Document;
use gitguardian_api::models::document_location::DocumentLocation;
use gitguardian_api::models::honeytoken_type::HoneytokenType;
use gitguardian_api::uuid::Uuid;

pub const SOURCE_UUID: Uuid = Uuid::from_u128(0x550e8400_e29b_41d4_a716_446655440000);

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

pub fn create_invitation() -> CreateInvitation {
    CreateInvitation::new("someone@example.com")
}
