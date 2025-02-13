use std::env;
use std::process;

use minigrep::{Config,run};
fn main() {
    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("异常: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}