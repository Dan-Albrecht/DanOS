; Stage 1's goal is just to load the next stage from disk and jump to it.
; Stage 1 is constrained to a single sector minus partion info / MBR overhead.
; Next stage doesn't have a size contraint.
; Stage 1 starts and exits in read mode.
    BITS  16
    ORG   0x7C00

; Segment | Name  | Offset Register | Purpose
; cs      | Code  | ip              | Instruction
; ds      | Data  | bx, di, si      | Data
; es      | Extra | bx, di, si      | String stuff...?
; ss      | Stack | sp, bp          | Stack, Base

main:
    cli             ; No interupts. We'll enable in the kernel when we can actually handle.
    xor ax, ax      ; Clear segments as we've set org
    mov ds, ax
    mov es, ax
    mov bx, 0x7000  ; Build stack out of the way a bit
    mov ss, ax
    mov sp, bx

    mov si, welcomeMsg
    call printString

    call loadFromDisk

    call STAGE2_TARGET_MEMORY_SEGMENT:0

superHault:
    cli             ; Don't need to allow interupts anymore
    mov si, haltMsg
    call printString
.hault:
    hlt             ; Go ahead and park
    jmp .hault      ; And if somehow we executed again...

; Disk Address Packet Structure
DAPS:
                    db 0x10                         ; Size of packet
                    db 0x0                          ; Always 0
    readCount:      dw MAX_SECTOR_READ_COUNT        ; Number of disk sectors (512 (0x200) bytes) to load. BIOS updates with actual read after completion. QEMU seems to only allow a max of 0x80 sector (64K), BOCHS doesn't seem to care, my ThinkPad 0x7F.
                    dw 0x0                          ; Target address offset
    targetSegment:  dw DISK_DATA_MEMORY_SEGMENT     ; Target address segment
    lba:            dd 0x1                          ; Lower 32-bits of LBA. Starts at 0. This code is in 0 and then the rest of the code is immediately next.
                    dd 0x0                          ; Upper 16-bits of LBA.

; Loads code to DISK_DATA_MEMORY_SEGMENT:0
loadFromDisk:
    pusha

    mov si, loadMsg1
    call printString
    
    xor ax, ax
    mov al, dl          ; DL is set by bios of the drive we're on, print that out
    call printHex16

    mov si, loadMsg2
    call printString

    mov al, FULL_SECTOR_BLOCKS
    call printHex16     ; Prints sectores we'll read

    mov si, loadMsg2_5
    call printString

    mov al, REMAINING_SECTORS
    call printHex16     ; Prints sectores we'll read

    mov si, loadMsg3
    call printString

    mov si, DAPS
    ; DL (read drive) is set by bios and we hopefully didn't overwrite it.

    mov cx, FULL_SECTOR_BLOCKS

    ; Maybe no full blocks to even load
    jcxz loadPartialBlock

loadFullBlock:    
    mov ah, 0x42    ; Extended read function
    int 0x13
    jc readFailed
    push si
    mov si, readBlock
    call printString
    pop si
    add [targetSegment], word (0x20 * MAX_SECTOR_READ_COUNT)
    add [lba], dword MAX_SECTOR_READ_COUNT
    loop loadFullBlock

loadPartialBlock:
    mov [readCount], word REMAINING_SECTORS
    cmp word [readCount], 0
    jz noPartial
    mov ah, 0x42    ; Extended read function
    int 0x13
    jc readFailed

    mov ax, [readCount]
    cmp al, REMAINING_SECTORS
    jne readMismatch

noPartial:
    popa
    ret

; AH has return code
readFailed:
    mov si, readFailedMsg
    call printString
    shr ax, 8       ; Error code is just in AH, so shift it down so that'll all that is present.
    call printHex16
    jmp superHault

; AL has actual read sectors
readMismatch:
    mov si, readMismatchMsg
    call printString
    call printHex16
    jmp superHault

printChar:          ; AL set to char to print.
    pusha
    mov ah, 0x0E    ; Teletype output function
    xor bx, bx      ; BH = page number (0), BL is N/A for this mode 
                    ; so 0 it for consistency
    int 0x10        ; Video Services
    popa
    ret

printString:        ; Null-terminated string at DS:[SI]. Modifies AX, BX, SI.
    pusha
.begin:
    mov al, [si]    ; [si] is shorthand for ds:[si]?
    or al, al       ; Are we at the end of the string?
    je .end         ; Yes, break out of loop
    call printChar
    inc si
    jmp .begin
.end:
    popa
    ret

; Print AX as hex with 0x prefix
printHex16:
    pusha 

    mov si, hexPrefix
    call printString

    mov dx, ax      ; DX contains the original AX incoming value
    mov cl, 12      ; Ammount to shift number we're working on
    mov bx, 4       ; Iteration counter

.begin:
    mov ax, dx      ; Copy the original value
    shr ax, cl
    and ax, 0xF
    cmp ax, 10
    jae .hexDigit
    add ax, 48
    jmp .decimalDigit

.hexDigit:
    add ax, 55

.decimalDigit:
    call printChar

    dec bx
    je .end
    sub cl, 4
    jmp .begin

.end:
    popa
    ret

welcomeMsg          db `Welcome to DanOS!\r\n`, 0
loadMsg1            db "Reading drive ", 0
loadMsg2            db " for ", 0
loadMsg2_5          db " blocks and ", 0
loadMsg3            db ` sectors\r\n`, 0
readBlock           db `Read block\r\n`, 0
readFailedMsg       db "Read failed: ", 0
readMismatchMsg     db "Wrong count of sectors read: ", 0
to32BitMsg          db `Switching to 32bit...\r\n`, 0
hexPrefix           db "0x", 0
haltMsg             db `\r\nEnd of line.`, 0

times 440 - ($ - $$) db 0xDA ; Above can be a max of 440 bytes; add padding as needed so below will be at exact needed offsets

; MBR Data
dd 0            ; Unique Disk ID. Aparently some OSs will just randomly overwrite
dw 0            ; Read Write (0) / ReadOnly (5A5A)
dq 0,0          ; First partion entry
dq 0,0          ; Second partion entry
dq 0,0          ; Thrid partion entry
dq 0,0          ; Fouth patition entry
db 0x55, 0xAA   ; Boot sector magic number
