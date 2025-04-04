use anyhow::Context;
use clap::{Parser, Subcommand};
use sha1::Digest;
use std::fs;
use std::io::{BufRead, Read, Write};
use std::path::PathBuf;

mod commands;
mod objects;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Doc comment
#[derive(Subcommand, Debug)]
enum Command {
    /// Doc comment
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        object_hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,
        file: PathBuf,
    },
    LsTree {
        #[clap(long)]
        name_only: bool,
        tree_hash: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Successfully init .git file");
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => commands::cat_files::invoke(pretty_print, &object_hash)?,
        Command::HashObject { write, file } => commands::hash_object::invoke(write, &file)?,
        Command::LsTree { name_only ,tree_hash} => commands::ls_tree::invoke(name_only, &tree_hash)?,
    }
    Ok(())
}
