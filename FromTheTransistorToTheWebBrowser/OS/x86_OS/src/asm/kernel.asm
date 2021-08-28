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

  ;; Print Screen Heading and Menu Options
  mov si, menuString
  call print_string

  ;; Get User Input, print to screen and choose menu option or run command
get_input:
  mov di, cmdString

keyloop:
  mov ax, 0x00                  ; ax = 0x00, al = 0x00
  int 0x16                      ; BIOS int get keystroke, char goes into al

  mov ah, 0x0e
  cmp al, 0xD                   ; did user press 'enter' key?
  je run_command
  int 0x10                      ; print input char to screen
  mov [di], al
  inc di
  jmp keyloop                   ; loop for next char

run_command:
  mov byte [di], 0              ; null terminate cmdString from di
  mov al, [cmdString]
  cmp al, 'F'
  jne not_found
  cmp al, 'N'
  je end_program
  mov si, success
  call print_string
  jmp get_input
  
  
not_found:
  mov si, failure
  call print_string
  jmp get_input

end_program:
  ;; End Pgm
  cli
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

menuString: db '--------------------------------', 0xA, 0xD,\ 
               'Kernel Booted, Welcome to my OS', 0xA, 0xD,\
               '--------------------------------', 0xA, 0xD, 0xA, 0xD,\
               'F) File & Program Browser', 0xA, 0xD, 0

success: db 0xA, 0xD, 'Command ran successfully', 0xA, 0xD, 0
failure: db 0xA, 0xD, 'Command not found :(', 0xA, 0xD, 0
cmdString: db '', 0

  ;; Sector padding magic
  times 512-($-$$) db 0



