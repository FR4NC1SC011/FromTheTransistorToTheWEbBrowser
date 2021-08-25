;;;
;;; Simple Boot sector that prints a character using BIOS int 0x10/ AH 0x0e 
;;;

  org 0x7c00                    ; 'origin' of Boot code

  ;; Set Video Mode
  mov ah, 0x00                  ; int 0x10/ ah 0x00 = set video mode
  mov al, 0x03                  ; 80x25 text mode
  int 0x10

  ;; Change color/Palette
  mov ah, 0x0B
  mov bh, 0x00
  mov bl, 0x01
  int 0x10

  mov ah, 0x0e                  ; BIOS Teletype output

  mov bx, helloString           ; moving memory address at helloString into BX reg
  call print_string

  mov bx, string2
  call print_string

  ;; End Pgm
  jmp $                         ; keep jumping to here; neverending loop

  ;; Included Files
  include 'print_string.asm'

;; VARIABLES
helloString: db 'HELLO WORLD', 0xA, 0xD, 0  ; 0/null to null terminate
string2:     db 'OS in ASM', 0

  ;; Boot Sector Magic
  times 510-($-$$) db 0         ; pad file with 0s until 510th byte

  dw 0xaa55                     ; BIOS Magic number in 511th and 512th bytes
