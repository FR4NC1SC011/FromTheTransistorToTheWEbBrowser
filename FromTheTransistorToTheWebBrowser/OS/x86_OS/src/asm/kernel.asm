;;; =======================================================================
;;; Kernel.asm: basic 'kernel' loaded from our bootsector
;;; =======================================================================

    ;; --------------------------------------------------------------------
    ;; Screen & Menu Set up
    ;; --------------------------------------------------------------------
main_menu:
    ;; Reset screen state
    call resetTextScreen

    ;; print menu header & options
    mov si, menuString          
    call print_string

    ;; --------------------------------------------------------------------
    ;; Get user input, print to screen & choose menu option or run command
    ;; --------------------------------------------------------------------
get_input:
    mov si, prompt
    call print_string
    xor cx, cx                  ; reset byte counter of input
    mov si, cmdString           ; si now pointing to cmdString

    mov ax, 0x2000              ; reset ES & DS segments to kernel area
    mov es, ax
    mov ds, ax

keyloop:
    xor ax, ax                  ; ah = 0x0, al = 0x0
    int 0x16                    ; BIOS int get keystroke ah=0, al <- character

    mov ah, 0x0e
    cmp al, 0xD                 ; user pressed enter?
    je run_command

    int 0x10                    ; else print input character to screen
    mov [si], al                ; store input character to string
    inc cx                      ; increment byte counter of input
    inc si                      ; go to next byte at di/cmdString
    jmp keyloop                 ; loop for next character from user

run_command:
    cmp cx, 0
    je input_not_found          ; handle empty input

    mov byte [si], 0            ; else null terminate cmdString from di
    mov si, cmdString           ; reset si to point to start of user input

check_commands:
    push cx
    mov di, cmdDir      
    repe cmpsb
    je filebrowser

    pop cx
    push cx
    mov di, cmdReboot
    mov si, cmdString
    repe cmpsb
    je reboot

    pop cx
    push cx
    mov di, cmdPrtreg
    mov si, cmdString
    repe cmpsb
    je registers_print

    pop cx
    push cx
    mov di, cmdGfxtst
    mov si, cmdString
    repe cmpsb
    je graphics_test

    pop cx
    push cx
    mov di, cmdHlt
    mov si, cmdString
    repe cmpsb
    je end_program

    pop cx

check_files:

input_not_found:
    mov si, failure             ; command not found, boo! D:
    call print_string
    jmp get_input

    ;; -----------------------------------------------------------
    ;; Menu F) - File/Program browser & loader
    ;; -----------------------------------------------------------
filebrowser:
    ;; Reset screen state
    call resetTextScreen

    mov si, fileTableHeading
    call print_string

    ;; Load File Table string from its memory location (0x1000:0000), print file
    ;;  and program names & sector numbers to screen, for user to choose
    ;; -------------------------------------------------------------------
    xor cx, cx              ; reset counter for # of bytes at current filetable entry
    mov ax, 0x1000          ; file table location
    mov es, ax              ; ES = 0x1000
    xor bx, bx              ; ES:BX = 0x1000:0
    mov ah, 0x0e            ; get ready to print to screen

filename_loop:
    mov al, [ES:BX]
    cmp al, 0               ; is file name null? at end of filetable?
    je get_program_name     ; no more names? at end of file table, move on

    int 0x10                ; otherwise print char in al to screen
    cmp cx, 9               ; if at end of name, go on
    je file_ext
    inc cx                  ; increment file entry byte counter
    inc bx                  ; get next byte at filetable
    jmp filename_loop

file_ext:
    ;; 2 blanks before file extension
    mov cx, 2
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
    ;; 9 blanks before entry #
    mov cx, 9
    call print_blanks_loop

    inc bx
    mov al, [ES:BX]
    call print_hex_as_ascii

start_sector_number:
    ;; 9 blanks before starting sector
    mov cx, 9
    call print_blanks_loop

    inc bx
    mov al, [ES:BX]
    call print_hex_as_ascii

file_size:
    ;; 13 blanks before file size
    mov cx, 14
    call print_blanks_loop

    inc bx
    mov al, [ES:BX]
    call print_hex_as_ascii
    mov al, 0xA
    int 0x10
    mov al, 0xD
    int 0x10

    inc bx                  ; get first byte of next file name
    xor cx, cx              ; reset counter for next file name
    jmp filename_loop

    ;; After File table printed to screen, user can input program to load
    ;; ------------------------------------------------------------------
    ;; TODO: Change to accomadate new file table layout
