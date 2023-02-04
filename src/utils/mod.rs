pub mod mask;
use std::io::Read;

use crate::Error;

pub mod read_blob;

const NCM_HEADER: [u8; 8] = [0x43, 0x54, 0x45, 0x4e, 0x46, 0x44, 0x41, 0x4d];
pub(crate) fn verify_ncm<I: Read>(input: &mut I) -> Result<(), Error> {
    let mut buf = [0u8; 8];
    input.read_exact(&mut buf)?;

    if buf != NCM_HEADER {
        Err(Error::InvalidNcm)?;
    }
    Ok(())
}
