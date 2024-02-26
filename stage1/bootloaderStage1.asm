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

    mov ax, 1
    call STAGE1_5_TARGET_MEMORY_SEGMENT << 4

    mov ax, 2
    call STAGE1_5_TARGET_MEMORY_SEGMENT << 4

    ; Should never reach this
    mov si, unexpectedReturn
    call printString
    jmp superHault

superHault:
    cli             ; Don't need to allow interupts anymore
    mov si, haltMsg
    call printString
.hault:
    hlt             ; Go ahead and park
    jmp .hault      ; And if somehow we executed again...

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

    mov al, DISK_DATA_SECTOR_LOAD_COUNT
    call printHex16     ; Prints sectores we'll read

    mov si, loadMsg3
    call printString

    mov ah, 0x2         ; Read Sectors From Drive function
    ; AL (sectors to read) already set
    mov ch, 0x0         ; Read cylinder 0
    mov cl, 0x2         ; Read sector 2. This bootloader was read from sector 1
                        ; and we've put everything else to load right after it.
    mov dh, 0x0         ; Read head 0
    ; DL (read drive) is set by bios and we hopefully didn't overwrite it.
    mov bx, DISK_DATA_MEMORY_SEGMENT
    mov es, bx          ; Setup es:bx memory target
    xor bx, bx          ; No segment offset to load to

    int 0x13            ; Do it
    jc readFailed
    cmp al, DISK_DATA_SECTOR_LOAD_COUNT
    jne readMismatch

    popa
    ret

; AH has return code
readFailed:
    mov si, readFailedMsg
    call printString
    shr ax, 8       ; Move this to the low byte so it only show the error code
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
loadMsg1            db "Reading from drive ", 0
loadMsg2            db " for ", 0
loadMsg3            db ` sectors...\r\n`, 0
readFailedMsg       db "Disk read failed with: ", 0
readMismatchMsg     db "Wrong count of sectors read: ", 0
to32BitMsg          db `Switching to 32bit...\r\n`, 0
hexPrefix           db "0x", 0
unexpectedReturn    db `Execution returned to Stage1 bootloader. Something is really busted.\r\n`, 0
haltMsg             db `\r\nEnd of line.`, 0

times 440 - ($ - $$) db 0xDA ; Above can be a max of 440 bytes add padding as needed so below will be at exact needed offsets

; MBR Data
dd 0            ; Unique Disk ID. Aparently some OSs will just randomly overwrite
dw 0            ; Read Write (0) / ReadOnly (5A5A)
dq 0,0          ; First partion entry
dq 0,0          ; Second partion entry
dq 0,0          ; Thrid partion entry
dq 0,0          ; Fouth patition entry
db 0x55, 0xAA   ; Boot sector magic number
