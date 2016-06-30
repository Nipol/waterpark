extern crate docopt;
extern crate rustc_serialize;

#[macro_use]
mod cli;
mod configuration;

use cli::show_version;
use configuration::Configuration;

fn main() {
  let conf = Configuration::parse();
  excute(conf);
}

fn excute(conf: Configuration) {
  if conf.args.cmd_version {
    show_version();
  }
}