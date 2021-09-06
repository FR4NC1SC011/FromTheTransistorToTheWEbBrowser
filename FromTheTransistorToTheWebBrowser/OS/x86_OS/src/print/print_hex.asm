;;;
;;;   Prints Hex values using reg DX print_string.asm
;;;
;;;   ASCII '0'-'9' = hex 0x30 - 0x39
;;;   ASCII 'A'-'F' = hex 0x41 - 0x46
;;;   ASCII 'a'-'f' = hex 0x61 - 0x66

print_hex:
  pusha             ;; Save all registers to the stack
  xor cx, cx         ;; Initialize loop counter


hex_loop:
  cmp cx, 4         ;; end of loop?
  je end_hexloop

  ;; Convert DX values to ASCII
  mov ax, dx
  and ax, 0x000F    ;; Turn 1st 3 hex values to 0, keep final digit to convert
  add al, 0x30              ;; get ASCII number or letter value
  cmp al, 0x39      ;; is hex value 0-9 ( <= 39 ) or A-F ( > 39 )
  jle move_intoBX
  add al, 0x7       ;; to get ASCII 'A' - 'F'


  ;; Move ASCII char into BX string

move_intoBX:
  mov bx, hexString + 5     ;; base address of hexString + len of hexString
  sub bx, cx                ;; subtract loop counter
  mov [bx], al
  ror dx, 4                 ;; rotate right 4 bits

  add cx, 1                 ;; increment counter
  jmp hex_loop


end_hexloop:
  mov si, hexString
  call print_string

  popa              ;; restore all registers from the stack
  ret               ;; return to caller

  ;; DATA
hexString: db '0x0000', 0
