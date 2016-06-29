use cli::{USAGE, Args};
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