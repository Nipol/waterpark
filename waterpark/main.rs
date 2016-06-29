extern crate docopt;
extern crate rustc_serialize;

#[macro_use]
mod cli;
mod configuration;

use configuration::Configuration;

fn main() {
  let conf = Configuration::parse();
  excute(conf);
}

fn excute(conf: Configuration) {
  println!("conf Success!");
}