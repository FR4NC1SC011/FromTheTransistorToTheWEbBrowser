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

  ;; Tele-type output strings
  mov bx, helloString           ; moving memory address at helloString into BX reg
  call print_string

  mov bx, string2
  call print_string

  mov dx, 0x12AB
  call print_hex

  ;; End Pgm
  jmp $                         ; keep jumping to here; neverending loop

  ;; Included Files
  include 'print_string.asm'    ; this should be print_string.inc ??? 
  include 'print_hex.asm'

;; VARIABLES
helloString: db 'Char Test: Testing', 0xA, 0xD, 0  ; 0/null to null terminate
string2:     db 'Hex Test: ', 0

  ;; Boot Sector Magic
  times 510-($-$$) db 0         ; pad file with 0s until 510th byte

  dw 0xaa55                     ; BIOS Magic number in 511th and 512th bytes
