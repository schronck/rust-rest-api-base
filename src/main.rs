#![deny(clippy::all)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::negative_feature_names)]
#![deny(unused_crate_dependencies)]

mod api;
mod config;
mod telemetry;

use actix_cors::Cors;
use actix_limitation::{Limiter, RateLimiter};
use actix_web::{dev::ServiceRequest, middleware::Compress, web::Data, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::{
    api::{routes, types::ApiError},
    config as conf,
    telemetry::*,
};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber(conf::subscriber(), conf::rust_log(), std::io::stdout);
    init_subscriber(subscriber);

    loop {
        if let Err(e) = try_main(&conf::ip(), &conf::port()).await {
            panic!("{e}");
        } else {
            std::process::exit(0);
        }
    }
}

async fn try_main(ip: &str, port: &str) -> Result<(), ApiError> {
    let port = port
        .parse::<u16>()
        .map_err(|e| ApiError::Other(e.to_string()))?;

    println!("Listening on http://{ip}:{port}");

    let limiter = Data::new(
        Limiter::builder(&conf::redis_url())
            .key_by(|req: &ServiceRequest| Some(req.peer_addr().unwrap().to_string()))
            .limit(5000)
            .period(std::time::Duration::from_secs(3600))
            .build()
            .unwrap(),
    );

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Compress::default())
            .wrap(Cors::permissive())
            .wrap(RateLimiter::default())
            .app_data(limiter.clone())
            .service(routes::numbers::add_two)
            .service(routes::numbers::sum)
    })
    .bind((ip, port))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use shiba as _;
}