get_program_name:
    mov al, 0xA                 ; print newline...
    int 0x10
    mov al, 0xD
    int 0x10
    mov di, cmdString           ; di now pointing to cmdString
    mov byte [cmdLength], 0     ; reset counter & length of user input

pgm_name_loop:
    xor ax, ax                  ; ah = 0x0, al = 0x0
    int 0x16                    ; BIOS int get keystroke ah = 0, al <- character

    mov ah, 0x0e                ; BIOS teletype output
    cmp al, 0xD                 ; user pressed enter?
    je start_search

    inc byte [cmdLength]        ; if not, add to counter
    mov [di], al                ; store input char to string    
    inc di                      ; go to next byte at di/cmdString
    int 0x10                    ; print input character to screen
    jmp pgm_name_loop           ; loop for next character from user

start_search:
    mov di, cmdString           ; reset di, point to start of command string
    xor bx, bx                  ; reset ES:BX to point to beginning of file table

check_next_char:
    mov al, [ES:BX]             ; get file table char
    cmp al, 0                   ; at end of file table?
    je pgm_not_found            ; if yes, program was not found

    cmp al, [di]                ; does user input match file table character?
    je start_compare

    ;; TODO: Add cmp al, ' ' line here and je start_compare
    ;;  so that user doesn't have to type out name with spaces to work

    add bx, 16                  ; if not, go to next file entry in table
    jmp check_next_char

start_compare:
    push bx                     ; save file table position
    mov byte cl, [cmdLength]

compare_loop:
    mov al, [ES:BX]             ; get file table char
    inc bx                      ; next byte in input/filetable
    cmp al, [di]                ; does input match filetable char?
    jne restart_search          ; if not search again from this point in filetable

    dec cl                      ; if it does match, decrement length counter
    jz found_program            ; counter = 0, input found in filetable
    inc di                      ; else go to next byte of input
    jmp compare_loop

restart_search:
    mov di, cmdString           ; else, reset to start of user input
    pop bx                      ; get the saved file table position
    inc bx                      ; go to next char in file table
    jmp check_next_char         ; start checking again

pgm_not_found:
    mov si, notFoundString      ; did not find program name in file table
    call print_string
    mov ah, 0x00                ; get keystroke, print to screen
    int 0x16
    mov ah, 0x0e
    int 0x10
    cmp al, 'Y'
    je filebrowser              ; reload file browser screen to search again
    jmp fileTable_end           ; else go back to main menu

    ;; read disk sector of program to memory and execute it by far jumping
    ;; -------------------------------------------------------------------
found_program:
    add bx, 4               ; go to starting sector # in file table entry
    mov cl, [ES:BX]         ; sector number to start reading at
    inc bx
    mov bl, [ES:BX]         ; file size in sectors / # of sectors to read

    xor ax, ax              
    mov dl, 0x00            ; disk #
    int 0x13                ; int 13h ah 0 = reset disk system

    mov ax, 0x8000          ; memory location to load pgm to
    mov es, ax
    mov al, bl              ; # of sectors to read
    xor bx, bx              ; ES:BX -> 0x8000:0x0000

    mov ah, 0x02            ; int 13 ah 02 = read disk sectors to memory
    mov ch, 0x00            ; track #
    mov dh, 0x00            ; head #
    mov dl, 0x00            ; drive #

    int 0x13
    jnc pgm_loaded          ; carry flag not set, success

    mov si, notLoaded       ; else error, program did not load correctly
    call print_string
    mov ah, 0x00
    int 0x16
    jmp filebrowser         ; reload file table

pgm_loaded:
    mov ax, 0x8000          ; program loaded, set segment registers to location
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    jmp 0x8000:0x0000       ; far jump to program

fileTable_end:
    mov si, goBackMsg       ; show go back message
    call print_string
    mov ah, 0x00            ; get keystroke
    int 0x16
    jmp main_menu           ; go back to main menu

    ;; -----------------------------------------------------------
    ;; Menu R) - Reboot: far jump to reset vector
    ;; -----------------------------------------------------------
