use cli::{USAGE, Args};
use rustc_serialize::json::{self, ToJson, Json};
use docopt::Docopt;

pub struct Configuration {
  pub args: Args
}

impl Configuration {
  pub fn parse() -> Self {
    Configuration {
      args: Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit()),
    }
  }
}

#[derive(RustcEncodable)]
pub struct WaterparkConfig {
  name: String,
  port: u16,
  tls: bool,
}

impl WaterparkConfig {
  pub fn init() -> Self {
    WaterparkConfig {
      name: "Waterpakr".to_string(),
      port: 80,
      tls: false,
    }
  }
}