use std::env;
use std::process; /* process::exit() */

use grepTest::Config;

// 每一个函数都有合理的返回值判断

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args)......
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("problem in parse args {}", err);
        process::exit(1);
    });

    println!("in file {}", config.file_name);

    if let Err(e) = grepTest::run(config) {
        println!("application error: {}", e);
        process::exit(1);
    }
}