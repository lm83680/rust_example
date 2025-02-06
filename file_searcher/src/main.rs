use ansi_term::Colour;
use clap::Parser;
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::path::Path; // 引入并行处理

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about("简单的读取命令行参数程序"), long_about = None)]
struct Args {
    /// 要搜索的关键词
    #[arg(index = 1)]
    search_term: String,

    /// 要搜索的目录
    #[arg(index = 2, default_value = "./")]
    directory: String,

    /// 是否递归搜索子目录
    #[arg(short, long, default_value = "false")]
    recursive: bool,

    ///是否忽略大小写
    #[arg(short, long, default_value = "false")]
    ignore_case: bool,
}

// 在文件中搜索匹配正则表达式的行
fn search_in_file(path: &Path, regex: &Regex) {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(content) => {
            // 按行遍历文件内容
            for line in content.lines() {
                // 如果当前行匹配正则表达式，则输出该行
                if regex.is_match(line) {
                    // 使用 ANSI 转义码高亮显示匹配的部分
                    let highlighted_line = regex.replace_all(line, |caps: &regex::Captures| {
                        // 高亮显示匹配的部分
                        Colour::Green
                            .bold()
                            .paint(caps.get(0).unwrap().as_str())
                            .to_string()
                    });

                    let path = Colour::Blue.paint(path.display().to_string());

                    // 打印匹配的文件路径和高亮显示的行
                    println!("{}: {}", path, highlighted_line);
                }
            }
        }
        Err(e) => {
            println!("读取文件时发生错误\n{}", e);
        }
    }
}

// 在目录中搜索文件，递归搜索子目录（如果需要）
fn search_directory(dir: &Path, regex: &Regex, recursive: bool) {
    // 如果给定路径是一个目录
    if dir.is_dir() {
        // 读取目录中的所有条目（文件和子目录）
        let entries = fs::read_dir(dir).unwrap();
        // 使用并行迭代（`par_bridge` 是 `rayon` 库提供的并行处理） || 闭包符  |e|{} 类似于 (e){}
        entries.par_bridge().for_each(|entry| {
            // 获取条目的路径
            let entry = entry.unwrap();
            let path = entry.path();

            // 如果当前条目是目录且需要递归，继续递归搜索该目录
            if path.is_dir() && recursive {
                search_directory(&path, regex, recursive);
            // 如果当前条目是文件，搜索该文件
            } else if path.is_file() {
                search_in_file(&path, regex);
            }
        });
    }
}

/// cargo run -- "hello" ./example_dir -r -i
fn main() {
    // 使用 `Args::parse()` 从命令行解析参数
    let args: Args = Args::parse();

    // 获取搜索的关键词、目录路径、忽略大小写的标志和是否递归搜索的标志
    let search_term: String = args.search_term;
    let directory: String = args.directory;
    let ignore_case: bool = args.ignore_case;
    let recursive: bool = args.recursive;

    // 根据是否忽略大小写，创建对应的正则表达式  .unwrap()-结构返回值，直接获取值而忽略err
    let regex = if ignore_case {
        // 如果忽略大小写，构造一个大小写不敏感的正则表达式
        Regex::new(&format!(r"(?i){}", search_term)).unwrap()
    } else {
        // 否则使用普通的正则表达式
        Regex::new(&search_term).unwrap()
    };

    // 将目录路径转换为 `Path` 类型
    let path: &Path = Path::new(&directory);
    // 调用 `search_directory` 函数开始搜索
    search_directory(path, &regex, recursive);
}
