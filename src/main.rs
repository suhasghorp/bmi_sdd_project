use bmi_sdd::create_app;
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// T044: Clap CLI struct with port and address arguments
#[derive(Parser, Debug)]
#[command(name = "bmi_sdd")]
#[command(about = "BMI Calculator Web Service", long_about = None)]
struct Cli {
    /// Port to bind to (can also be set via PORT env var)
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Address to bind to
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,
}

#[tokio::main]
async fn main() {
    // T046: Setup tracing-subscriber with env_filter
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bmi_sdd=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // T045: PORT environment variable handling with precedence over CLI flag
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(cli.port);

    let bind_address = format!("{}:{}", cli.address, port);

    // T047: Structured logging for server startup
    tracing::info!(
        address = %bind_address,
        "Starting BMI Calculator server"
    );

    // Build application router with API endpoints
    let app = create_app();

    // Bind to configured address
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server ready and listening on http://{}", bind_address);

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    tracing::info!("Server shut down gracefully");
}

// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received CTRL+C signal, shutting down gracefully...");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM signal, shutting down gracefully...");
        },
    }
}
