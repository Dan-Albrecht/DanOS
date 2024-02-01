    BITS  32
    ORG STAGE_2_LOAD_TARGET ; This is where we loaded when we read from disk

    VGA_ADDRESS_PORT    equ 0x3D4           ; Cathode Ray Tube Control (CRTC) Address Register
    VGA_DATA_PORT       equ 0x3D5           ; CRTC Data Register
    CURSOR_HIGH         equ 0xE             ; Cursor Location High Register
    CURSOR_LOW          equ 0xF             ; Cursor Location Low Register
    VGA_BUFFER          equ 0xB8000         ; VGA color buffer

entry:

    mov esi, welcomeMsg32
    call printVgaStringLastLine32
    jmp KERNEL32_JUMP_TARGET

; We seem to be running in 80x25
printVgaStringLastLine32:
    pusha

    mov dx, VGA_ADDRESS_PORT
    mov al, CURSOR_HIGH
    out dx, al              ; We want the cursor high byte
    
    mov dx, VGA_DATA_PORT
    in al, dx               ; Get it
    
    xor ecx, ecx            ; We'll be ultimately storing an offset into the VGA buffer
                            ; representing where the cursor is and where we should start
                            ; writing the string that this function is called with.
    mov ch, al              ; Move the read value in to the high byte

    mov dx, VGA_ADDRESS_PORT
    mov al, CURSOR_LOW
    out dx, al              ; Now we want the low byte
    
    mov dx, VGA_DATA_PORT
    in al, dx               ; Get it
    
    or cl, al               ; Add it to our offset register
    shl ecx, 1              ; * 2 as each character is VGA buffer is 2 bytes wide


    add ecx, VGA_BUFFER     ; Add the base address of the buffer, so we're now indexing 
                            ; to the correct position where the cursor is.
    
.begin:
    mov al, [esi]           ; Get the next character
    or al, al               ; Are we at the end of the string?
    je .end                 ; Yes, break out of loop

    mov dx, 0x1A00          ; Green text on blue background
    or dl, al               ; Set the character byte
    mov word [ecx], dx

    inc esi
    add ecx, 2
    jmp .begin
.end:

    sub ecx, VGA_BUFFER     ; Get ecx back to when we read it from the ports
                            ; as we're going to write it back to move the cursor
                            ; where the text now ends.
    shr ecx, 1              ; Divide by 2 as we're going to the curros offset 

    mov dx, VGA_ADDRESS_PORT    ; This is basically the opposite of above
    mov al, CURSOR_HIGH         ; Write the high & low bytes to move the cursor
    out dx, al                  ; instead of reading them.
    
    mov dx, VGA_DATA_PORT
    mov al, ch           
    out dx, al               

    mov dx, VGA_ADDRESS_PORT
    mov al, CURSOR_LOW
    out dx, al             
    
    mov dx, VGA_DATA_PORT
    mov al, cl
    out dx, al    

    popa
    ret

welcomeMsg32          db "We've made it to 32-bit mode and have VGA color!", 0
