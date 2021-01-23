use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use toml::from_str;
use serde::Deserialize;
use flexi_logger::{Duplicate, Logger};
use app_dirs2::{app_dir, AppDataType, AppInfo};
use log::{error, debug};
use warp::Filter;
use std::net::{SocketAddr, IpAddr};

#[derive(Deserialize)]
struct Config {
    ip: Option<String>,
    port: Option<u16>,
    tmdb: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "< ip:{:#?}, port:{:#?}, tmdb:{} >", self.ip, self.port, self.tmdb)
    }
}

pub const APP_INFO: AppInfo = AppInfo {
    name: "vennactors",
    author: "devyntk",
};

#[tokio::main]
async fn main() {
    let log_dir =
        app_dir(AppDataType::UserConfig, &APP_INFO, "log/").expect("Error getting log directory");

    Logger::with_env_or_str("info")
        .log_to_file()
        .directory(log_dir)
        .duplicate_to_stdout(Duplicate::Debug)
        .start()
        .unwrap();

    let path = Path::new("./vennactors.toml");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            error!("couldn't open config file {}: {}", display, why);
            panic!()
        },
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            error!("couldn't read config file {}: {}", display, why);
            panic!()
        },
        Ok(_) => (),
    }

    let config: Config = match from_str(&*s.into_boxed_str()) {
        Ok(conf) => (conf),
        Err(err) => {
            error!("Couldn't read config file: {}", err);
            panic!()
        }
    };
    debug!("Config: {}", config);

    let routes = warp::any().map(|| "Hello, World!");

    let ip = config.ip.unwrap_or("127.0.0.1".into());
    let port = config.port.unwrap_or("8080".parse().unwrap());

    let addr = SocketAddr::new(IpAddr::V4(ip.parse().unwrap()), port);

    warp::serve(routes).run(addr).await;
}
