    BITS  16
    ORG   STAGE_1_5_LOAD_TARGET

    pusha

    mov ax, 0x4F00          ; Function 00h - Return VBE Controller Information
    mov di, vbeInfoBlock    ; Buffer to receive data
    push cs                 ; We're currently keep the buffer with the code
    pop es                  ; Buffer will be read from es:di
    int 0x10
    call checkSuccess

    ; Check to see if switchable 8 bits per color is supported
    and byte [capabilities], 0x1
    jz callFailed           ; Fail if not

    mov cx, [videoModeSegment]

    push ds
    mov ds, cx

    push es
    mov es, cx              ; BUGBUG: Using this as a 'current segment' shortcut, gotta be a right way
    
    mov si, [videoModeOffset]
    
    call lookForGoodMode
    cmp cx, 0xFFFF
    je .noModes
    jmp .modeFound
    

.noModes:
    mov si, noModesFound
    call printString
    hlt

.modeFound:

    call switchToNewMode

    pop es
    pop ds
    popa
    ret

; In: CX is mode to switch to
switchToNewMode:
    pusha

    mov ax, 0x4F02      ; Function 02h - Set VBE Mode
    mov bx, cx          ; The mode to use
    or bx, 0x1 << 14    ; Linear buffer
    ;or bx, 0x1 << 15    ; Don't clear display data. BUGBUG: Just curious, should clear.
    int 0x10

    call checkSuccess
    call moveCursorToTopLeft
    mov si, weMadeIt
    call printString

    popa
    ret

; In:  ES is set to segment with mode data
; In:  DS is set to segment with mode data
; In:  SI is pointer offset (es:[si]) to current mode to examine
; Out: CX is mode number to use, 0xFFFF on failure.
lookForGoodMode:
    pusha

    mov di, modeInfoBlock   ; Buffer for data

    push si
    mov si, msg0
    call printString
    pop si

.loopStart:

    mov ax, 0x4F01          ; Function 01h - Return VBE Mode Information. This needs to be in the loop as the interup returns writes stats back to it.
    mov cx, es:[si]
    cmp cx, 0xFFFF
    je .loopEnd           ; No more modes to look at
    
    int 0x10
    call checkSuccess

    mov dx, [modeAttributes]
    and dx, 0x1 << 0        ; Mode supported in this hardware config
    jz .notAMach

    mov dx, [modeAttributes]
    and dx, 0x1 << 2        ; Can use this mode from BIOS (useful before we get to real mode)
    jz .notAMach

    mov dx, [modeAttributes]
    and dx, 0x1 << 4       ; Graphics (not text) mode
    jz .notAMach           

    mov dx, [modeAttributes]
    and dx, 0x1 << 7        ; Linear frame buffer mode is available
    jz .notAMach

    push si
    push ax

    mov si, msg1
    call printString
    mov ax, cx
    call printHex16

    xor bx, bx              ; 'Is bad match' flag

    mov dx, [xResolution]
    cmp dx, 0x320
    je .goodX
    mov bx, 1
.goodX:
    mov si, msg2
    call printString
    mov ax, dx
    call printHex16

    mov dx, [yResolution]
    cmp dx, 0x258
    je .goodY
    mov bx, 1
.goodY:
    mov si, msg3
    call printString
    mov ax, dx
    call printHex16

    mov si, msgSpace
    call printString

    xor ax, ax              ; Clear out high as next sevearl are only one byte
    xor dx, dx
    
    mov dl, [numberOfPlanes]
    mov ax, dx
    cmp ax, 1
    je .goodNumberOfPlanes
    mov bx, 1
.goodNumberOfPlanes:
    call printHex16
    mov si, msg4
    call printString

    mov dl, [bitsPerPixel]
    mov ax, dx

    cmp ax, 8
    je .goodNumberOfPixels
    mov bx, 1
.goodNumberOfPixels:

    call printHex16
    mov si, msg5
    call printString    
    call printNewline

    pop ax                  ; Restore these here, because stuff can jump to not a match and not push
    pop si

    test bx, bx
    jne .notAMach

    mov si, winner
    call printString
    jmp .loopEnd

.notAMach:
    
    add si, 2               ; Move to next mode
    jmp .loopStart

.loopEnd:

    mov [modeAttributes], cx    ; Clearly I have no idea what I'm doing, just need to persiste this somewhere breifly and we're done with this structure
    popa
    mov cx, [modeAttributes]
    ret

checkSuccess:
    cmp ax, 0x4F
    jnz callFailed
    ret

callFailed:
    mov si, msgCallFailed
    call printString
    hlt

%include "consoleStuff.asm"

msg0            db `Potential video modes:\r\n`, 0
msg1            db "Mode ", 0
msg2            db " - ", 0
msg3            db " x ", 0
msg4            db " planes and ", 0
msg5            db " bits per pixel.", 0
winner          db `This is the mode we'll use\r\n`, 0
msgSpace        db " ", 0
msgCallFailed   db `VBE call failed`, 0
noModesFound    db `No good modes found\r\n`, 0
weMadeIt        db `This is our new video mode\r\n`, 0

; 512 bytes
vbeInfoBlock:
    vbeSignature        db 'VBE2'
    vbeVersion          dw 0
    oemString           dd 0
    capabilities        dd 0
    videoModeOffset     dw 0
    videoModeSegment    dw 0
    totalMemory         dw 0 ; Number of 64kb blocks
    otherStuff          times 512 - ($ - vbeInfoBlock) db 0xDA

; 256 bytes
modeInfoBlock:
modeAttributes          dw 0
    winAAttributes      db 0
    winBAttributes      db 0
    winGranulatrity     dw 0
    winSize             dw 0
    winASegment         dw 0
    winBSegment         dw 0
    winFunc             dd 0
    bytesPerScanLine    dw 0
    xResolution         dw 0
    yResolution         dw 0
    xCharSize           db 0
    yCharSize           db 0
    numberOfPlanes      db 0
    bitsPerPixel        db 0
    otherStuff2         times 256 - ($ - modeInfoBlock) db 0xDB