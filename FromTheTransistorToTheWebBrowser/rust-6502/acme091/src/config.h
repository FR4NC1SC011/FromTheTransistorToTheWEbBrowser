//
// ACME - a crossassembler for producing 6502/65c02/65816 code.
// Copyright (C) 1998-2006 Marco Baye
// Have a look at "acme.c" for further info
//
// Configuration
#ifndef config_H
#define config_H


// types
typedef unsigned int	zone_t;
typedef signed long	value_t;	// At least 32 bits
typedef unsigned long	uvalue_t;	// just for logical shift right
typedef struct label_t		label_t;
typedef struct node_ra_t	node_ra_t;
typedef struct node_t		node_t;
typedef struct result_t		result_t;

// Debugging flag, should be undefined in release version
// #define FDEBUG

// Maximum nesting depth of "!src" and macro calls
// Is not actually a limitation, but a means of finding recursions
#define MAX_NESTING	64
// Default value for output buffer
#define FILLVALUE_INITIAL	0
// Default value for "!fill"
#define FILLVALUE_FILL		0
// Default value for "!align" (234 = NOP)
#define FILLVALUE_ALIGN		234

// Nullpointer definition
#ifndef NULL
#define NULL	((void *)0)
#endif
// Boolean values
#ifndef FALSE
typedef int	bool;
#define FALSE	0
#define TRUE	1
#endif


#endif
