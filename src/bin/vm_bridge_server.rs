extern crate actix_web;
extern crate anyhow;
extern crate url;
extern crate vm_bridge;

use actix_web::http::StatusCode;
use actix_web::rt::blocking::BlockingError;
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use std::future::Ready;
use thiserror::Error;
use url::Url;
use vm_bridge::config::Config;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;
    println!("Config loaded.");
    println!("Host ip: {}", config.host_ip);
    println!("VM ip: {}", config.vm_ip);

    run_webserver(config)?;

    Ok(())
}

#[actix_web::main]
async fn run_webserver(config: Config) -> anyhow::Result<()> {
    HttpServer::new(|| App::new().service(open_url_service))
        .bind((config.host_ip, config.port))?
        .run()
        .await?;
    Ok(())
}

#[derive(Debug, Error)]
enum OpenUrlError {
    // #[error({self:})]
    #[error("{self:?}")]
    UrlParse(#[from] url::ParseError),
    // #[error({self})]
    #[error("{self:?}")]
    OpenUrl(std::io::Error),
    #[error("Internal server failure")]
    Internal,
}

impl ResponseError for OpenUrlError {
    fn status_code(&self) -> StatusCode {
        match self {
            OpenUrlError::UrlParse(_) => StatusCode::BAD_REQUEST,
            OpenUrlError::OpenUrl(_) => StatusCode::INTERNAL_SERVER_ERROR,
            OpenUrlError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

struct Okay;

impl Responder for Okay {
    type Error = actix_web::Error;
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        std::future::ready(Ok(HttpResponse::Ok().finish()))
    }
}

#[post("/open_url")]
async fn open_url_service(url: web::Json<vm_bridge::OpenUrl>) -> Result<Okay, OpenUrlError> {
    let url = Url::parse(&url.url)?;
    web::block(move || vm_bridge::open_url(url))
        .await
        .map_err(|blocking_error| match blocking_error {
            BlockingError::Error(e) => OpenUrlError::OpenUrl(e),
            BlockingError::Canceled => OpenUrlError::Internal,
        })?;
    // .map_err(|e| HttpResponse::InternalServerError().finish())?;

    // HttpResponse::Ok().finish()
    Ok(Okay)
}
