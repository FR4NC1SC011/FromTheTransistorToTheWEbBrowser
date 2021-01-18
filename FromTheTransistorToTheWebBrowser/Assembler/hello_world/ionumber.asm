section .data 
	userMsg db 'Please enter a number: '
	lenUsrMsg equ $-userMsg
	dispMsg db 'You have entered: '
	lenDispMsg equ $-dispMsg



section .bss 
	num resb 5

section .text
	global main
	main: 
	; User Prompt
	mov eax, 4
	mov ebx, 1
	mov ecx, userMsg
	mov edx, lenUsrMsg
	int 80h
	
	; Read and store the user input
	mov eax, 3
	mov ebx, 2
	mov ecx, num
	mov edx, 5
	int 80h

	; Output the message 'The entered number is: '
	mov eax, 4
	mov ebx, 1
	mov ecx, dispMsg
	mov edx, lenDispMsg
	int 80h

	; Output the number entered 
	mov eax, 4
	mov ebx, 1
	mov ecx, num
	mov edx, 5
	int 80h

	; Exit code
	mov eax, 1
	mov ebx, 0
	int 80h





