use kernel_shared::assemblyStuff::ports::outB;

const PIC1_DATA: u16 = 0x21;
const PIC2_DATA: u16 = 0xA1;

pub fn disablePic() {
    unsafe {
        outB(PIC1_DATA, 0xFF);
        outB(PIC2_DATA, 0xFF);
    }
}
