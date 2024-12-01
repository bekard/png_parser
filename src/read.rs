use std::io::Read;

pub fn read_u32_be<Reader: Read>(reader: &mut Reader) -> u32 {
    let mut buf: [u8; 4] = [0; 4];
    reader.read_exact(&mut buf).unwrap();

    u32::from_be_bytes(buf[..4].try_into().unwrap())
}
