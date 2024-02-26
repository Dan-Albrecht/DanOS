; Global Descriptor Table

nullSegment:
    dq 0x0          ; Per the spec

codeSegment:
    dw 0xFFFF       ; Limit bits 0-15
    dw 0x0          ; Base bits 0-15
    db 0x0          ; Base bits 16-23
    db 10011010b    ; 1  Present bit. Yes, can access.
                    ; 00 Descriptor Privilege Level field. This is a ring 0 segment.
                    ; 1  Type bits. This is a code/data segment.
                    ; 1  Executable bit. This is a code segment.
                    ; 0  Conforming/Direction bit. Less privledged rings cannot call this.
                    ; 1  Since this is a code segment this is the Readable bit. Yes, can read.
                    ; 0  Accessed (gets set by hardware when the segment is accessed, aparently used in debugging)
    db 11001111b    ; 1 Granularity. Limit is sized in 4KB pages.
                    ; 1 DB bit. This is a 32bit segment.
                    ; 0 Long mode bit. This is not long (64 bit), this is 32bit.
                    ; 0 Available. Random bit for us to play with we don't care about.
                    ; 1111 limit bits 16-19
    db 0x0          ; Base bits 24-31

dataSegment:
    dw 0xFFFF       ; Limit bits 0-15
    dw 0x0          ; Base bits 0-15
    db 0x0          ; Base bits 16-23
    db 10010010b    ; 1  Present bit. Yes, can access.
                    ; 00 Descriptor Privilege Level field. This is a ring 0 segment.
                    ; 1  Type bits. This is a code/data segment.
                    ; 0  Executable bit. This is a data segment.
                    ; 0  Conforming/Direction bit. Less privledged rings cannot call this.
                    ; 1  Since this is a data segment this is the Writable bit. Yes, can write.
                    ; 0  Accessed (gets set by hardware when the segment is accessed, aparently used in debugging)
    db 11001111b    ; 1 Granularity. Limit is sized in 4KB pages.
                    ; 1 DB bit. This is a 32bit segment.
                    ; 0 Long mode bit. This is not long (64 bit), this is 32bit.
                    ; 0 Available. Random bit for us to play with we don't care about.
                    ; 1111 limit bits 16-19
    db 0x0          ; Base bits 24-31

gdtDescriptor:
    dw gdtDescriptor - nullSegment - 1  ; Table size - 1 (per spec)
    dd GDT_ADDRESS                      ; Starting address

end:

CODE_SEGMENT        equ codeSegment - nullSegment
DATA_SEGMENT        equ dataSegment - nullSegment
DESCRIPTOR_OFFSET   equ gdtDescriptor - nullSegment
GDT_SIZE            equ end - nullSegment
