    BITS  16
    ORG   0x7C00

; Segment | Name  | Offset Register | Purpose
; cs      | Code  | ip              | Instruction
; ds      | Data  | bx, di, si      | Data
; es      | Extra | bx, di, si      | String stuff...?
; ss      | Stack | sp, bp          | Stack, Base

main:
    cli             ; We'll enable this when we're ready
    xor ax, ax      ; Clear segments as we've set org
    mov ds, ax
    mov es, ax
    mov bx, 0x7000  ; Build stack out of the way a bit
    mov ss, ax
    mov sp, bx

    mov si, welcomeMsg
    call printString

    call loadStage2

    ; Disable for now, want to play with Rust code
    ; call STAGE1_5_TARGET_MEMORY_SEGMENT << 4

    mov si, to32BitMsg
    call printString

    jmp switchTo32bit

    ; Should never reach this
    mov si, returnedFrom32
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
loadStage2:
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

switchTo32bit:
    lgdt [gdtDescriptor]
    mov eax, cr0                        ; Get current state so we can only modify what we want
    or eax, 0x1                         ; We want protected mode
    mov cr0, eax                        ; Apply it
    jmp CODE_SEGMENT:handOffTo32bitCode ; Far jump to flush cache/piplines

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
loadMsg1            db "Reading Stage 2 loader from drive ", 0
loadMsg2            db " for ", 0
loadMsg3            db ` sectors...\r\n`, 0
readFailedMsg       db "Disk read failed with: ", 0
readMismatchMsg     db "Wrong count of sectors read: ", 0
to32BitMsg          db `Switching to 32bit...\r\n`, 0
hexPrefix           db "0x", 0
returnedFrom32      db `32bit mode returned. Something is really busted.\r\n`, 0
haltMsg             db `\r\nEnd of line.`, 0

%include "gdt.asm"

    BITS 32
; Gets everything in a consistent state after switching to 32bit/protected mode
; Then move execution to the stage2 loader (which we've previously loaded to memory)
; as we're too chatty with log messages and are out of space in this segment.
handOffTo32bitCode:
    mov ax, DATA_SEGMENT    ; Load the data segment address
    mov ds, ax              ; Set all segments do it
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov ebp, 0x80000        ; Put stack back where it was
    mov esp, ebp            ; Both are the same as its empty to start with

    jmp STAGE_2_JUMP_TARGET


times 510 - ($ - $$) db 0xDA ; Pad so this will end up exactly at 512 bytes
dw 0xAA55                    ; Boot sector magic number
