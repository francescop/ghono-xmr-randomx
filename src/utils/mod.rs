use byteorder::WriteBytesExt;
use byteorder::{ByteOrder, LittleEndian};
use hex::FromHex;
use log::*;
use randomx_rs;

pub mod client;
pub mod config;
pub mod work;
pub mod worker;

/// Return the binary data represented by the hexadecimal string hexstr.
pub fn unhexlify(hexstr: &str) -> Result<[u8; 32], hex::FromHexError> {
    <[u8; 32]>::from_hex(hexstr)
}

pub fn pack_nonce(blob: &mut [u8], nonce_bytes: &[u8; 4]) {
    blob[39] = nonce_bytes[0];
    blob[40] = nonce_bytes[1];
    blob[41] = nonce_bytes[2];
    blob[42] = nonce_bytes[3];
}

#[test]
fn test_unhexlify() {
    let hexstr = "308c6f50a99d6854394ea0e471cbd5234a29554a86df1f6708a4cbe2093a4078".to_string();
    let bin_data: [u8; 32] = [
        48, 140, 111, 80, 169, 157, 104, 84, 57, 78, 160, 228, 113, 203, 213, 35, 74, 41, 85, 74,
        134, 223, 31, 103, 8, 164, 203, 226, 9, 58, 64, 120,
    ];

    assert_eq!(unhexlify(&hexstr).unwrap()[0..32], bin_data)
}

#[test]
fn test_pack2() {
    let blob = "0c0cbbd9dffa056ed9f488ea952afa3ff2663b1ec70a60baea5543c2bab0f25a9e830d2d40eb5f000000009caec86efcd1554b50015f58db69445c62381943a4385ce402ae15ded652657019".to_string();
    let nonce: u32 = 1;

    let mut blob_hash = <[u8; 76]>::from_hex(&blob).unwrap();

    let res_bin_final = [
        12, 12, 187, 217, 223, 250, 5, 110, 217, 244, 136, 234, 149, 42, 250, 63, 242, 102, 59, 30,
        199, 10, 96, 186, 234, 85, 67, 194, 186, 176, 242, 90, 158, 131, 13, 45, 64, 235, 95, 1, 0,
        0, 0, 156, 174, 200, 110, 252, 209, 85, 75, 80, 1, 95, 88, 219, 105, 68, 92, 98, 56, 25,
        67, 164, 56, 92, 228, 2, 174, 21, 222, 214, 82, 101, 112, 25,
    ];
    pack_nonce(&mut blob_hash, &nonce.to_le_bytes());
    assert_eq!(blob_hash, res_bin_final);

    for x in 0..2_000_0000u32 {
        pack_nonce(&mut blob_hash, &x.to_le_bytes());
    }
}

