pub mod client;
pub mod server;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    Server,
    Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.subcmd {
        SubCommand::Server => {
            let server = server::Server::from_env();
            server.serve().await.expect("running server failed");
        }
        SubCommand::Client => {
            let client = client::E2EClient::new();
            let secret = client.get_secrets().await.expect("getting secret failed");
            tracing::info!("get secret response: {}", secret);
        }
    }
}
