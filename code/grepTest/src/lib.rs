use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    query: String, /* 拷贝消耗 */
    pub file_name: String,
    case_sensitive : bool,
}

impl Config {
    /* new函数的返回值比较复杂 */

    /* 这个不能new函数重载嘛？ */
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough args");
    //     }
    //     let _query = args[1].clone();
    //     let file_name = args[2].clone();

    //     Ok(Config {_query, file_name})
    // }
    
    // mut 保证了 next()
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // 迭代器
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("not a query"),
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("not a file_name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

/* trait对象 */
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let context = fs::read_to_string(config.file_name)?; // 避免了unwrap()
    for line in search(&config.query, &context) {
        println!("{}", line);
    }
    Ok(())
}

/* 当入参是&的时候 */
pub fn search<'a>(query: &'a str, context: &'a str) -> Vec<& 'a str> {
    context
        .lines()
        .filter(|line| line.contains(query))
        .collect()

    // 以上可以替换 // 使用迭代器更有效
    // for-each line in context, if line contains query, then collect into the new Vec 
}

#[cfg(test)]
mod tests {
    use super::*;

    /* 这个测试仅针对内部的功能函数 */
    #[test]
    fn one_result() {
        let query = "duct";
        let context = "\
rust:
safe fast productive
pick three";
        assert_eq!(vec!["safe fast productive"], search(query, context));
    }
}