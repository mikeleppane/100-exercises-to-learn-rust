#![allow(dead_code)]

pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

fn example() {
    let _: WrappingU32 = 42.into();
    let _ = WrappingU32::from(42);
}
