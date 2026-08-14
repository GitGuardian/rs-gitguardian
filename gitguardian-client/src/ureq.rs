use bytes::Bytes;
use gitguardian_api::{ApiCall, ApiConfig};
use http::Response;

use crate::error::Error;

const BODY_LIMIT: u64 = 64 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct Client {
    agent: ::ureq::Agent,
    config: ApiConfig,
}

impl Client {
    pub fn new(config: ApiConfig) -> Self {
        Self::with_agent(config, default_agent())
    }

    pub fn with_agent(config: ApiConfig, agent: ::ureq::Agent) -> Self {
        Self { agent, config }
    }

    pub fn config(&self) -> &ApiConfig {
        &self.config
    }

    pub fn send<C: ApiCall>(&self, call: &C) -> Result<C::Output, Error> {
        let request = call.build(&self.config)?;
        let response = self.agent.run(request.map(|body| body.to_vec()))?;

        let (parts, mut body) = response.into_parts();
        let bytes = body.with_config().limit(BODY_LIMIT).read_to_vec()?;

        Ok(call.parse(Response::from_parts(parts, Bytes::from(bytes)))?)
    }
}

pub fn agent_config_builder() -> ::ureq::config::ConfigBuilder<::ureq::typestate::AgentScope> {
    ::ureq::Agent::config_builder().http_status_as_error(false)
}

pub fn default_agent() -> ::ureq::Agent {
    agent_config_builder().build().new_agent()
}
