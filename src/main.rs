use std::ffi::CStr;
use anyhow::Context;
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::{fs, io};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use flate2::Compression;
use sha1::{Digest, Sha1};

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
                .parse::<u64>()
                .context("Could not convert size to usize.")?;
            let mut z = z.take(size);
            match kind {
                Kind::Blob => {
                    let stdout = std::io::stdout();
                    let mut stdout = stdout.lock();
                    let n = std::io::copy(&mut z, &mut stdout).context("write .git blob file")?;
                    anyhow::ensure!(n == size, "expected to find blob size: {size},actual: {n}");
                }
            }
        }
        Command::HashObject { write, file, } => {
            fn write_blob<W>(file: &Path, writer: W) -> anyhow::Result<String> where W: Write {
                let stat = std::fs::metadata(file).context("read file metadata")?;
                let writer = ZlibEncoder::new(writer,Compression::default());
                let mut writer = HashWriter {
                    writer,
                    hasher: Sha1::new(),
                };
                write!(writer,"blob ");
                write!(writer,"{}\0",stat.len());
                let mut file = fs::File::open(file).context("read file")?;
                std::io::copy(&mut file, &mut writer).context("write file")?;
                let _ = writer.writer.finish()?;
                let hash = writer.hasher.finalize();
                Ok(hex::encode(hash))
            }
            let hash = if write {
                let tmp = "temporary";
                let hash = write_blob(&file,std::fs::File::create(tmp).context("construct temporary file for blob")?).context("construct temporary file for blob")?;
                fs::create_dir_all(format!(".git/objects/{}/",&hash[..2])).context("creat sub dir of .git")?;
                fs::rename(tmp, format!(".git/objects/{}/{}",&hash[..2],&hash[2..])).context("rename sub dir of .git")?;
                hash
            }else {
                write_blob(&file, std::io::sink()).context("write out file")?;
                "you must be using a '-w' format".parse()?
            };
            println!("{hash}")
        }
    }
    Ok(())
}

struct HashWriter<W> {
    writer: W,
    hasher: Sha1,
}

impl<W> Write for HashWriter<W> where W: Write{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
