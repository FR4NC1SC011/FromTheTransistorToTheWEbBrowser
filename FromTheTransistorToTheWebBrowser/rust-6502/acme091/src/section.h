//
// ACME - a crossassembler for producing 6502/65c02/65816 code.
// Copyright (C) 1998-2006 Marco Baye
// Have a look at "acme.c" for further info
//
// Section stuff
#ifndef section_H
#define section_H

#include "config.h"


// "section" structure type definition
typedef struct {
	zone_t		zone;	// current zone value
	const char*	type;	// "Zone", "Subzone" or "Macro"
	char*		title;	// zone title, subzone title or macro title
	int		allocated;	// whether title was malloc()'d
} section_t;


// Constants
#define ZONE_GLOBAL	0	// Number of "global zone"


// Variables
extern section_t	*Section_now;	// current section structure


// Prototypes
extern void	Section_new_zone(section_t*, const char* type, char* title, bool allocated);
extern void	Section_init(void);
extern void	Section_passinit(void);
extern void	Section_finalize(section_t* section);


#endif
