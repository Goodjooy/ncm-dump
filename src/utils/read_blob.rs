use std::io::{self, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use tap::Tap;

pub type Bytes = Vec<u8>;

impl<R:Read> ReadBlob for R  {
    
}

pub trait ReadBlob: Read {
    fn read_blob(&mut self) -> io::Result<Bytes> {
        let len = self.read_u32::<LittleEndian>()?;

        let mut buffer = vec![0u8; len as usize];
        self.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    fn read_blob_map<F: FnMut(&mut u8)>(&mut self, mapper: F) -> io::Result<Bytes> {
        Ok(self
            .read_blob()?
            .tap_mut(|blob| blob.iter_mut().for_each(mapper)))
    }

    fn read_blob_or_empty(&mut self) -> io::Result<Option<Bytes>> {
        self.read_blob()
            .map(|blob| if blob.is_empty() { None } else { Some(blob) })
    }
}

