//use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use crate::read;

use read::*;
use std::fmt;
use std::io;
use std::io::Read;

const MAX_DATA_LENGTH: u32 = 2u32.pow(31);

#[derive(Default, Debug)]
pub struct ChunkEntry {
    length: u32,
    pub type_code: [u8; 4],
    data: Vec<u8>,
    crc: u32,
}

impl fmt::Display for ChunkEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MAX_DATA_LENGTH: usize = 8;

        let type_code = String::from_utf8_lossy(&self.type_code);
        // let data_fmt = if self.data.len() < MAX_DATA_LENGTH {
        //     format!("Data {:?}", &self.data[..])
        // } else {
        //     let data = &self.data[..MAX_DATA_LENGTH];
        //     format!("First {} bytes of data: {:?}", data.len(), data)
        // };

        write!(
            f,
            "Chunk {}, Data Length: {} Bytes, CRC: {:#x}",
            type_code, self.length, self.crc
        )
    }
}

impl ChunkEntry {
    pub fn read<T: io::Read + io::Seek>(reader: &mut T) -> Result<ChunkEntry, io::Error> {
        let mut res: ChunkEntry = ChunkEntry::default();

        let length = read_u32_be(reader);

        if length > MAX_DATA_LENGTH {
            let error = io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "[!] Wrong size of the chunk data: {}, it can't be more than {}",
                    length, MAX_DATA_LENGTH
                ),
            );
            return Err(error);
        }

        res.length = length;
        reader.read_exact(res.type_code.as_mut_slice())?;

        // skeep data for now
        // res.data.resize(res.length as usize, 0);
        // reader.read_exact(res.data.as_mut_slice())?;

        reader.seek_relative(length as i64)?;

        res.crc = read_u32_be(reader);

        Ok(res)
    }

    pub fn foo(&self) {
        println!("foo!");
    }
}
