extern crate crypto;

use crypto::digest::Digest;

pub fn calculate_sha1_bytes(input: &[u8]) -> Vec<u8> {
    let mut hasher = crypto::sha1::Sha1::new();
    hasher.input(input);
    let mut result = vec![0; hasher.output_bytes()];
    hasher.result(&mut result);
    result
}