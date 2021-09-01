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
  cmp al, 'F'                   ; File table command
  je filebrowser
  cmp al, 'R'                   ; Reboot
  je reboot
  cmp al, 'N'                   ; end our current program
  je end_program
  mov si, failure               ; command not found, boo!
  call print_string
  jmp get_input
  
  
filebrowser:                    ; Menu F)  - File Browser
  ;; Reset Screen State
  ;; ------------------
  ;; Set Video Mode
  mov ah, 0x00                  ; int 0x10/ ah 0x00 = set video mode
  mov al, 0x03                  ; 80x25 text mode
  int 0x10

  ;; Change color/Palette
  mov ah, 0x0B
  mov bh, 0x00
  mov bl, 0x01
  int 0x10

  mov si, fileTableHeading
  call print_string

  ;; Load File Table string from its memory location (0x1000:0000)
  xor cx, cx                    ; reset counter for # chars in file
  mov ax, 0x1000                ; file table location
  mov es, ax                    ; ES = 0x1000
  xor bx, bx                    ; ES:BX = 0x1000:0
  mov ah, 0x0e                  ; get ready to print to screen

fileTable_loop:
  inc bx
  mov al, [ES:BX]
  cmp al, '}'                   ; end of filetable?
  je stop
  cmp al, '-'                   ; at sector number of element?
  je sectorNumber_loop
  cmp al, ','                   ; between table elements?
  je next_element
  inc cx
  int 0x10
  jmp fileTable_loop

sectorNumber_loop:
  cmp cx, 21
  je fileTable_loop
  mov al, ' '
  int 0x10
  inc cx
  jmp sectorNumber_loop

next_element:
  xor cx, cx
  mov al, 0xA
  int 0x10
  mov al, 0xD
  int 0x10
  mov al, 0xA
  int 0x10
  mov al, 0xD
  int 0x10
  jmp fileTable_loop

stop:
  hlt
  
  

reboot:                         ; Far jump to Reset Vector
  jmp 0xFFFF:0x0000

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
               'F) File & Program Browser', 0xA, 0xD,\
               'R) Reboot', 0xA, 0xD, 0

success: db 0xA, 0xD, 'Command ran successfully', 0xA, 0xD, 0
failure: db 0xA, 0xD, 'Command not found :(', 0xA, 0xD, 0

fileTableHeading: db '--------------          ----------',\
                      'File/Program           Sector', 0xA, 0xD,\
                      '-------------          ----------', 0xA, 0xD, 0

cmdString: db '', 0

  ;; Sector padding magic
  times 512-($-$$) db 0



