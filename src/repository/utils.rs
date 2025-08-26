use std::io;

pub fn hex_to_bytes(hex: &str) -> std::result::Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

pub fn read_object(hash: &str) -> io::Result<(String, Vec<u8>)> {
    use flate2::read::ZlibDecoder;
    use std::fs::File;
    use std::io::Read;

    let path = format!(".xit/objects/{}/{}", &hash[..2], &hash[2..]);
    let file = File::open(path)?;
    let mut decoder = ZlibDecoder::new(file);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;

    let null_byte_pos = buffer
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid object format"))?;

    let header = String::from_utf8_lossy(&buffer[..null_byte_pos]);
    let content = buffer[null_byte_pos + 1..].to_vec();

    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid object header",
        ));
    }

    Ok((parts[0].to_string(), content))
}