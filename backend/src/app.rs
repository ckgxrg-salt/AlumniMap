use actix_web::{middleware, web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

/// Errors that may happen in the App
pub enum AppError {
    /// App cannot connect to the database
    DbErr,
    /// App cannot listen on the specified port
    ListenErr,
    /// App encountered an error during runtime
    RuntimeErr,
}

/// Runs the backend app
///
/// # Errors
/// If the app fails to connect to the given database or cannot bind to the specified port, it will not run and exit immediately.
/// If the app encounters a runtime error, it will halt and return a [`AppError::RuntimeErr`].
pub async fn run(db_url: String) -> Result<(), AppError> {
    let conn = Database::connect(db_url)
        .await
        .map_err(|_| AppError::DbErr)?;
    let state = AppState { db: conn };
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
