;; -----------------------------------------------------------
;; RESET GRAPHICS MODE SCREEN 
;; -----------------------------------------------------------
resetGraphicsScreen:
  ;; Set Video Mode
  mov ah, 0x00                  ; int 0x10/ ah 0x00 = set video mode
  mov al, 0x13                  ; 320x200, 256 colors graphics mode
  int 0x10

  ret


