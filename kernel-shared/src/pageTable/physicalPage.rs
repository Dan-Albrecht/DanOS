
#[repr(C, packed)]
pub struct PhysicalPage {
    Bytes: [u8; 4096],
}
