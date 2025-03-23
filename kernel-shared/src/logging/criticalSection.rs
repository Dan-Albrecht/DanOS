use critical_section::RawRestoreState;

pub struct DanOSCriticalSection;
critical_section::set_impl!(DanOSCriticalSection);

unsafe impl critical_section::Impl for DanOSCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // We're currently running with interrupts disabled so nothing to do right now
    }

    unsafe fn release(_restore_state: RawRestoreState) {
    }
}
