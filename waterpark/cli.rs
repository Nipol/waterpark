// The MIT License (MIT)
//
// Copyright (c) 2016 Nipol
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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