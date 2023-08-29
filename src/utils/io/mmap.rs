// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::io;

use memmap2::MmapAsRawDesc;
use positioned_io::{ReadAt, Size};

/// Wrapper type of [`memmap2::Mmap`] that implements [`ReadAt`] and [`Size`]
pub struct Mmap(memmap2::Mmap);

impl Mmap {
    pub fn map(file: impl MmapAsRawDesc) -> io::Result<Self> {
        Ok(Self(unsafe { memmap2::Mmap::map(file)? }))
    }
}

impl ReadAt for Mmap {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        let start = pos as usize;
        let end = start + buf.len();
        if end <= self.0.len() {
            buf.copy_from_slice(&self.0[start..end]);
            Ok(buf.len())
        } else {
            let len = self.0.len() - start;
            buf[..len].copy_from_slice(&self.0[start..]);
            Ok(len)
        }
    }
}

impl Size for Mmap {
    fn size(&self) -> io::Result<Option<u64>> {
        Ok(Some(self.0.len() as _))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use anyhow::Result;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_mmap_read_at_and_size(bytes: Vec<u8>) -> Result<()> {
        let tmp = tempfile::Builder::new().tempfile()?.into_temp_path();
        fs::write(&tmp, &bytes)?;
        let mmap = Mmap::map(&fs::File::open(&tmp)?)?;

        assert_eq!(mmap.size()?.unwrap_or_default() as usize, bytes.len());

        let mut buffer = [0; 128];
        for pos in 0..bytes.len() {
            let size = mmap.read_at(pos as _, &mut buffer)?;
            assert_eq!(&bytes[pos..(pos + size)], &buffer[..size]);
        }

        Ok(())
    }
}