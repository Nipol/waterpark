pub const USAGE: &'static str = "
Waterpark File share platform.

Usage:
  waterpark init [options]
  waterpark config <path> [options]

Options
  --blahblah      blahblah
  --blahblah2     blahblah2
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
  pub cmd_init: bool,
  pub arg_path: Vec<String>,
}