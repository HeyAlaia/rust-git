use anyhow::Context;
use clap::Parser;
use flate2::read::ZlibDecoder;
use sha1::Digest;
use std::ffi::CStr;
use std::fmt;
use std::io::{BufRead, BufReader, Read};

#[derive(PartialEq, Debug, Eq)]
pub(crate) enum Kind {
    Blob,
    Tree,
    Commit,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Blob => write!(f, "blob"),
            Kind::Tree => write!(f, "tree"),
            Kind::Commit => write!(f, "commit"),
        }
    }
}

pub(crate) struct Object<R> {
    pub(crate) kind: Kind,
    pub(crate) expected_size: u64,
    pub(crate) reader: R,
}

impl Object<()> {
    pub(crate) fn read(hash: &str) -> anyhow::Result<Object<impl BufRead>> {
        let f = std::fs::File::open(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
            .context("Could not open object file")?;
        let z = ZlibDecoder::new(f);
        let mut z = BufReader::new(z);
        let mut buf = Vec::new();
        z.read_until(0, &mut buf)
            .context("Could not read object file")?;
        if std::str::from_utf8(&buf).is_err() {
            println!("buf 不是有效的 UTF-8 数据");
        }
        let header = CStr::from_bytes_with_nul(&buf).expect("know there is exactly one nul.");
        let header = header
            .to_str()
            .context("Could not convert header to string.")?;
        let Some((kind, size)) = header.split_once(" ") else {
            anyhow::bail!("Could not find blob prefix.")
        };
        let kind = match kind {
            "blob" => Kind::Blob,
            "tree" => Kind::Tree,
            "commit" => Kind::Commit,
            _ => anyhow::bail!("Unknown {} type.", kind),
        };
        let size = size
            .parse::<u64>()
            .context("Could not convert size to usize.")?;
        let mut z = z.take(size);
        Ok(Object {
            kind,
            expected_size: size,
            reader: z,
        })
    }
}
