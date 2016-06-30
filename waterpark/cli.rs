pub const USAGE: &'static str = "
Waterpark : Simply File share platform.
  By Nipol

Usage:
  waterpark [Options]
  waterpark init [Options]
  waterpark help
  waterpark version

Default Options:
  -c --config PATH         서버 설정을 담은 config.json 파일 위치를 지정합니다.
                           [Default: ./config.json]

init Options:
  -n --name SERVERNAME     서버 이름을 지정합니다. 
                           [Default: waterpark].
  -p --port PORT           서버가 오픈될 포트를 지정합니다.
                           [Default: 80].
  --tls                    TODO
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
  pub cmd_init: bool,
  pub cmd_help: bool,
  pub cmd_version: bool,
  pub flag_config: Vec<String>,
  pub flag_name: String,
  pub flag_port: u16,
  pub flag_tls: bool,
}

pub fn show_version() {
  println!("0.0.1");
}