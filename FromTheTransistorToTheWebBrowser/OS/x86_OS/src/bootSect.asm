;;;
;;; Simple Boot loader that uses INT13, AH2 to read from disk into memory 
;;;

  org 0x7c00                    ; 'origin' of Boot code

  ;; set up ES/BX memory address/segment:offset to load sector(s) into
  mov bx, 0x1000                ; load sector to memory address 0x1000
  mov es, bx                    ; ES = 0X1000
  mov bx, 0                     ; ES:BX = 0x1000:0

  ;; set up disk read
  mov dh, 0x0                   ; head 0
  mov dl, 0x0                   ; drive 0
  mov ch, 0x0                   ; cylinder 0
  mov cl, 0x02                  ; starting sector to read from disk

read_disk:
  mov ah, 0x02
  mov al, 0x01
  int 0x13

  jc read_disk

  mov ax, 0x1000
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  mov ss, ax

  jmp 0x1000:0x0

  ;; Boot Sector Magic
  times 510-($-$$) db 0         ; pad file with 0s until 510th byte

  dw 0xaa55                     ; BIOS Magic number in 511th and 512th bytes
