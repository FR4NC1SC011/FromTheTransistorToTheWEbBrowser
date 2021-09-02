;;-------------------------------
;; Print strings in SI Register |
;;-------------------------------

print_string:
  pusha                         ; store all registers onto stack
  mov ah, 0x0e                  ; int 10h/ ah 0x0e BIOS teletype output
  mov bh,  0x0                  ; page number
  mov bl, 0x07                  ; color

print_char:
  lodsb                         ; move char value at address in bx into al
  cmp al, 0                     
  je end_print                  ; jump if equal (al = 0) to halt label
  int 0x10                      ; print char in al
  jmp print_char                ; loop

end_print:
  popa                          ; restore all registers from the stack
  ret

