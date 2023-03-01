use std::{future::pending, net::SocketAddr};

use tokio::signal::ctrl_c;

mod config;
mod routes;

pub async fn run() {
    /* initialize tracing */
    tracing_subscriber::fmt::init();

    /* build app with routes */
    let app = routes::_create_routes();

    /*
     * run app with hyper
     * `axum::Server` is a re-export of `hyper::Server`
     */
    let add = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", add);
    axum::Server::bind(&add)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async { ctrl_c().await.expect("failed to install Ctrl+C handler") };

    #[cfg(unix)]
    let terminate = {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }

    println!("signal received, starting graceful shutdown")
}
