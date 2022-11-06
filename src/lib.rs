/*
 *                        _oo0oo_
 *                       o8888888o
 *                       88" . "88
 *                       (| -_- |)
 *                       0\  =  /0
 *                     ___/`---'\___
 *                   .' \\|     |// '.
 *                  / \\|||  :  |||// \
 *                 / _||||| -:- |||||- \
 *                |   | \\\  - /// |   |
 *                | \_|  ''\---/''  |_/ |
 *                \  .-\__  '-'  ___/-. /
 *              ___'. .'  /--.--\  `. .'___
 *           ."" '<  `.___\_<|>_/___.' >' "".
 *          | | :  `- \`.;`\ _ /`;.`/ - ` : | |
 *          \  \ `_.   \_ __\ /__ _/   .-` /  /
 *      =====`-.____`.___ \_____/___.-`___.-'=====
 *                        `=---='
 *
 *
 *      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 *            佛祖保佑     永不宕机     永无BUG
 *
 *        佛曰:
 *                写字楼里写字间，写字间里程序员；
 *                程序人员写程序，又拿程序换酒钱。
 *                酒醒只在网上坐，酒醉还来网下眠；
 *                酒醉酒醒日复日，网上网下年复年。
 *                但愿老死电脑间，不愿鞠躬老板前；
 *                奔驰宝马贵者趣，公交自行程序员。
 *                别人笑我忒疯癫，我笑自己命太贱；
 *                不见满街漂亮妹，哪个归得程序员？
 */

use std::{env, fs};
use std::env::Args;
use std::error::Error;

pub struct Config {
    pub keyword: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();
        let keyword = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a keyword")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(
            Config {
                keyword,
                filename,
                case_sensitive,
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.keyword, &contents)
    } else {
        search_case_insensitive(&config.keyword, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}


pub fn search<'a>(keyword: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(keyword) {
    //         results.push(line);
    //     }
    // }
    //
    // results

    // 采用迭代器实现
    contents.lines()
        .filter(|line| line.contains(keyword))
        .collect()
}

pub fn search_case_insensitive<'a>(keyword: &str, contents: &'a str) -> Vec<&'a str> {
    let lower_keyword = keyword.to_lowercase();
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&lower_keyword) {
    //         results.push(line);
    //     }
    // }
    //
    // results

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&lower_keyword))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let keyword = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(keyword, contents))
    }

    #[test]
    fn case_insensitive() {
        let keyword = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";

        assert_eq!(vec!["Rust:", "Trust me"], search_case_insensitive(keyword, contents))
    }
}
