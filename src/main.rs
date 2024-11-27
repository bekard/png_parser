#![allow(unused_imports)]
#![allow(dead_code)]

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::mem::size_of;
use std::path::Path;

const SIGNATURE: &[u8] = &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
const END_TYPE_CODE: &str = "IEND";

#[derive(Default, Debug)]
struct Chunk {
    length: u32,
    type_code: [u8; 4],
    data: Vec<u8>,
    crc: u32,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MAX_DATA_LENGTH: usize = 8;

        let type_code = String::from_utf8_lossy(&self.type_code[..]);
        let data_fmt = if self.data.len() < MAX_DATA_LENGTH {
            format!("Data {:?}", &self.data[..])
        } else {
            let data = &self.data[..MAX_DATA_LENGTH];
            format!("First {} bytes of data: {:?}", data.len(), data)
            //&self.data[..MAX_DATA_LENGTH]
        };

        write!(
            f,
            "Chunk {}, Data Length: {} Bytes, CRC: {:#x}\n{}",
            type_code, self.length, self.crc, data_fmt
        )
    }
}

impl Chunk {
    fn read<T: io::Read + io::Seek>(reader: &mut T) -> Result<Chunk, io::Error> {
        println!("[d] Current offset: {:#x}", reader.stream_position()?);

        let mut res: Chunk = Chunk::default();

        res.length = reader.read_u32::<BigEndian>()?;
        reader.read_exact(res.type_code.as_mut_slice())?;

        res.data.resize(res.length as usize, 0);
        reader.read_exact(res.data.as_mut_slice())?;

        res.crc = reader.read_u32::<BigEndian>()?;

        Ok(res)
    }
}

fn main() -> Result<(), io::Error> {
    //let img_path = Path::new("Z:/prog/prob/png_parser/res/red_pixel.png");
    // let img_path = Path::new("C:\\Users\\antar\\Downloads\\PNG_transparency_demonstration_1.png");
    let img_path = Path::new("C:\\Users\\antar\\Downloads\\duck.png");
    println!("Image path: {}", img_path.canonicalize()?.display());

    let img = File::open(img_path)?;
    let mut reader = io::BufReader::new(img);

    let mut sign_buf: [u8; SIGNATURE.len()] = [0; SIGNATURE.len()];
    reader.read_exact(&mut sign_buf)?;

    if SIGNATURE.eq(&sign_buf) {
        println!("Signature found");
    } else {
        panic!("Not a PNG file");
    }

    let mut continue_parsing = true;
    while continue_parsing {
        match Chunk::read(&mut reader) {
            Ok(chunk) => {
                println!("{}", chunk);
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
