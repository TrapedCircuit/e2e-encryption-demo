use clique_sibyl_commonlib::{
    attestation::attest_with_signature, tls::config::create_tls_client_config,
};
use reqwest::ClientBuilder;

use crate::server::GetSecretReq;

#[derive(Debug, Clone)]
pub struct E2EClient {
    pub client: reqwest::Client,
}

impl E2EClient {
    const URL: &'static str = "http://localhost:7878/get_secret";

    pub fn new() -> Self {
        let tls_config = create_tls_client_config(None, None);
        let client = ClientBuilder::new()
            .use_preconfigured_tls(tls_config)
            .build()
            .expect("failed to build client");

        Self { client }
    }

    pub async fn get_secrets(&self) -> anyhow::Result<String> {
        let (attestation, signature) = attest_with_signature(b"placeholder")?;
        let req = GetSecretReq::new(attestation, signature);

        let res = self
            .client
            .post(Self::URL)
            .json(&req)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }
}
