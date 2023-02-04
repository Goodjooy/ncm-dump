mod decoding;
pub mod file_meta;
pub mod output;
mod utils;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom, Write};

use base64::{engine::general_purpose, Engine};
use file_meta::MetaInfo;
use output::OutputFile;

use crate::{
    decoding::decrypt_aes128_ecb,
    utils::{mask::Mask, read_blob::ReadBlob, verify_ncm},
};

const AES_CORE_KEY: [u8; 16] = [
    0x68, 0x7A, 0x48, 0x52, 0x41, 0x6D, 0x73, 0x6F, 0x35, 0x6B, 0x49, 0x6E, 0x62, 0x61, 0x78, 0x57,
];
const AES_MODIFY_KEY: [u8; 16] = [
    0x23, 0x31, 0x34, 0x6C, 0x6A, 0x6B, 0x5F, 0x21, 0x5C, 0x5D, 0x26, 0x30, 0x55, 0x3C, 0x27, 0x28,
];

#[derive(Debug, thiserror::Error)]

pub enum Error {
    #[error("Io Error: {0}")]
    Io(#[from] io::Error),
    #[error("Base64 Decode Error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("Serde Json Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Not a Netease Cloud Music copyright file")]
    InvalidNcm,
}

type Result<T> = core::result::Result<T, self::Error>;

pub struct NcmInfo {
    pub meta: MetaInfo,
    pub cover: Option<Vec<u8>>,
}

/// ncm dump
///
pub fn ncm_dump<I, O>(mut input: I, output: &O) -> self::Result<NcmInfo>
where
    I: Read + Seek,
    O: OutputFile<Error = io::Error>,
{
    verify_ncm(&mut input)?;

    input.seek(SeekFrom::Current(2))?;

    let key_data = input.read_blob_map(|v| *v ^= 0x64)?;
    let de_key_data = decrypt_aes128_ecb(AES_CORE_KEY, &key_data)?;
    // 17 = len("neteasecloudmusic")
    let key_data = &de_key_data[17..];

    let modify_data = input.read_blob_map(|v| *v ^= 0x63)?;
    let base64 = general_purpose::STANDARD;
    let de_modify_data = base64.decode(&modify_data[22..])?;
    let modify_data = decrypt_aes128_ecb(AES_MODIFY_KEY, &de_modify_data)?;
    // 6 = len("music:")
    let modify_data = &modify_data[6..];
    let meta_info = serde_json::from_slice::<MetaInfo>(modify_data)?;

    // crc check
    input.seek(SeekFrom::Current(4))?;
    input.seek(SeekFrom::Current(5))?;

    let image_data = input.read_blob_or_empty()?;

    let mask = Mask::new(key_data);
    const N: usize = 0x8000;
    let mut buff = vec![0u8; N];
    let mut output = output.open_write(&meta_info.format)?;
    loop {
        match input.read_exact(&mut buff) {
            Ok(_) => (),
            Err(err) => match err.kind() {
                ErrorKind::UnexpectedEof => break,
                _ => Err(err)?,
            },
        }

        for (idx, v) in buff.iter_mut().enumerate() {
            *v ^= mask[idx]
        }
        output.write_all(&buff)?;
    }

    Ok(NcmInfo {
        meta: meta_info,
        cover: image_data,
    })
}
