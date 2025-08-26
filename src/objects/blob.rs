use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::io::Result;
use std::io::Write;

pub fn compute_sha1(data: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn compress_zlib(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

pub fn hash_to_hex(hash: &[u8; 20]) -> String {
    hash.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

pub fn create_blob(content: &[u8]) -> Result<String> {
    let header = format!("blob {}\0", content.len());
    let data = [header.as_bytes(), content].concat();
    let hash = compute_sha1(&data);
    let compressed_data = compress_zlib(&data)?;
    let hash_str = hash_to_hex(&hash);

    // Create the directory structure
    let dir_path = format!(".xit/objects/{}", &hash_str[0..2]);
    std::fs::create_dir_all(&dir_path)?;

    let path = format!("{}/{}", dir_path, &hash_str[2..]);
    std::fs::write(path, compressed_data)?;
    Ok(hash_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_blob() {
        let content = b"hello world";
        let hash = create_blob(content).unwrap();
        assert_eq!(hash, "95d09f2b10159347eece71399a7e2e907ea3df4f");

        // Clean up created files
        let dir_path = format!(".xit/objects/{}", &hash[0..2]);
        fs::remove_dir_all(dir_path).unwrap();
    }
}