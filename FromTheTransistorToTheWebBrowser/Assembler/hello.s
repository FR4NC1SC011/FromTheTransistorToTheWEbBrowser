.global _start

.text

_start:
	
	# Write (1, message, 13)

	mov $1, %rax					# system call 1 is write
	mov $1, %rdi					# file handle 1 is stdout
	mov $message, %rsi		# address of string to output
	mov $13, %rdx					# number if bytes
	syscall								# invoke operating system


	mov $60, %rax         # system call 60 is exit
	xor %rdi, %rdi			  # we want return code 0 
	syscall							  # invoke operating system to exit 


message:
	.ascii "Hello, world\n"
