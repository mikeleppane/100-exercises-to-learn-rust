#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    fn new(value: u16) -> Self {
        Self { value }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self::new(value as u16)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self::new(*value)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self::new(*value as u16)
    }
}

impl std::ops::Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.value.saturating_add(rhs.value);
        Self::new(sum)
    }
}

impl std::ops::Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        let sum = self.value.saturating_add(rhs);
        Self::new(sum)
    }
}

impl std::ops::Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &u16) -> Self::Output {
        let sum = self.value.saturating_add(*rhs);
        Self::new(sum)
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
