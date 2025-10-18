use crate::Rogblob::RogBlob;
use crate::init::{self, GitRepo};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::io;
use std::io::Write;
use std::str::from_utf8;
use std::{fs, fs::File, io::Read};
pub trait rogObject {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, data: &[u8]);
    fn init(&mut self);
    fn fmt(&self) -> &str;
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
fn object_write(repo: &GitRepo, obj: &dyn rogObject) -> io::Result<String> {
    //serialize data
    let data = obj.serialize();
    // rebuild thr rog obj
    let header = format!("{} {}\0", obj.fmt(), data.len());
    let mut full = Vec::new();
    full.extend_from_slice(header.as_bytes());
    full.extend_from_slice(&data);

    //get hash
    let mut hasher = Sha1::new();
    hasher.update(&full);
    let sha = format!("{:x}", hasher.finalize());

    //build the path for the obj
    let dir = &sha[0..2];
    let file = &sha[2..];
    let path = repo
        .repo_file(&["objects", dir, file], true)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file path invalid"))?;

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let f = File::create(&path)?;
        let mut encoder = ZlibEncoder::new(f, Compression::default());
        encoder.write_all(&full)?;
        encoder.finish()?;
    }
    Ok(sha)
}
