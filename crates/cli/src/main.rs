use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "llmirc", about = "LLM-IR toolchain CLI (stub)")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Parse { file: String },
    Canon { file: String },
    Check { file: String },
    Run { file: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Parse { file } => {
            let src = std::fs::read_to_string(file)?;
            match llmir_reader::parse(&src) {
                Ok(node) => println!("parsed: {} bytes", llmir_canon::format(&node).len()),
                Err(e) => eprintln!("parse error: {e}"),
            }
        }
        Cmd::Canon { file } => {
            let src = std::fs::read_to_string(file)?;
            if let Ok(node) = llmir_reader::parse(&src) {
                println!("{}", llmir_canon::format(&node));
            } else {
                eprintln!("cannot canon: parse failed");
            }
        }
        Cmd::Check { file } => {
            let src = std::fs::read_to_string(file)?;
            if let Ok(node) = llmir_reader::parse(&src) {
                let _ = llmir_schema::check_shapes(&node);
                let _ = llmir_types::typecheck(&node);
                println!("ok");
            } else {
                eprintln!("cannot check: parse failed");
            }
        }
        Cmd::Run { file } => {
            let src = std::fs::read_to_string(file)?;
            if let Ok(node) = llmir_reader::parse(&src) {
                let code = llmir_lower::lower(&node).unwrap_or_default();
                let rc = llmir_vm::run(&code);
                println!("exit {}", rc);
            } else {
                eprintln!("cannot run: parse failed");
            }
        }
    }
    Ok(())
}
