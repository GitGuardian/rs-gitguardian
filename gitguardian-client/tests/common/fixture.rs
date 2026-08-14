use gitguardian_api::api::v1::honeytoken::CreateHoneytoken;
use gitguardian_api::api::v1::invitation::CreateInvitation;
use gitguardian_api::api::v1::jwt::CreateJwt;
use gitguardian_api::api::v1::multiscan::MultiScan;
use gitguardian_api::api::v1::scan::Scan;
use gitguardian_api::api::v1::team::CreateTeam;
use gitguardian_api::models::document::Document;

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

pub fn create_jwt() -> CreateJwt {
    CreateJwt::new("https://api.hasmysecretleaked.com")
}

pub fn create_honeytoken() -> CreateHoneytoken {
    CreateHoneytoken::new("honeytoken A", "AWS")
}

pub fn create_team() -> CreateTeam {
    CreateTeam::new("feature team A")
}

pub fn create_invitation() -> CreateInvitation {
    CreateInvitation::new("someone@example.com")
}
