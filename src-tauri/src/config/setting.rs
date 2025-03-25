use std::env;

use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

 


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct User {
    pub download_path: String,
    pub user_no: String,
    pub pwd: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub(crate) struct Settings {
    pub user: User,
}


impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "app".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("../config/app"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("/config/{run_mode}"))
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("/config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            .set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}