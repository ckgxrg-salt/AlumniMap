use clap::{Parser, Subcommand};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};
use std::io::Write;
use std::path::PathBuf;
use std::{error::Error, io, str::FromStr};

use backend::server;
use entity::{profile, university};
use migration::{Migrator, MigratorTrait};

mod settings;

#[derive(Parser)]
struct Cli {
    #[arg(
        short = 'c',
        long = "config",
        default_value = "/var/lib/alumnimap/config.toml",
        help = "Config file"
    )]
    config: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the server
    Server,
    /// Run database migration
    Migrate,
    /// List data from database
    List { kind: String },
    /// Interactively add entries in the terminal
    Add { kind: String },
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let settings = settings::Settings::new(&args.config);

    let db_uri = settings.general.database_uri;
    let assets_root = settings.general.assets_root;
    let base_point = settings.base.into();

    let addr = settings.general.addr;
    let port = settings.general.port;

    match args.command {
        Commands::Server => run_server(&db_uri, &assets_root, base_point, addr, port).await,
        Commands::Migrate => run_migration(&db_uri).await,
        Commands::List { kind } => list(&db_uri, &kind).await,
        Commands::Add { kind } => interactive_add(&db_uri, &kind).await,
    }?;

    Ok(())
}

/// Actually starts the server
async fn run_server(
    uri: &str,
    assets_root: &str,
    base_point: university::Model,
    addr: String,
    port: u16,
) -> Result<(), Box<dyn Error>> {
    let db = Database::connect(uri).await?;
    let pending = Migrator::get_pending_migrations(&db).await?;
    if !pending.is_empty() {
        println!("Running migration");
        Migrator::up(&db, None).await?;
        println!("Success");
    }
    println!("Running server");
    server::run(db, assets_root, base_point, addr, port).await?;
    Ok(())
}

/// Ensure the database is ready
async fn run_migration(uri: &str) -> Result<(), Box<dyn Error>> {
    let db = Database::connect(uri).await?;
    println!("Running migration");
    Migrator::up(&db, None).await?;
    println!("Success");
    Ok(())
}

/// See all entries
async fn list(uri: &str, kind: &str) -> Result<(), Box<dyn Error>> {
    let db = Database::connect(uri).await?;
    if kind.eq("university") {
        let list = university::Entity::find().all(&db).await?;
        for each in list {
            println!("{}: {}", each.id, each.title);
        }
        Ok(())
    } else if kind.eq("profile") {
        let list = profile::Entity::find().all(&db).await?;
        for each in list {
            let university = university::Entity::find_by_id(each.university_id)
                .one(&db)
                .await?
                .unwrap()
                .title;
            println!("{}: {}: {university}", each.id, each.name_primary);
        }
        Ok(())
    } else {
        eprintln!("Unknown kind");
        Err(Box::new(server::AppError::RuntimeErr))
    }
}
/// Add an entry in the terminal
async fn interactive_add(uri: &str, kind: &str) -> Result<(), Box<dyn Error>> {
    let db = Database::connect(uri).await?;
    if kind.eq("university") {
        println!("Creating new University");
        let title = ask_value("Title");
        let icon = ask_value("Icon");
        let colour = ask_value("Colour");
        let longitude = ask_value("Longitude");
        let latitude = ask_value("Latitude");
        let model = university::ActiveModel {
            title: Set(title),
            icon: Set(icon),
            colour: Set(colour),
            longitude: Set(longitude),
            latitude: Set(latitude),
            ..Default::default()
        };
        university::Entity::insert(model).exec(&db).await?;
        println!("Success");
        Ok(())
    } else if kind.eq("profile") {
        println!("Creating new Profile");
        let name_primary = ask_value("Primary Name");
        let name_supplementary = ask_value_nullable("Supplementary Name");
        let class_of = ask_value("Class of");
        let avatar = ask_value("Avatar");
        let university = university::Entity::find()
            .filter(university::Column::Title.contains(ask_value::<String>("University")))
            .one(&db)
            .await?
            .expect("No such University")
            .id;
        let major = ask_value_nullable("Major");
        let bio = ask_value_nullable("Bio");
        let email = ask_value_nullable("Email");
        let qq = ask_value_nullable("QQ");
        let wechat = ask_value_nullable("Wechat");
        let model = profile::ActiveModel {
            name_primary: Set(name_primary),
            name_supplementary: Set(name_supplementary),
            class_of: Set(class_of),
            avatar: Set(avatar),
            university_id: Set(university),
            major: Set(major),
            bio: Set(bio),
            email: Set(email),
            qq: Set(qq),
            wechat: Set(wechat),
            ..Default::default()
        };
        profile::Entity::insert(model).exec(&db).await?;
        println!("Success");
        Ok(())
    } else {
        eprintln!("Unknown kind");
        Err(Box::new(server::AppError::RuntimeErr))
    }
}

fn ask_value<T: FromStr>(name: &str) -> T {
    let mut value = String::new();
    loop {
        print!("{name}: ");
        io::stdout().flush().unwrap_or_default();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read input");

        let parsed = value.trim().parse::<T>();
        if let Ok(val) = parsed {
            break val;
        }
        eprintln!("Parse failed");
    }
}
fn ask_value_nullable(name: &str) -> Option<String> {
    let mut value = String::new();
    print!("{name}: ");
    io::stdout().flush().unwrap_or_default();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");

    let parsed = value.trim().parse::<String>().unwrap();
    if parsed.is_empty() {
        eprintln!("Parse failed, implying null");
        None
    } else {
        Some(parsed)
    }
}
