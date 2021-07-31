org 0x7C00
bits 16

%define ENDL 0x0D, 0x0A

start:
  jmp main


; Prints a string to the screen
; Params:
;     - ds:si points to string
;

puts:
  ; save registers we will modify
  push si
  push ax

.loop:
  lodsb      ; loads next character in al
  or al, al  ; next char is null?
  jz .done

  mov ah, 0x0e   ; call bios interrupt
  int 0x10

  jmp .loop

.done:
  pop ax
  pop si
  ret

main:
  ; setup data segments
  mov ax, 0
  mov ds, ax
  mov es, ax
  
  ; setup stack
  mov ss, ax
  mov sp, 0x7CC00

  ; print message
  mov si, msh_hello
  call puts

.halt:
  jmp .halt

msh_hello: db 'hello world', ENDL, 0


times 510-($-$$) db 0
dw 0AA55h
