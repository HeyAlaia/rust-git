use anyhow::Context;
use clap::Parser;
use flate2::read::ZlibDecoder;
use sha1::Digest;
use std::ffi::CStr;
use std::io::{BufRead, BufReader, Read};

enum Kind {
    Blob,
}


pub(crate) fn invoke(pretty_print: bool, object_hash: &str) -> anyhow::Result<()> {
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
    Ok(())
}