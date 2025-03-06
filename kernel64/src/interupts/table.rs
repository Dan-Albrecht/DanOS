use super::InteruptDescriptorTable::{ExceptionStackFrame, InterruptHandlerIntImpl, InterruptHandlerWithCodeIntImpl};

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt0(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(0, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt1(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(1, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt2(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(2, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt3(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(3, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt4(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(4, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt5(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(5, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt6(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(6, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt7(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(7, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt8(stackFrame: ExceptionStackFrame, errorCode: u64) {
    unsafe {
        InterruptHandlerWithCodeIntImpl(8, stackFrame, errorCode);
    }
}


#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt9(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(9, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt10(stackFrame: ExceptionStackFrame, errorCode: u64) {
    unsafe {
        InterruptHandlerWithCodeIntImpl(10, stackFrame, errorCode);
    }
}


#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt11(stackFrame: ExceptionStackFrame, errorCode: u64) {
    unsafe {
        InterruptHandlerWithCodeIntImpl(11, stackFrame, errorCode);
    }
}


#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt12(stackFrame: ExceptionStackFrame, errorCode: u64) {
    unsafe {
        InterruptHandlerWithCodeIntImpl(12, stackFrame, errorCode);
    }
}


#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt13(stackFrame: ExceptionStackFrame, errorCode: u64) {
    unsafe {
        InterruptHandlerWithCodeIntImpl(13, stackFrame, errorCode);
    }
}


#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt14(stackFrame: ExceptionStackFrame, errorCode: u64) {
    unsafe {
        InterruptHandlerWithCodeIntImpl(14, stackFrame, errorCode);
    }
}


#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt15(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(15, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt16(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(16, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt17(stackFrame: ExceptionStackFrame, errorCode: u64) {
    unsafe {
        InterruptHandlerWithCodeIntImpl(17, stackFrame, errorCode);
    }
}


#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt18(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(18, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt19(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(19, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt20(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(20, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt21(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(21, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt22(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(22, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt23(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(23, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt24(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(24, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt25(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(25, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt26(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(26, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt27(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(27, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt28(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(28, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt29(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(29, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt30(stackFrame: ExceptionStackFrame, errorCode: u64) {
    unsafe {
        InterruptHandlerWithCodeIntImpl(30, stackFrame, errorCode);
    }
}


#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt31(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(31, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt32(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(32, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt33(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(33, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt34(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(34, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt35(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(35, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt36(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(36, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt37(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(37, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt38(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(38, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt39(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(39, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt40(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(40, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt41(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(41, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt42(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(42, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt43(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(43, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt44(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(44, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt45(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(45, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt46(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(46, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt47(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(47, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt48(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(48, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt49(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(49, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt50(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(50, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt51(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(51, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt52(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(52, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt53(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(53, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt54(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(54, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt55(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(55, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt56(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(56, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt57(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(57, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt58(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(58, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt59(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(59, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt60(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(60, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt61(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(61, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt62(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(62, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt63(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(63, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt64(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(64, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt65(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(65, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt66(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(66, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt67(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(67, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt68(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(68, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt69(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(69, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt70(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(70, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt71(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(71, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt72(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(72, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt73(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(73, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt74(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(74, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt75(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(75, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt76(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(76, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt77(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(77, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt78(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(78, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt79(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(79, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt80(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(80, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt81(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(81, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt82(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(82, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt83(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(83, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt84(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(84, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt85(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(85, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt86(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(86, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt87(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(87, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt88(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(88, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt89(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(89, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt90(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(90, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt91(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(91, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt92(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(92, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt93(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(93, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt94(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(94, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt95(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(95, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt96(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(96, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt97(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(97, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt98(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(98, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt99(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(99, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt100(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(100, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt101(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(101, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt102(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(102, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt103(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(103, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt104(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(104, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt105(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(105, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt106(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(106, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt107(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(107, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt108(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(108, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt109(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(109, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt110(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(110, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt111(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(111, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt112(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(112, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt113(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(113, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt114(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(114, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt115(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(115, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt116(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(116, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt117(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(117, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt118(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(118, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt119(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(119, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt120(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(120, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt121(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(121, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt122(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(122, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt123(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(123, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt124(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(124, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt125(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(125, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt126(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(126, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt127(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(127, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt128(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(128, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt129(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(129, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt130(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(130, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt131(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(131, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt132(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(132, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt133(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(133, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt134(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(134, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt135(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(135, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt136(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(136, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt137(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(137, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt138(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(138, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt139(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(139, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt140(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(140, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt141(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(141, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt142(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(142, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt143(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(143, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt144(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(144, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt145(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(145, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt146(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(146, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt147(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(147, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt148(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(148, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt149(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(149, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt150(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(150, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt151(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(151, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt152(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(152, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt153(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(153, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt154(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(154, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt155(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(155, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt156(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(156, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt157(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(157, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt158(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(158, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt159(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(159, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt160(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(160, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt161(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(161, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt162(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(162, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt163(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(163, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt164(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(164, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt165(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(165, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt166(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(166, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt167(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(167, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt168(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(168, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt169(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(169, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt170(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(170, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt171(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(171, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt172(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(172, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt173(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(173, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt174(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(174, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt175(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(175, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt176(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(176, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt177(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(177, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt178(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(178, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt179(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(179, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt180(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(180, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt181(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(181, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt182(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(182, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt183(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(183, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt184(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(184, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt185(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(185, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt186(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(186, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt187(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(187, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt188(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(188, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt189(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(189, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt190(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(190, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt191(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(191, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt192(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(192, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt193(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(193, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt194(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(194, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt195(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(195, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt196(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(196, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt197(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(197, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt198(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(198, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt199(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(199, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt200(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(200, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt201(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(201, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt202(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(202, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt203(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(203, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt204(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(204, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt205(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(205, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt206(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(206, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt207(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(207, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt208(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(208, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt209(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(209, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt210(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(210, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt211(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(211, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt212(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(212, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt213(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(213, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt214(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(214, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt215(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(215, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt216(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(216, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt217(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(217, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt218(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(218, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt219(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(219, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt220(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(220, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt221(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(221, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt222(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(222, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt223(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(223, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt224(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(224, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt225(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(225, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt226(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(226, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt227(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(227, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt228(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(228, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt229(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(229, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt230(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(230, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt231(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(231, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt232(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(232, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt233(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(233, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt234(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(234, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt235(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(235, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt236(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(236, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt237(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(237, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt238(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(238, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt239(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(239, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt240(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(240, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt241(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(241, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt242(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(242, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt243(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(243, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt244(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(244, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt245(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(245, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt246(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(246, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt247(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(247, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt248(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(248, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt249(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(249, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt250(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(250, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt251(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(251, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt252(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(252, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt253(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(253, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt254(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(254, stackFrame);
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn Interrupt255(stackFrame: ExceptionStackFrame) {
    unsafe {
        InterruptHandlerIntImpl(255, stackFrame);
    }
}

