
#[repr(C, packed)]
pub struct PciHeader{
    VendorID: u16,
    DeviceID: u16,
    Command: u16,
    pub Status: u16,
    RevisionID: u8,
    ProgIF: u8, // Programming Interface Byte
    Subclass: u8,
    pub ClassCode: u8,
    CacheLineSize: u8,
    LatenchTimer: u8,
    pub HeaderType: u8,
    BIST: u8, // Built In Self Test
}