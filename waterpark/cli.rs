pub const USAGE: &'static str = "
Waterpark File share platform.

Usage:
  waterpark
  waterpark init
  waterpark config <path>
  waterpark help
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
  pub cmd_init: bool,
  pub cmd_config: bool,
  pub cmd_help: bool,
  pub arg_path: Vec<String>,
}