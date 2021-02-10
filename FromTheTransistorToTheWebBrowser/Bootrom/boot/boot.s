.code16 
.global init

init:
  jmp init # jump to "init"


.fill 510-(. - init), 1, 0 # add zeroes to make it 510 bytes long
