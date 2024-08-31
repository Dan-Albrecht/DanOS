
#[repr(C, packed)]
pub struct PhysicalPage {
    // BUGBUG: Should this be SIZE_OF_PAGE?
    // No, kill this whole type and just use the constant
    Bytes: [u8; 4096],
}
