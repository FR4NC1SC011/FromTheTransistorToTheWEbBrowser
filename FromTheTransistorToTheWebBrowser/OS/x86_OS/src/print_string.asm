;; Prints  character strings in BX Register
print_string:
  pusha                         ; store all reg values onto the stack

print_char:
  mov al, [bx]                  ; move char val at address in bx into al
  cmp al, 0
  je end_print                  ; jmp if equal (al == 0) to halt label 
  int 0x10                      ; print character in al
  add bx, 1                     ; move 1 byte forward/ get next char
  jmp print_char                ; loop

end_print:
  popa                          ; restore reg from the stack before returning
  ret


