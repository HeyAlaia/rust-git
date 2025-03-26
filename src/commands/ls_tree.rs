use crate::objects::{Kind, Object};
use anyhow::Context;
use std::ffi::CStr;
use std::io::Write;
use std::io::{BufRead, Read};

pub(crate) fn invoke(name_only: bool, tree_hash: &str) -> anyhow::Result<()> {

    let mut object = Object::read(tree_hash).context("parse out blob object file")?;

    match object.kind {
        Kind::Tree => {
            let mut buf = Vec::new();
            let mut hashbuf = [0; 20];
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            loop {
                buf.clear();
                let mode_and_name = object.reader.read_until(0, &mut buf).context("read file")?;
                if mode_and_name == 0 {
                    break;
                }
                object
                    .reader
                    .read_exact(&mut hashbuf[..])
                    .context("read tree object hash")?;

                let mode_and_line = CStr::from_bytes_with_nul(&buf).context("parse path")?;
                let mut bits = mode_and_line.to_bytes().splitn(2, |&b| b == b' ');
                let mode = bits.next().expect("parse path");
                let name = bits
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("tree entry has no file name"))?;
                if name_only {
                    stdout.write_all(name).context("write tree entry name")?;
                } else {
                    let mode = std::str::from_utf8(mode).context("parse mode and valid utf-8")?;
                    let hash = hex::encode(&hashbuf);
                    let object = Object::read(&hash).context("read file type")?;
                    write!(stdout, " {mode:0>6} {} {hash} ", object.kind).context("write tree entry kind")?;
                    stdout.write_all(name).context("write tree entry name")?;
                }
                write!(stdout, "\n").context("write tree entry name")?;
            }
        }
        _ => anyhow::bail!("unknown object type {}", object.kind),
    }
    Ok(())
}