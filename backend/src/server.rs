use actix_web::{middleware, web, App, HttpServer};
use sea_orm::DatabaseConnection;
use std::{error::Error, fmt::Display};

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

/// Errors that may happen in the App
#[derive(Debug)]
pub enum AppError {
    /// App cannot connect to the database
    DbErr,
    /// App cannot listen on the specified port
    ListenErr,
    /// App encountered an error during runtime
    RuntimeErr,
}
impl Error for AppError {}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

/// Runs the backend app
///
/// # Errors
/// If the app fails to connect to the given database or cannot bind to the specified port, it will not run and exit immediately.
/// If the app encounters a runtime error, it will halt and return a [`AppError::RuntimeErr`].
pub async fn run(db: DatabaseConnection) -> Result<(), AppError> {
    let state = AppState { db };
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(routes::setup)
    });
    server = server
        .bind(("127.0.0.1", 8080))
        .map_err(|_| AppError::ListenErr)?;
    server
        .run()
        .await
        .map(|()| Ok(()))
        .map_err(|_| AppError::RuntimeErr)?
}
