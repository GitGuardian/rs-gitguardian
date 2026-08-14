use gitguardian_client::ureq::{Client, agent_config_builder};
use gitguardian_mock::{MockServer, prefer};

pub fn agent_preferring(status: u16) -> ::ureq::Agent {
    let value = ::ureq::http::HeaderValue::from_str(&prefer::code(status)).unwrap();
    agent_config_builder()
        .middleware(
            move |mut request: ::ureq::http::Request<::ureq::SendBody>,
                  next: ::ureq::middleware::MiddlewareNext| {
                request.headers_mut().insert(prefer::HEADER, value.clone());
                next.handle(request)
            },
        )
        .build()
        .new_agent()
}

pub fn client(server: &MockServer) -> Client {
    Client::new(super::config(server))
}

pub fn client_preferring(server: &MockServer, status: u16) -> Client {
    Client::with_agent(super::config(server), agent_preferring(status))
}