#[test]
fn test_pack1() {
    let blob = "0c0cbbd9dffa056ed9f488ea952afa3ff2663b1ec70a60baea5543c2bab0f25a9e830d2d40eb5f000000009caec86efcd1554b50015f58db69445c62381943a4385ce402ae15ded652657019".to_string();

    let target = "c5a70000".to_string();
    let height: u64 = 2182450;
    let seed = "308c6f50a99d6854394ea0e471cbd5234a29554a86df1f6708a4cbe2093a4078".to_string();
    let seed_hash = unhexlify(&seed);

    let b = hex::decode(&blob);
    let res: Vec<u8> = vec![
        12, 12, 187, 217, 223, 250, 5, 110, 217, 244, 136, 234, 149, 42, 250, 63, 242, 102, 59, 30,
        199, 10, 96, 186, 234, 85, 67, 194, 186, 176, 242, 90, 158, 131, 13, 45, 64, 235, 95, 0, 0,
        0, 0, 156, 174, 200, 110, 252, 209, 85, 75, 80, 1, 95, 88, 219, 105, 68, 92, 98, 56, 25,
        67, 164, 56, 92, 228, 2, 174, 21, 222, 214, 82, 101, 112, 25,
    ];
    assert_eq!(b.unwrap(), res);

    let b39 = vec![
        12, 12, 187, 217, 223, 250, 5, 110, 217, 244, 136, 234, 149, 42, 250, 63, 242, 102, 59, 30,
        199, 10, 96, 186, 234, 85, 67, 194, 186, 176, 242, 90, 158, 131, 13, 45, 64, 235, 95,
    ];

    assert_eq!(b39.len(), 39);
    assert_eq!(b39[0..39], res[0..39]);

    let bin_res = vec![
        12, 12, 187, 217, 223, 250, 5, 110, 217, 244, 136, 234, 149, 42, 250, 63, 242, 102, 59, 30,
        199, 10, 96, 186, 234, 85, 67, 194, 186, 176, 242, 90, 158, 131, 13, 45, 64, 235, 95, 0, 0,
        0, 0, 156, 174, 200, 110, 252, 209, 85, 75, 80, 1, 95, 88, 219, 105, 68, 92, 98, 56, 25,
        67, 164, 56, 92, 228, 2, 174, 21, 222, 214, 82, 101, 112, 25,
    ];

    let nonce: u32 = 1;
    let res_bin_nonce: [u8; 43] = [
        12, 12, 187, 217, 223, 250, 5, 110, 217, 244, 136, 234, 149, 42, 250, 63, 242, 102, 59, 30,
        199, 10, 96, 186, 234, 85, 67, 194, 186, 176, 242, 90, 158, 131, 13, 45, 64, 235, 95, 1, 0,
        0, 0,
    ];

    use byteorder::{LittleEndian, WriteBytesExt};
    let mut nonce_bin = vec![];
    nonce_bin.write_u32::<LittleEndian>(nonce).unwrap();

    assert_eq!(nonce_bin, vec![1, 0, 0, 0]);

    let res_bin_plus_nonce = [
        12, 12, 187, 217, 223, 250, 5, 110, 217, 244, 136, 234, 149, 42, 250, 63, 242, 102, 59, 30,
        199, 10, 96, 186, 234, 85, 67, 194, 186, 176, 242, 90, 158, 131, 13, 45, 64, 235, 95, 1, 0,
        0, 0,
    ];

    let bin_plus_nonce = [&b39[..], &nonce_bin[..]].concat();
    assert_eq!(bin_plus_nonce, res_bin_plus_nonce);

    let res_bin_final = [
        12, 12, 187, 217, 223, 250, 5, 110, 217, 244, 136, 234, 149, 42, 250, 63, 242, 102, 59, 30,
        199, 10, 96, 186, 234, 85, 67, 194, 186, 176, 242, 90, 158, 131, 13, 45, 64, 235, 95, 1, 0,
        0, 0, 156, 174, 200, 110, 252, 209, 85, 75, 80, 1, 95, 88, 219, 105, 68, 92, 98, 56, 25,
        67, 164, 56, 92, 228, 2, 174, 21, 222, 214, 82, 101, 112, 25,
    ];

    let bin_final = [&bin_plus_nonce[..], &bin_res[43..]].concat();
    assert_eq!(bin_final, res_bin_final);
}

#[test]
fn rx_calculate_hash() {
    env_logger::init();

    /* nonce ~50000
     * final hash: c404cf4211047a5148a76d1c106b81ec610af36682cd63c49b90f1ccf7940000
     */
    let blob = "0c0cbbd9dffa056ed9f488ea952afa3ff2663b1ec70a60baea5543c2bab0f25a9e830d2d40eb5f000000009caec86efcd1554b50015f58db69445c62381943a4385ce402ae15ded652657019".to_string();
    let target = "c5a70000".to_string();
    let height: u64 = 2182450;
    let seed = "308c6f50a99d6854394ea0e471cbd5234a29554a86df1f6708a4cbe2093a4078".to_string();
    let mut nonce: u32 = 0;

    let target_hash = <[u8; 4]>::from_hex(&target).expect("decode failed");
    let mut blob_hash = <[u8; 76]>::from_hex(&blob).unwrap();
    let seed_hash = <[u8; 32]>::from_hex(&seed).unwrap();

    let target_num: u32 = u32::from_le_bytes(target_hash);
    let target_64 = target_num as u64;

    let mut hash_count: u64 = 0;

    let rx_flags = randomx_rs::RandomXFlag::FLAG_HARD_AES
        | randomx_rs::RandomXFlag::FLAG_JIT
        | randomx_rs::RandomXFlag::FLAG_ARGON2_AVX2
        | randomx_rs::RandomXFlag::FLAG_LARGE_PAGES
        | randomx_rs::RandomXFlag::FLAG_FULL_MEM
        | randomx_rs::RandomXFlag::FLAG_ARGON2_SSSE3;

    let rx_cache = randomx_rs::RandomXCache::new(rx_flags, &[0u8; 32]).unwrap();
    let rx_dataset = randomx_rs::RandomXDataset::new(rx_flags, &rx_cache, 0).unwrap();
    let rx_vm = randomx_rs::RandomXVM::new(rx_flags, Some(&rx_cache), Some(&rx_dataset)).unwrap();

    let mut rx_hash = [0u8; 32];
    while nonce <= 1000 {
        pack_nonce(&mut blob_hash, &nonce.to_le_bytes());
        let rx_hash = rx_vm.calculate_hash(&mut blob_hash).expect("no data");
        nonce += 1;
    }
    assert_eq!(1, 1)
}
