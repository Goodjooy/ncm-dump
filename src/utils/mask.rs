use std::{num::Wrapping, ops::Index};

pub struct Mask([u8; 256]);

impl Index<usize> for Mask {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        let idx = (index + 1) & 0xff;
        let boxing = &self.0;

        let j = self.0[idx] as usize;
        &boxing[(j + boxing[(j + idx) & 0xff] as usize) & 0xff]
    }
}

impl Mask {
    pub(crate) fn new(key: &[u8]) -> Self {
        let mut box_data = [0u8; 256];
        box_data
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = i as u8);

        let mut last_u8 = Wrapping(0);
        let mut key_iter = key.iter().copied().cycle();
        for idx in 0..256 {
            let key = Wrapping(key_iter.next().unwrap());
            let current = Wrapping(box_data[idx]);

            let c = (current + last_u8 + key).0 & 0xff;

            (box_data[idx], box_data[c as usize]) = (box_data[c as usize], box_data[idx]);
            last_u8 = Wrapping(c);
        }

        Self(box_data)
    }
}