reboot:
    jmp 0xFFFF:0x0000

    ;; -----------------------------------------------------------
    ;; Menu P) - Print Register Values
    ;; -----------------------------------------------------------
registers_print:
    ;; Reset screen state
    call resetTextScreen

    ;; print register values to screen
    mov si, printRegHeading
    call print_string

    call print_registers

    ;; Go back to main menu
    mov si, goBackMsg
    call print_string
    mov ah, 0x00
    int 0x16                ; get keystroke
    jmp main_menu           ; go back to main menu

    ;; -----------------------------------------------------------
    ;; Menu G) - Graphics Mode Test(s)
    ;; -----------------------------------------------------------
graphics_test:
    ;; Reset screen state (gfx)
    call resetGraphicsScreen

    ;; Test Square
    mov ah, 0x0C            ; int 0x10 ah 0x0C - write gfx pixel
    mov al, 0x02            ; green
    mov bh, 0x00            ; page #

    ;; Starting pixel of square
    mov cx, 100             ; column #
    mov dx, 100             ; row #
    int 0x10

    ;; Pixels for columns
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
    jne squareColLoop       ; pixels for next row

    mov ah, 0x00
    int 0x16                ; get keystroke
    jmp main_menu

    ;; -----------------------------------------------------------
    ;; Menu N) - End Pgm
    ;; -----------------------------------------------------------
end_program:
    cli                         ; clear interrupts
    hlt                         ; halt the cpu

    ;; ===========================================================
    ;; End Main Logic
    ;; ===========================================================

    ;; Small routine to convert hex byte to ascii, assume hex digit in AL
print_hex_as_ascii:
    mov ah, 0x0e
    add al, 0x30        ; convert to ascii number
    cmp al, 0x39        ; is value 0h-9h or A-F
    jle hexNum
    add al, 0x7         ; add hex 7 to get ascii 'A' - 'F'
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
    ;; Include Files
    ;; -----------------------------------------------------------
    include "../print/print_string.asm"
    include "../print/print_hex.asm"
    include "../print/print_registers.asm"
    include "../screen/resetTextScreen.asm"
    include "../screen/resetGraphicsScreen.asm"

    ;; -----------------------------------------------------------
    ;; Variables
    ;; -----------------------------------------------------------
menuString:     db '---------------------------------',0xA,0xD,\
        'Kernel Booted, Welcome to QuesOS!', 0xA, 0xD,\
        '---------------------------------', 0xA, 0xD, 0xA, 0xD,0

prompt:         db '>:',0

success:        db 0xA,0xD, 'Program found!', 0xA,0xD,0
failure:        db 0xA,0xD,'Command/Program not found, try again',0xA,0xD,0

windowsMsg:     db 0xA,0xD, 'Oops! Something went wrong :(', 0xA,0xD,0
notLoaded:      db 0xA,0xD, 'Error! Program Not Loaded, Try Again',0xA,0xD,0

        ;; Prompt commands
cmdDir:         db 'dir',0      ; directory command; list all files/pgms on disk
cmdReboot:      db 'reboot',0   ; 'warm' reboot option
cmdPrtreg:      db 'prtreg',0   ; print register values
cmdGfxtst:      db 'gfxtst',0   ; graphics mode test
cmdHlt:         db 'hlt',0      ; e(n)d our current program

fileTableHeading:   db '---------   ---------   -------   ------------   --------------',\
        0xA,0xD,'File Name   Extension   Entry #   Start Sector   Size (sectors)',\
        0xA,0xD,'---------   ---------   -------   ------------   --------------',\
        0xA,0xD,0

printRegHeading:        db '--------   ------------',0xA,0xD,\
        'Register   Mem Location', 0xA,0xD,\
        '--------   ------------',0xA,0xD,0

notFoundString:     db 0xA,0xD,'program not found!, try again? (Y)',0xA,0xD,0
sectNotFound:       db 0xA,0xD,'sector not found!, try again? (Y)',0xA,0xD,0

cmdLength:          db 0

goBackMsg:      db 0xA, 0xD, 0xA, 0xD, 'Press any key to go back...', 0
dbgTest:        db 'Test',0
cmdString:      db ' ', 0

    ;; -----------------------------------------------------------
    ;; Sector Padding magic
    ;; -----------------------------------------------------------
    times 1536-($-$$) db 0       ; pads out 0s until we reach 512th byte
