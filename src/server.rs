use std::{net::SocketAddr, sync::Arc};

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use clique_sibyl_commonlib::{
    attestation::{verify_attestation, Attestation},
    tls::config::create_tls_server_config,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Server {
    pub secret: String,
    pub port: u16,
}

impl Server {
    pub fn from_env() -> Self {
        Self {
            secret: std::env::var("SECRET").unwrap_or("default_secret".to_string()),
            port: std::env::var("PORT")
                .unwrap_or("7878".to_string())
                .parse()
                .expect("failed to parse port"),
        }
    }

    pub async fn serve(&self) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/secret", post(get_secrets))
            .with_state(self.clone());

        let addr = SocketAddr::new([0, 0, 0, 0].into(), self.port);
        let tls_config = create_tls_server_config()?;
        let rustls_config = RustlsConfig::from_config(Arc::new(tls_config));

        tracing::info!("listening on {}", addr);
        axum_server::bind_rustls(addr, rustls_config)
            .serve(app.into_make_service())
            .await?;
        Ok(())
    }
}

async fn get_secrets(
    State(server): State<Server>,
    Json(req): Json<GetSecretReq>,
) -> (StatusCode, String) {
    tracing::info!("received request: {:?}", req);
    let GetSecretReq { attestation, .. } = req;
    if verify_attestation(&attestation, None, None).is_ok() {
        (StatusCode::OK, server.secret.clone())
    } else {
        (StatusCode::FORBIDDEN, "verification failed".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecretReq {
    attestation: Attestation,
    signature: String,
}

impl GetSecretReq {
    pub fn new(attestation: Attestation, signature: String) -> Self {
        Self {
            attestation,
            signature,
        }
    }
}
