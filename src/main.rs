// 声明我们创建的模块，让 main.rs 可以访问它们
mod cli;
mod error;
mod md_to_typ;
mod typ_to_md;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 根据子命令执行不同逻辑
    match &cli.command {
        Commands::Md2typ(args) => {
            println!("-> Running md2typ command...");

            // 1. 读取输入文件内容
            let input_path = &args.input;
            if !input_path.exists() {
                // 使用 anyhow 创建一个错误
                anyhow::bail!("Input file does not exist: {}", input_path.display());
            }
            let markdown_content = fs::read_to_string(input_path)?;
            println!("   - Successfully read input file: {}", input_path.display());

            // 2. 决定输出文件路径
            let output_path = args.output.clone().unwrap_or_else(|| {
                // 如果用户没有提供 -o, 则自动生成输出路径
                // 例如 a/b/c.md -> a/b/c.typ
                input_path.with_extension("typ")
            });
            println!("   - Output path will be: {}", output_path.display());

            // 3. (下一步实现) 调用核心转换逻辑
            // let typst_content = markst::md_to_typ::convert(&markdown_content)?;
            let typst_content = "/* Conversion logic not yet implemented */";
            
            // 4. 将转换后的内容写入文件
            fs::write(&output_path, typst_content)?;
            println!("✅ Successfully converted to {}", output_path.display());
        }
    }

    Ok(())
}