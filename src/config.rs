use std::fs::File;
use std::io;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

use crate::function_source::FunctionSources;
use crate::table_source::TableSources;
use crate::utils::prettify_error;

#[derive(Clone, Debug, Serialize)]
pub struct Config {
    pub watch: bool,
    pub pool_size: u32,
    pub keep_alive: usize,
    pub default_srid: Option<i32>,
    pub worker_processes: usize,
    pub listen_addresses: String,
    pub connection_string: String,
    pub table_sources: Option<TableSources>,
    pub function_sources: Option<FunctionSources>,
    pub ca_root_file: Option<String>,
    pub danger_accept_invalid_certs: bool,
}

#[derive(Deserialize)]
pub struct ConfigBuilder {
    pub watch: Option<bool>,
    pub pool_size: Option<u32>,
    pub keep_alive: Option<usize>,
    pub default_srid: Option<i32>,
    pub worker_processes: Option<usize>,
    pub listen_addresses: Option<String>,
    pub connection_string: String,
    pub table_sources: Option<TableSources>,
    pub function_sources: Option<FunctionSources>,
    pub ca_root_file: Option<String>,
    pub danger_accept_invalid_certs: Option<bool>,
}

impl ConfigBuilder {
    pub fn finalize(self) -> Config {
        Config {
            watch: self.watch.unwrap_or(false),
            pool_size: self.pool_size.unwrap_or(20),
            keep_alive: self.keep_alive.unwrap_or(75),
            default_srid: self.default_srid,
            worker_processes: self.worker_processes.unwrap_or_else(num_cpus::get),
            listen_addresses: self
                .listen_addresses
                .unwrap_or_else(|| "0.0.0.0:3000".to_owned()),
            connection_string: self.connection_string,
            table_sources: self.table_sources,
            function_sources: self.function_sources,
            ca_root_file: self.ca_root_file,
            danger_accept_invalid_certs: self.danger_accept_invalid_certs.unwrap_or(false),
        }
    }
}

pub fn read_config(file_name: &str) -> io::Result<Config> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config_builder: ConfigBuilder = serde_yaml::from_str(contents.as_str())
        .map_err(prettify_error("Can't read config file".to_owned()))?;

    Ok(config_builder.finalize())
}
