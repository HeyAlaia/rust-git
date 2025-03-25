use std::ffi::CStr;
use anyhow::Context;
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};

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
}

enum Kind {
    Blob,
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
        } => {
            anyhow::ensure!(pretty_print, "you must be using a pretty print format");
            let f = std::fs::File::open(format!(
                ".git/objects/{}/{}",
                &object_hash[..2],
                &object_hash[2..]
            ))
            .context("Could not open object file")?;
            let z = ZlibDecoder::new(f);
            let mut z = BufReader::new(z);
            let mut buf = Vec::new();
            z.read_until(0, &mut buf)
                .context("Could not read object file")?;
            if std::str::from_utf8(&buf).is_err() {
                println!("buf 不是有效的 UTF-8 数据");
            }
            let header =
                CStr::from_bytes_with_nul(&buf).expect("know there is exactly one nul.");
            let header = header
                .to_str()
                .context("Could not convert header to string.")?;
            let Some((kind, size)) = header.split_once(" ") else {
                anyhow::bail!("Could not find blob prefix.")
            };
            let kind = match kind {
                "blob" => Kind::Blob,
                _ => anyhow::bail!("Unknown blob type."),
            };
            let size = size
                .parse::<usize>()
                .context("Could not convert size to usize.")?;
            buf.clear();
            buf.resize(size, 0);
            z.read_exact(&mut buf[..])
                .context("Could not read object file")?;
            let n = z.read(&mut [0]).context("validate file")?;
            anyhow::ensure!(n == 0, "Need at least one object. had {n} bytes");
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            match kind {
                Kind::Blob => {
                    stdout
                        .write_all(&buf)
                        .context("Could not write to stdout.")?;
                }
            }
        }
    }

    Ok(())
}
