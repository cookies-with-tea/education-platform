use std::env;
use std::net::IpAddr;
use crate::core::DatabaseConfig;

pub struct AppConfig {
    pub app_host: IpAddr,
    pub app_port: u16,
    pub db_config: DatabaseConfig,
}

impl AppConfig {
    pub fn new() -> Self {
        let app_host_str = env::var("APP_HOST").expect("APP_HOST must be set");
        let app_host = app_host_str.parse().expect("APP_HOST is not a valid IP address");
        let app_port_str = env::var("APP_PORT").expect("APP_PORT must be set");
        let app_port = app_port_str.parse().expect("APP_PORT is not a valid port number");

        let db_config = DatabaseConfig::new();

        AppConfig {
            app_host,
            app_port,
            db_config,
        }
    }
}