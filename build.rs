// build.rs

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
}

fn setup_database_url() -> Result<(), Box<dyn std::error::Error>> {
    // Чтение APP_ENV, по умолчанию "local"
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| {
        println!("APP_ENV is not set, using the default value 'local'");
        "local".to_string()
    });
    // Построение пути к config/{app_env}.toml
    let config_path = Path::new("config").join(format!("{}.toml", app_env));

    // Чтение содержимого конфигурационного файла
    let config_content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Failed to read the configuration file: {:?}", config_path));

    // Парсинг TOML
    let config: Config = toml::from_str(&config_content)
        .unwrap_or_else(|_| panic!("Failed to parse the configuration file: {:?}", config_path));

    // Установка переменной окружения DATABASE_URL для макросов SQLx
    println!("cargo:rustc-env=DATABASE_URL={}", config.database.url);
    Ok(())
}
fn compile_protos() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from("src/gen");
    std::fs::create_dir_all(&out_dir)?;

    tonic_build::configure()
        .out_dir(&out_dir)
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/common.proto",
                "proto/instruments.proto",
                "proto/marketdata.proto",
                "proto/operations.proto",
                "proto/orders.proto",
                "proto/sandbox.proto",
                "proto/stoporders.proto",
                "proto/users.proto",
            ],
            &["proto"],
        )?;
    rename_file(&out_dir)?;
    Ok(())
}
fn rename_file(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let old_path = out_dir.join("tinkoff.public.invest.api.contract.v1.rs");
    let new_path = out_dir.join("tinkoff_public_invest_api_contract_v1.rs");
    if old_path.exists() {
        fs::rename(old_path, new_path)?;
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_database_url()?;
    compile_protos()?;
    Ok(())
}
