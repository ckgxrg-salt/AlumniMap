use backend::app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = app::run("postgres://loco:loco@10.233.1.2:5432/alumnimap_dev".to_string()).await;
    Ok(())
}
