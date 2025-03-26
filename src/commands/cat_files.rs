use crate::objects::{Kind, Object};
use anyhow::Context;
use std::io::Read;

pub(crate) fn invoke(pretty_print: bool, object_hash: &str) -> anyhow::Result<()> {
    anyhow::ensure!(pretty_print, "you must be using a pretty print format");

    let mut object = Object::read(object_hash).context("parse out blob object file")?;

    match object.kind {
        Kind::Blob => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let n =
                std::io::copy(&mut object.reader, &mut stdout).context("write .git blob file")?;
            anyhow::ensure!(
                n == object.expected_size,
                "expected to find blob size: {},actual: {n}",
                object.expected_size
            );
        }
        _ => anyhow::bail!("unknown object type {}", object.kind),
    }
    Ok(())
}
