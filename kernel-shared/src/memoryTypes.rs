use core::marker::PhantomData;

pub struct VirtualAddressPlain {
    pub address: usize,
}

pub struct VirtualAddress<T> {
    pub address: usize,
    _x: PhantomData<T>,
}

pub struct PhysicalAddressPlain {
    pub address: usize,
}

pub struct PhysicalAddress<T> {
    pub address: usize,
    _x: PhantomData<T>,
}

pub struct MemoryAddress {
    pub r#virtual: VirtualAddressPlain,
    pub physical: PhysicalAddressPlain,
}

pub struct SomeSortOfIndex {
    pub value: u8,
}

impl<T> PhysicalAddress<T> {
    pub fn new(address: usize) -> Self {
        PhysicalAddress {
            address,
            _x: PhantomData,
        }
    }

    pub fn is_null(&self) -> bool {
        self.address == 0
    }

    // BUGBUG: Unsafe as this will only work if identity mapped and we don't know
    pub fn unsafePtr(&self) -> *mut T {
        self.address as *mut T
    }
}

impl VirtualAddressPlain {
    pub fn derp<T>(&self) -> VirtualAddress<T> {
        VirtualAddress {
            address: self.address,
            _x: PhantomData,
        }
    }
}

impl<T> VirtualAddress<T> {
    pub fn new(address: usize) -> Self {
        VirtualAddress {
            address,
            _x: PhantomData,
        }
    }

    pub fn ptr(&self) -> *mut T{
        self.address as *mut T
    }

    pub fn is_null(&self) -> bool {
        self.address == 0
    }
}
