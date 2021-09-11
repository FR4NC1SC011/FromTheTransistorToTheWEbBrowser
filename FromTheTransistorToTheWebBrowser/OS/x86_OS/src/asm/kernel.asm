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
  jmp get_input
  
;; -----------------------------------------------------------
;; Menu F) - File/Program Browser & Loader 
;; -----------------------------------------------------------
filebrowser:                    ; Menu F)  - File Browser
  ;; Reset Screen State
  call resetTextScreen

  mov si, fileTableHeading
  call print_string

  ;; Load File Table string from its memory location (0x1000:0000)
  xor cx, cx                    ; reset counter for # of bytes at ft entry
  mov ax, 0x1000                ; file table location
  mov es, ax                    ; ES = 0x1000
  xor bx, bx                    ; ES:BX = 0x1000:0
  mov ah, 0x0e                  ; get ready to print to screen

filename_loop:
  mov al, [ES:BX]
  cmp al, 0                     ; is file name null? at end of filetable?
  je get_program_name           ; no more names? move on 

  int 0x10                      ; otherwise print char in al to screen
  cmp cx, 9                     ; if at end of name, go on
  je file_ext
  inc cx                        ; increment file entry byte counter
  inc bx                        ; get next byte at filetable
  jmp filename_loop                

file_ext:
  ;; 3 blanks before file extension
  mov cx, 3
  call print_blanks_loop

  inc bx
  mov al, [ES:BX]
  int 0x10
  inc bx
  mov al, [ES:BX]
  int 0x10
  inc bx
  mov al, [ES:BX]
  int 0x10

dir_entry_number:
  ; 9 blanks before entry #
  mov cx, 9
  call print_blanks_loop

  inc bx
  mov al, [ES:BX]
  call print_hex_as_ascii

start_sector_number:
  ; 8 blanks before starting sector
  mov cx, 9
  call print_blanks_loop

  inc bx
  mov al, [ES:BX]
  call print_hex_as_ascii

file_size:
  ; 12 blanks before file size
  mov cx, 12
  call print_blanks_loop

  inc bx
  mov al, [ES:BX]
  call print_hex_as_ascii
  mov al, 0xA
  int 0x10
  mov al, 0xD
  int 0x10

  inc bx
  xor cx, cx
  jmp filename_loop

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
  cmp al, 0                     ; at end of file table?
  je pgm_not_found

  cmp al, [di]                  ; does user input match file table char?
  je start_compare

  add bx, 16                    ; if not, get next char in ft and recheck
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

;; Read disk sector of program to memory and execute it by far jumping
;;---------------------------------------------------------------------
found_program:
  add bx, 4                     ; go to starting sector # in ft entry
  mov cl, [ES:BX]               ; use to get sector number
  inc bx
  mov bl, [ES:BX]               ; file size in sectors

  xor ax, ax
  mov dl, 0x00
  int 0x13                     ; int 13h ah 0 = reset disk system

  mov ax, 0x8000               ; memory location to load pgm to
  mov es, ax
  mov al, bl                   ; # of sectors to read
  xor bx, bx                   ; ES:BX -> 0x8000:0x0000

  mov ah, 0x02                 ; int 13h ah 02 = read disk sectors to memory
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

;; Small routine to convert hex byte to ascii, assume hex digit in AL 
print_hex_as_ascii:
  mov ah, 0x0e
  add al, 0x30                  ; convert to ASCII number
  cmp al, 0x39                  ; is value 0h-9h or A-F
  jle hexNum
  add al, 0x7                       ; add hex 7 to get ASCII 'A' - 'F'

hexNum:
  int 0x10
  ret

;; Small routine to print out cx # of spaces to screen
print_blanks_loop:
  cmp cx, 0
  je end_blanks_loop
  mov ah, 0x0e
  mov al, ' '
  int 0x10
  dec cx
  jmp print_blanks_loop

end_blanks_loop:
  ret




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

fileTableHeading: db  '----------  ----------  --------  -------  ------------',\
             0xA, 0xD,'File Name   Extension   Entry N.   Start   Size(Sector)',\
             0xA, 0xD,'----------  ----------  --------  -------  ------------',\
             0xA, 0xD, 0

printRegHeading:  db '--------   ------------',0xA, 0xD,\
                     'Register   Mem Location', 0xA, 0xD,\
                     '--------   ------------',0xA, 0xD, 0

cmdLength: db 0

notFoundString: db 0xA, 0xD, 'Error: 404 Program not foud, try again? (Y)', 0xA, 0xD, 0
secNotFound: db 0xA, 0xD, 'Sector Not Found, try again? (Y)', 0xA, 0xD, 0

cmdString: db '', 0

  ;; Sector padding magic
  times 1536-($-$$) db 0



