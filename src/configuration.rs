#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn fmt(x: &str, y: &str) -> String {
    format!("{}/{}", x, y)
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut s = config::Config::default();
    s.merge(config::File::with_name("configuration"))?;
    s.try_into()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
