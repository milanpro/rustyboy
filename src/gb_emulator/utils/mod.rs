pub trait U16Ext {
    fn lo(&self) -> u8;
    fn hi(&self) -> u8;
}

impl U16Ext for u16 {
    fn lo(&self) -> u8 {
        *self as u8
    }
    fn hi(&self) -> u8 {
        (*self >> 8) as u8
    }
}
