;;
;;  kernel.asm: basic 'kernel' loaded from our bootsector
;;

  ;; Set Video Mode
  mov ah, 0x00                  ; int 0x10/ ah 0x00 = set video mode
  mov al, 0x03                  ; 80x25 text mode
  int 0x10

  ;; Change color/Palette
  mov ah, 0x0B
  mov bh, 0x00
  mov bl, 0x01
  int 0x10

  mov si, testString
  call print_string

  hlt

print_string:
  mov ah, 0x0e                  ; int 10h/ ah 0x0e BIOS teletype output
  mov bh,  0x0                  ; page number
  mov bl, 0x07                  ; color

print_char:
  mov al, [si]                  ; move char value at address in bx into al
  cmp al, 0                     
  je end_print                  ; jump if equal (al = 0) to halt label
  int 0x10                      ; print char in al
  add si, 1                     ; move 1 byte forward/ get next char  
  jmp print_char                ; loop

end_print:
  ret

testString: db 'Kernel Booted, Welcome to my OS!', 0xA, 0xD, 0

  ;; Sector padding magic
  times 512-($-$$) db 0



