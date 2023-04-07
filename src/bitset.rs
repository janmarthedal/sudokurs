use std::fmt;

#[derive(Clone, Copy)]
pub struct BitSet {
    bits: usize,
}

impl BitSet {
    pub fn new() -> Self {
        Self { bits: 0 }
    }
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }
    pub fn count(&self) -> usize {
        self.bits.count_ones() as usize
    }
    pub fn contains(&self, value: usize) -> bool {
        self.bits & 1 << value != 0
    }
    pub fn insert(&mut self, value: usize) {
        self.bits |= 1 << value;
    }
    pub fn remove(&mut self, value: usize) {
        self.bits &= !(1 << value);
    }
    pub fn intersection(&self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }
}

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;
        f.write_char('{')?;
        let mut first = true;
        for i in 0..usize::BITS {
            if self.contains(i as usize) {
                if first {
                    first = false;
                } else {
                    f.write_char(',')?;
                }
                write!(f, "{}", i)?;
            }
        }
        f.write_char('}')?;
        Ok(())
    }
}
