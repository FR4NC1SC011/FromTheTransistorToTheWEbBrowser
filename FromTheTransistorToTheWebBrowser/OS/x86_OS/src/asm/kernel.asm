;;
;;  kernel.asm: basic 'kernel' loaded from our bootsector
;;


;; ----------------------------------------------------------------------
;; Screen & Menu Set Up
;; ----------------------------------------------------------------------
main_menu:
  ;; Reset Screen State
  call resetTextScreen

  mov si, menuString
  call print_string

;; ----------------------------------------------------------------------
;; Get User Input, print to screen and choose menu option or run command
;; ----------------------------------------------------------------------
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
  cmp al, 'P'                   ; print registers
  je registers_print
  cmp al, 'G'                   ; graphics mode test
  je graphics_test
  cmp al, 'N'                   ; end our current program
  je end_program
  mov si, failure               ; command not found, boo!
  call print_string
  jmp get_program_name
  
;; -----------------------------------------------------------
;; Menu F) - File/Program Browser & Loader 
;; -----------------------------------------------------------
filebrowser:                    ; Menu F)  - File Browser
  ;; Reset Screen State
  call resetTextScreen

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
  je get_program_name
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

;; After File table printed to screen, user can input program to load
;;------------------------------------------------------------------------------------
get_program_name:
  mov ah, 0x0e                  ; print newline
  mov al, 0xA       
  int 0x10
  mov al, 0xD
  int 0x10
  mov di, cmdString             ; di now pointing to cmdString
  mov byte [cmdLength], 0       ; reset counter & length of user input

pgm_name_loop:
  mov ah, 0x00                  ; get keystroke 
  int 0x16                      ; BIOS int get keystroke, char goes into al

  mov ah, 0x0e
  cmp al, 0xD                   ; did user press 'enter' key?
  je start_search

  inc byte [cmdLength]          ; if not, add to counter
  mov [di], al                  ; store input char to command string
  inc di
  int 0x10                      ; print input character to screen
  jmp pgm_name_loop             ; loop for next char

start_search:
  mov di, cmdString             ; reset di, point to start of command string
  xor bx, bx                    ; reset ES:BX to point to beginning of file table

check_next_char:
  mov al, [ES:BX]               ; get file table char
  cmp al, '}'                   ; at end of file table?
  je pgm_not_found

  cmp al, [di]                  ; does user input match file table char?
  je start_compare

  inc bx                        ; if not, get next char in filetable and recheck
  jmp check_next_char  

start_compare:
  push bx                       ; save file table position
  mov byte cl, [cmdLength]

compare_loop:
  mov al, [ES:BX]               ; get file table char
  inc bx                        ; next byte in input/filetable
  cmp al, [di]                  ; does input match filetable char?
  jne restart_search 

  dec cl                        ; if it does match, decrement length counter
  jz found_program              ; counter = 0, input found in filetable
  inc di                        ; else go to next byte of input
  jmp compare_loop

restart_search:
  mov di, cmdString             ; else, reset to the start of the user input
  pop bx                        ; get the saved file table position
  inc bx                        ; go to next char in file table
  jmp check_next_char           ; start checking again

pgm_not_found:
  mov si, notFoundString        ; did not find program name in file table
  call print_string
  mov ah, 0x00                  ; get keystroke
  int 0x16
  mov ah, 0x0e
  int 0x10
  cmp al, 'Y'
  je filebrowser
  jmp filetable_end

;; Get sector number after pgm name in file table
;;-------------------------------------------------
found_program:
  inc bx
  mov cl, 10                    ; use to get sector number
  xor al, al                    ; reset al to 0

next_sector_number:
  mov dl, [ES:BX]               ; checking next byte of file table
  inc bx
  cmp dl, ','                    ; at end of sector number?
  je load_program               ; if so, load program of that sector
  cmp dl, 48                    ; else, check if al is '0' - '9' in ASCII
  jl sector_not_found           ; before '0' not a number
  cmp dl, 57
  jg sector_not_found           ; after '9' not a number
  sub dl, 48                    ; convert ASCII char to integer
  mul cl                        ; al * cl (al * 10), result in AH/AL (AX)
  add al, dl                    ; al = al + dl
  jmp next_sector_number

sector_not_found:
  mov si, secNotFound           ; did not find program name in file table
  call print_string
  mov ah, 0x00
  int 0x16
  mov ah, 0x0e
  int 0x10
  cmp al, 'Y'
  je filebrowser
  jmp filetable_end

