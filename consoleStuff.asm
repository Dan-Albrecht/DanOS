disableCursor:
    pusha

    ; http://www.osdever.net/FreeVGA/vga/crtcreg.htm#0A
    ; https://wiki.osdev.org/Text_Mode_Cursor
    mov dx, 0x3D4   ; CRTC Address Register
    mov al, 0xA     ; Cursor Start register
    out dx, al

    inc dx          ; CRTC Data Register
    mov al, 0x20    ; Bit 5 disables, other stuff dunno
    out dx, al

    popa
    ret

scrollVga:
    pusha

    mov cx, 0xB800  ; VGA color buffer
    mov es, cx      ; Set segment to buffer
    xor bx, bx
    
.beginCopy:
    cmp bx, 0xF00
    je .beginZero
    mov word dx, [es: bx+160]   ; Each character is 2 bytes, this is one line forward
    mov word [es: bx], dx
    inc bx
    jmp .beginCopy

.beginZero:
    cmp bx, 0xFA0
    je .end
    mov word [es: bx], 0
    inc bx
    jmp .beginZero

.end:
    popa
    ret

; We seem to be running in 80x25
printVgaStringLastLine:
    pusha

    mov cx, 0xB800  ; VGA color buffer
    mov es, cx      ; Set segment to buffer
    ; Add x*2
    ; Add y*160
    xor bx, bx      ; Offset into buffer
    add bx, 0
    add bx, 0xF00   ; Last line
    
.begin:
    mov al, [si]    ; [si] is shorthand for ds:[si]?
    or al, al       ; Are we at the end of the string?
    je .end         ; Yes, break out of loop

    mov dx, 0x1A00  ; Green text on blue background
    or dl, al
    mov word [es: bx], dx

    inc si
    add bx, 2
    jmp .begin
.end:
    popa
    ret

printNewline:
    mov si, newline
    call printString
    ret

; Prints AX as bin with 0b prefix
printBin16: 
    pusha

    mov si, binaryPrefix
    call printString

    mov bx, ax      ; BX has original value

    shr ax, 8
    call printBin8

    mov al, '`'
    call printChar

    mov ax, bx
    call printBin8
    popa
    ret

; Prints AL as bin, no prefix
printBin8: 
    pusha
    mov bl, 8       ; bl is iteration counter, do this for 16 bits
    mov ch, al      ; CX is the current shifted value

.begin:
    mov dh, ch
    and dh, 0x80
    je .zero
    mov al, '1'
    jmp .afterZero
.zero:
    mov al, '0'
.afterZero:
    call printChar
    dec bl
    je .end
    shl ch, 1
    jmp .begin

.end:
    popa
    ret

colorMsg        db "01234567891123456789212345678931234567894123456789512345678961234567897123456789", 0
binaryPrefix    db "0b", 0
newline         db `\r\n`, 0
