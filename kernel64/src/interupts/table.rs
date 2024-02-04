use super::InteruptDescriptorTable::{ExceptionStackFrame, InterruptHandlerIntImpl, InterruptHandlerWithCodeIntImpl};

#[inline(never)]
#[no_mangle]
pub extern "x86-interrupt" fn Interrupt0(stackFrame: ExceptionStackFrame) {
    InterruptHandlerIntImpl(0, stackFrame);
}

#[inline(never)]
#[no_mangle]
pub extern "x86-interrupt" fn Interrupt3(stackFrame: ExceptionStackFrame) {
    InterruptHandlerIntImpl(3, stackFrame);
}

#[inline(never)]
#[no_mangle]
pub extern "x86-interrupt" fn Interrupt14(stackFrame: ExceptionStackFrame, errorCode: u64) {
    InterruptHandlerWithCodeIntImpl(14, stackFrame, errorCode);
}


