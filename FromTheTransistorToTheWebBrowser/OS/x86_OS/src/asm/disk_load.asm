;;;
;;; Disk Load:  Read DH sectors into ES:BX memory location from drive DL
;;;
disk_load:
  push dx         ;; Store DX on stack so we can check number of sectors actually read later

  mov ah, 0x02    ;; int 13/ah=02h, BIOS read disk sectors
  mov al, dh      ;; number of sectors we want to read ex. 1
  mov ch, 0x00    ;; cylinder 0
  mov dh, 0x00    ;; head 0
  mov cl, 0x02    ;; start reading at CL sector (sector 2 in this case, right after our bootsector)

  int 0x13        ;; BIOS interrupts for disk functions

  jc disk_error   ;; jump if disk_read error (carry flag set)

  pop dx          ;; restore DX from the stack
  cmp dh, al      ;; if AL(# sectors actually read) != DH(#sectors we wanted to read)
  jne disk_error  ;; error, sectors read not equal to number we wanted to read
  ret             ;; return to caller


disk_error:
  mov bx, DISK_ERROR_MSG
  call print_string
  hlt



DISK_ERROR_MSG: db 'Disk Read Error!!!!', 0
