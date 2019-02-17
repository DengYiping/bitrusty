mod decodeutil;
mod metainfo;
mod hash;

use std::io;
use io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    // access the torrent file
    let f = File::open("test.torrent")?;
    let mut buf_reader = io::BufReader::new(f);
    let mut buf = Vec::new();
    buf_reader.read_to_end(&mut buf)?; // read the whole content into the memory
    let metainfo = metainfo::parse_metainfo(&buf)?;
    println!("metainfo = {:?}", metainfo);

    Ok(())
}
