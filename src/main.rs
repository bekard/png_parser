#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::io;
use std::io::Read;
use std::mem::size_of;
use std::path::Path;

use crate::chunk_entry::ChunkEntry;

pub mod chunk_entry;

const SIGNATURE: &[u8] = &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
const END_TYPE_CODE: &str = "IEND";

fn main() -> Result<(), io::Error> {
    //let img_path = Path::new("Z:/prog/prob/png_parser/res/red_pixel.png");
    // let img_path = Path::new("C:\\Users\\antar\\Downloads\\PNG_transparency_demonstration_1.png");
    let img_path = Path::new("C:\\Users\\antar\\Downloads\\duck.png");
    // println!("Image path: {}", img_path.canonicalize()?.display());

    let img = File::open(img_path)?;
    let mut reader = io::BufReader::new(img);

    let mut sign_buf: [u8; SIGNATURE.len()] = [0; SIGNATURE.len()];
    reader.read_exact(&mut sign_buf)?;

    if SIGNATURE.ne(&sign_buf) {
        panic!("Not a PNG file");
    }

    let mut continue_parsing = true;
    while continue_parsing {
        match ChunkEntry::read(&mut reader) {
            Ok(chunk) => {
                println!("{}\n", chunk);
                if chunk.type_code.eq(END_TYPE_CODE.as_bytes()) {
                    continue_parsing = false;
                }
            }
            Err(error) => match error.kind() {
                io::ErrorKind::UnexpectedEof => continue_parsing = false,
                _ => eprintln!("Unknown error: {error}"),
            },
        }
    }

    Ok(())
}
