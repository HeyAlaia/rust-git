use anyhow::Context;
use clap::{Parser};
use flate2::write::ZlibEncoder;
use std::{fs};
use std::io::{ Write};
use std::path::{Path, PathBuf};
use flate2::Compression;
use sha1::{Digest, Sha1};

pub(crate) fn invoke(write: bool, file: &PathBuf) -> anyhow::Result<()>{
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
    println!("{hash}");
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