;; Read disk sector of program to memory and execute it by far jumping
;;---------------------------------------------------------------------
load_program:
  mov cl, al                   ; cl = sector # to start loading/reading at

  mov ah, 0x00                 ; int 13h ah 0 = reset disk system
  mov dl, 0x00
  int 0x13

  mov ax, 0x8000               ; memory location to load pgm to
  mov es, ax
  xor bx, bx                   ; ES:BX -> 0x8000:0x0000

  mov ah, 0x02                 ; int 13h ah 02 = read disk sectors to memory
  mov al, 0x01                 ; # of sectors to read
  mov ch, 0x00                 ; track #
  mov dh, 0x00                 ; head #
  mov dl, 0x00                 ; drive #

  int 0x13
  jnc pgm_loaded               ; carry flag not set success

  mov si, notLoaded            ; else error, program did not load correctly
  call print_string
  mov ah, 0x00
  int 0x16
  jmp filebrowser              ; reload file table

pgm_loaded:
  mov ax, 0x8000                ; program loaded, set segment registers to location
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  mov ss, ax
  jmp 0x8000:0x000              ; far jump to progam

filetable_end:
  mov si, goBackMsg
  call print_string
  mov ah, 0x00                  ; get keystroke
  int 0x16                      
  jmp main_menu                 ; go back to main menu
  
;; -----------------------------------------------------------
;; Menu R) - Reboot: Jump to Reset Vector
;; -----------------------------------------------------------
reboot:                         ; Far jump to Reset Vector
  jmp 0xFFFF:0x0000

;; -----------------------------------------------------------
;; Menu P) - Print Register Values 
;; -----------------------------------------------------------
registers_print:
  ;; Reset Screen State
  call resetTextScreen

  mov si, printRegHeading
  call print_string

  call print_registers

  mov si, goBackMsg
  call print_string
  mov ah, 0x00
  int 0x16                      ; get keystroke
  jmp main_menu                 ; go back to main menu


;; -----------------------------------------------------------
;; Menu G) - Graphics Mode Test 
;; -----------------------------------------------------------
graphics_test:
  call resetGraphicsScreen

  ;; Test Square
  mov ah, 0x0C                  ; int 0x10 ah 0x0C - write gfx pixel
  mov al, 0x01                  ; blue
  mov bh, 0x00                  ; page #
  
  ;; Starting pixel of square
  mov cx, 100                   ; column #
  mov dx, 100                   ; row #
  int 0x10

  ;; Pixels for Columns
squareColLoop:
  inc cx
  int 0x10
  cmp cx, 150
  jne squareColLoop

  ;; Go down one row
  inc dx
  int 0x10
  mov cx, 99
  cmp dx, 150
  jne squareColLoop

  mov ah, 0x00
  int 0x16
  jmp main_menu

;; -----------------------------------------------------------
;; Menu N) - End Pgm
;; -----------------------------------------------------------
end_program:
  ;; End Pgm
  cli
  hlt



;; -----------------------------------------------------------
;; ===========================================================
;; End Main Logic
;; ===========================================================
;; -----------------------------------------------------------



;; -----------------------------------------------------------
;; INCLUDE FILES
;; -----------------------------------------------------------
  include "../print/print_string.asm"
  include "../print/print_registers.asm"
  include "../print/print_hex.asm"
  include "../screen/resetTextScreen.asm"
  include "../screen/resetGraphicsScreen.asm"

;; -----------------------------------------------------------
;; VARIABLES
;; -----------------------------------------------------------
menuString: db '--------------------------------', 0xA, 0xD,\ 
               'Kernel Booted, Welcome to my OS', 0xA, 0xD,\
               '--------------------------------', 0xA, 0xD, 0xA, 0xD,\
               'F) File & Program Browser', 0xA, 0xD,\
               'R) Reboot', 0xA, 0xD,\ 
               'G) Graphics Mode Test', 0xA, 0xD,\
               'P) Print Register Values', 0xA, 0xD, 0

success:   db 0xA, 0xD, 'Command ran successfully', 0xA, 0xD, 0
failure:   db 0xA, 0xD, 'Command not found :(', 0xA, 0xD, 0
notLoaded: db 0xA, 0xD, 'Error Program Not Loaded, Try Again', 0xA, 0xD, 0

goBackMsg: db 0xA, 0xD, 0xA, 0xD, 'Press any key to go back...', 0

fileTableHeading: db '--------------     ----------',0xA, 0xD,\
                      'File/Program        Sector', 0xA, 0xD,\
                     '--------------     ----------', 0xA, 0xD, 0

printRegHeading:  db '--------   ------------',0xA, 0xD,\
                     'Register   Mem Location', 0xA, 0xD,\
                     '--------   ------------',0xA, 0xD, 0

cmdLength: db 0

notFoundString: db 0xA, 0xD, 'Error: 404 Program not foud, try again? (Y)', 0xA, 0xD, 0
secNotFound: db 0xA, 0xD, 'Sector Not Found, try again? (Y)', 0xA, 0xD, 0

cmdString: db '', 0

  ;; Sector padding magic
  times 1536-($-$$) db 0



