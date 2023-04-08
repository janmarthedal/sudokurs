use std::fmt;

pub trait BitStore:
    Copy
    + Clone
    + Sized
    + PartialEq
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitOr<Output = Self>
    + std::ops::Shl<usize, Output = Self>
    + std::ops::BitAndAssign
    + std::ops::BitOrAssign
    + std::ops::Not<Output = Self>
    + std::ops::ShrAssign<usize>
{
    // + std::ops::BitXor<Output = Self> + std::ops::Shr<usize, Output = Self> + std::ops::BitXorAssign + std::ops::ShlAssign<usize>
    fn count_ones(&self) -> u32;
    fn zero() -> Self;
    fn one() -> Self;
}

#[derive(Clone, Copy)]
pub struct BitSet<T: BitStore> {
    bits: T,
}

impl BitStore for usize {
    fn count_ones(&self) -> u32 {
        (*self as usize).count_ones()
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl<T: BitStore> BitSet<T> {
    pub fn new() -> Self {
        Self { bits: T::zero() }
    }
    pub fn count(&self) -> usize {
        self.bits.count_ones() as usize
    }
    pub fn contains(&self, value: usize) -> bool {
        self.bits & (T::one() << value) != T::zero()
    }
    pub fn insert(&mut self, value: usize) {
        self.bits |= T::one() << value;
    }
    pub fn remove(&mut self, value: usize) {
        self.bits &= !(T::one() << value);
    }
    pub fn intersection(&self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }
    pub fn iter(&self) -> Iter<T> {
        Iter {
            bits: self.bits,
            value: 0,
        }
    }
}

pub struct Iter<T: BitStore> {
    bits: T,
    value: usize,
}

impl<T: BitStore> Iterator for Iter<T> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == T::zero() {
            return None;
        }
        while self.bits & T::one() == T::zero() {
            self.bits >>= 1;
            self.value += 1;
        }
        let value = self.value;
        self.bits >>= 1;
        self.value += 1;
        return Some(value);
    }
}

impl<T: BitStore> IntoIterator for &BitSet<T> {
    type Item = usize;
    type IntoIter = Iter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: BitStore> fmt::Debug for BitSet<T> {
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
