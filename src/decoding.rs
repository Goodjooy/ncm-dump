use std::io::{Cursor, Read, self};

use aes::{cipher::{generic_array::GenericArray, KeyInit, BlockDecrypt}, Aes128};

const BLOCK_SIZE: usize = 16;

pub(crate) fn pkcs7_un_padding(src: &mut Vec<u8>) {
    let unpadding = src.iter().next_back().copied().unwrap_or_default() & 0x0F;
    let _ = src
        .drain((src.len() - unpadding as usize)..)
        .collect::<Vec<_>>();
}

pub(crate) fn decrypt_aes128_ecb(key: [u8; 16], data: &[u8]) -> io::Result<Vec<u8>> {
    let key = GenericArray::from(key);
    let chipper = Aes128::new(&key);

    let mut buff = [0u8; BLOCK_SIZE];
    let mut cursor = Cursor::new(data);
    let mut ret = Vec::with_capacity(BLOCK_SIZE * BLOCK_SIZE);
    let times = data.len() / BLOCK_SIZE;
    for _ in 0..times {
        cursor.read_exact(&mut buff)?;
        let mut block = GenericArray::from(buff);
        chipper.decrypt_block(&mut block);

        ret.extend(block)
    }
    pkcs7_un_padding(&mut ret);
    Ok(ret)
}
