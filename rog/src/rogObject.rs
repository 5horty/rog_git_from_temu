use crate::Rogblob::{self};
use crate::init::{self, GitRepo};
use flate2::{Compression, bufread::ZlibEncoder, read::ZlibDecoder};
use std::str::from_utf8;
use std::{fs, io::Read};
pub trait rogObject {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, data: &[u8]);
    fn init(&mut self);
}

fn object_read(repo: &GitRepo, sha: &str) -> Option<Box<dyn rogObject>> {
    let path = repo.repo_file(&["objects", &sha[0..2], &sha[2..]], false)?; // getting the dir 2 then file the last bytes
    let data = fs::read(path).ok()?;
    let mut raw = ZlibDecoder::new(&data[..]);
    let mut decompressed = Vec::new();
    raw.read_to_end(&mut decompressed).unwrap();
    let split = decompressed.iter().position(|b| *b == b' ').unwrap();
    let fmt = &decompressed[..split]; // gets cmd eg blob
    let rest = decompressed.iter().position(|b| *b == 0).unwrap();
    let size = from_utf8(&decompressed[split + 1..rest])
        .ok()?
        .parse::<usize>()
        .ok()?; // getting the rest
    if size != decompressed.len() - (rest - 1) {
        panic!("failed malformed rog object");
    }
    let content = &decompressed[rest + 1..];
    let obj: Box<dyn rogObject> = match fmt {
        b"blob" => Box::new(RogBlob::from_bytes(content)),
        b"commit" => Box::new(RogCommit::from_bytes(content)),
        b"tree" => Box::new(RogTree::from_bytes(content)),
        b"tag" => Box::new(RogTag::from_bytes(content)),
        _ => panic!("wrong cmd"),
    };
    Some(obj)
}
