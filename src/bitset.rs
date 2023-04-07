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
    pub fn count(&self) -> u32 {
        self.bits.count_ones()
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
    pub fn iter(&self) -> Iter {
        Iter {
            bits: self.bits,
            value: 0,
        }
    }
}

pub struct Iter {
    bits: usize,
    value: usize,
}

impl Iterator for Iter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            return None;
        }
        while self.bits & 1 == 0 {
            self.bits >>= 1;
            self.value += 1;
        }
        let value = self.value;
        self.bits >>= 1;
        self.value += 1;
        return Some(value);
    }
}

impl IntoIterator for &BitSet {
    type Item = usize;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;
        f.write_char('{')?;
        let mut first = true;
        for i in self {
            if first {
                first = false;
            } else {
                f.write_char(',')?;
            }
            write!(f, "{}", i)?;
        }
        f.write_char('}')?;
        Ok(())
    }
}
