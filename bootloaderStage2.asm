    BITS  32
    ORG 0xA000 ; This is where we loaded when we read from disk
    KERNEL_ADDRESS equ entry + 0x200 ; It's in the next sector

entry:

    mov esi, welcomeMsg32
    call printVgaStringLastLine32
    jmp KERNEL_ADDRESS
    hlt
    jmp $

; We seem to be running in 80x25
printVgaStringLastLine32:
    pusha

    mov ecx, 0xB8000  ; VGA color buffer
    ; Add x*2
    ; Add y*160
    add ecx, 0xF00   ; Last line
    
.begin:
    mov al, [esi]   ; Get the next character
    or al, al       ; Are we at the end of the string?
    je .end         ; Yes, break out of loop

    mov dx, 0x1A00  ; Green text on blue background
    or dl, al       ; Set the character byte
    mov word [ecx], dx

    inc esi
    add ecx, 2
    jmp .begin
.end:
    popa
    ret

welcomeMsg32          db "We've made it to 32-bit mode!", 0
