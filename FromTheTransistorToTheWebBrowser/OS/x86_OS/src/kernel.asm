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
  mov ah, 0x0e
  mov bh,  0x0
  mov bl, 0x07

print_char:
  mov al, [si]
  cmp al, 0
  je end_print
  int 0x10
  add si, 1
  jmp print_char

end_print:
  ret

testString: db 'Kernel Booted, Welcome to my OS!', 0xA, 0xD, 0

  ;; Sector padding magic
  times 512-($-$$) db 0



