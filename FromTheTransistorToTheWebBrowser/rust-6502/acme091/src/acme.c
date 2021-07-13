//
// ACME - a crossassembler for producing 6502/65c02/65816 code.
// Copyright (C) 1998-2006 Marco Baye
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA

#define RELEASE		"0.91"		// update before release (FIXME)
#define CODENAME	"Gargravarr"	// update before release
#define CHANGE_DATE	"26 Mar"	// update before release
#define CHANGE_YEAR	"2006"		// update before release
#define HOME_PAGE	"http://home.pages.de/~mac_bacon/smorbrod/acme/"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "acme.h"
#include "alu.h"
#include "cliargs.h"
#include "config.h"
#include "cpu.h"
#include "dynabuf.h"
#include "encoding.h"
#include "flow.h"
#include "global.h"
#include "input.h"
#include "label.h"
#include "macro.h"
#include "mnemo.h"
#include "output.h"
#include "platform.h"
#include "section.h"


// Constants
static const char	FILE_WRITETEXT[]	= "w";
static const char	FILE_WRITEBINARY[]	= "wb";
// names for error messages
static const char	name_outfile[]		= "output filename";
static const char	name_dumpfile[]		= "label dump filename";
// long options
#define OPTION_HELP		"help"
#define OPTION_FORMAT		"format"
#define OPTION_OUTFILE		"outfile"
#define OPTION_LABELDUMP	"labeldump"
#define OPTION_SETPC		"setpc"
#define OPTION_CPU		"cpu"
#define OPTION_INITMEM		"initmem"
#define OPTION_MAXERRORS	"maxerrors"
#define OPTION_MAXDEPTH		"maxdepth"
#define OPTION_VERSION		"version"


// Variables
static const char	**toplevel_sources;
static int		toplevel_src_count	= 0;
static signed long	starting_pc		= PC_UNDEFINED;
static signed long	fill_value		= MEMINIT_USE_DEFAULT;
static struct cpu_t	*default_cpu		= NULL;
const char*	labeldump_filename	= NULL;
const char*	output_filename		= NULL;
// maximum recursion depth for macro calls and "!source"
signed long	macro_recursions_left	= MAX_NESTING;
signed long	source_recursions_left	= MAX_NESTING;

// Functions

// Show release and platform info (and exit, if wanted)
static void show_version(bool exit_after) {
	puts(
"This is ACME, release " RELEASE " (\"" CODENAME "\"), " CHANGE_DATE " " CHANGE_YEAR "\n"
"  " PLATFORM_VERSION
	);
	if(exit_after)
		exit(EXIT_SUCCESS);
}

// Show full help (headline, release/platform/version, copyright, dedication,
// warranty disclaimer, usage) and exit program (SUCCESS)
static void show_help_and_exit(void) {
	puts("\n"
"ACME - the ACME Crossassembler for Multiple Environments\n"
"  Copyright (C) 1998-" CHANGE_YEAR " Marco Baye");
	show_version(FALSE);
	puts(
"ACME comes with ABSOLUTELY NO WARRANTY; for details read the help file.\n"
"  This is free software, and you are welcome to redistribute it under\n"
"  certain conditions; as outlined in the GNU General Public License.\n"
"Dedicated to the wisest being I ever had the pleasure of reading\n"
"  books of (currently spending some time dead for tax reasons).\n"
"The newest version can be found at the ACME homepage:\n"
"  " HOME_PAGE "\n"
"\n"
"Usage: acme [OPTION...] [FILE]...\n"
"  -h, --" OPTION_HELP "             show this help and exit.\n"
"  -f, --" OPTION_FORMAT " FORMAT    select output format.\n"
"  -o, --" OPTION_OUTFILE " FILE     select output file.\n"
"  -l, --" OPTION_LABELDUMP " FILE   select label dump file.\n"
"      --" OPTION_SETPC " NUMBER     set program counter.\n"
"      --" OPTION_CPU " CPU          select target processor.\n"
"      --" OPTION_INITMEM " NUMBER   define 'empty' memory.\n"
"      --" OPTION_MAXERRORS " NUMBER set number of errors before exiting.\n"
"      --" OPTION_MAXDEPTH " NUMBER  set recursion depth for macro calls and !src.\n"
"  -vDIGIT                set verbosity level.\n"
PLATFORM_OPTION_HELP
"  -V, --" OPTION_VERSION "          show version and exit.\n"
	);
	exit(EXIT_SUCCESS);
}


// Error handling

// Tidy up before exiting by saving label dump
int ACME_finalize(int exit_code) {
	FILE*	fd;

	if(labeldump_filename) {
		fd = fopen(labeldump_filename, FILE_WRITETEXT);
		if(fd) {
			Label_dump_all(fd);
			fclose(fd);
		} else {
			fprintf(stderr,
				"Error: Cannot open label dump file \"%s\".\n",
				labeldump_filename);
			exit_code = EXIT_FAILURE;
		}
	}
	return(exit_code);
}

// Save output file
static void save_output_file(void) {
	FILE*	fd;

	// if no output file chosen, tell user and do nothing
	if(output_filename == NULL) {
		fputs("No output file specified (use the \"-o\" option or the \"!to\" pseudo opcode).", stderr);
		return;
	}
	fd = fopen(output_filename, FILE_WRITEBINARY);
	if(fd == NULL) {
		fprintf(stderr, "Error: Cannot open output file \"%s\".\n",
			output_filename);
		return;
	}
	Output_save_file(fd, Mem_lowest_pc, Mem_highest_pc);
	fclose(fd);
}

// Perform a single pass. Returns number of "NeedValue" type errors.
static int perform_pass(int flags) {
	FILE*	fd;
	int	i;

	// be verbose
	if(Process_verbosity > 1)
		puts((flags & PASS_ISFIRST) ? "First pass." : (
			(flags & PASS_ISERROR) ?
				"Further pass needed to find error." :
					"Further pass."
		));
	// call modules' "pass init" functions
	CPU_passinit(default_cpu, starting_pc);// set default cpu values
	Encoding_passinit();	// set default encoding
	Section_passinit();	// set initial zone (untitled)
	// init variables
	pass_flags	= flags;
	pass_undefined_count	= 0;	// no "NeedValue" errors yet
	pass_real_errors = 0;	// no real errors yet
	pc_inc		= 0;	// Increase PCs by this amount at end of line
	// Process toplevel files
	for(i = 0; i<toplevel_src_count; i++) {
		if((fd = fopen(toplevel_sources[i], FILE_READBINARY)))
			Parse_and_close_file(fd, toplevel_sources[i]);
		else {
			fprintf(stderr, "Error: Cannot open toplevel file \"%s\".\n", toplevel_sources[i]);
			pass_real_errors++;
		}
	}
	if(pass_real_errors)
		exit(ACME_finalize(EXIT_FAILURE));
	else
		CPU_end_segment();
	// free all memory blocks on "free-at-end-of-pass" list
	autofree_free(&autofree_list_pass);
	pass_count++;
	return(pass_undefined_count);
}

// do passes until done (or errors occured). Return whether output is ready.
static bool do_actual_work(void) {
	int	undefined_prev,	// "NeedValue" errors of previous pass
		undefined_curr;	// "NeedValue" errors of current pass

	pass_count = 0;
	undefined_curr = perform_pass(PASS_ISFIRST);	// First pass
	// now pretend there has been a pass before the first one
	undefined_prev = undefined_curr + 1;
	// As long as the number of "NeedValue" errors is decreasing but
	// non-zero, keep doing passes.
	while(undefined_curr && (undefined_curr < undefined_prev)) {
		undefined_prev = undefined_curr;
		undefined_curr = perform_pass(0);
	}
	// If still errors (unsolvable by doing further passes),
	// perform additional pass to find and show them
	if(undefined_curr == 0)
		return(TRUE);
	perform_pass(PASS_ISERROR);
	return(FALSE);
}

// copy string to DynaBuf
static void keyword_to_dynabuf(const char keyword[]) {
	DYNABUF_CLEAR(GlobalDynaBuf);
	DynaBuf_add_string(GlobalDynaBuf, keyword);
	DynaBuf_append(GlobalDynaBuf, '\0');
	DynaBuf_to_lower(GlobalDynaBuf, GlobalDynaBuf);// convert to lower case
}

// check output format (the output format tree must be set up at this point!)
static void set_output_format(void) {
	keyword_to_dynabuf(cliargs_get_string("output format"));
	if(!Output_set_output_format()) {
		fprintf(stderr, "%sUnknown output format (use 'cbm' or 'plain').\n", cliargs_error);
		exit(EXIT_FAILURE);
	}
}

// check CPU type (the cpu type tree must be set up at this point!)
static void set_starting_cpu(void) {
	keyword_to_dynabuf(cliargs_get_string("CPU type"));
	if(!CPU_find_cpu_struct(&default_cpu)) {
		fprintf(stderr, "%sUnknown CPU type (use 6502, 6510, 65c02 or 65816).\n", cliargs_error);
		exit(EXIT_FAILURE);
	}
}

// set program counter
static void set_starting_pc(void) {
	starting_pc = cliargs_get_long("program counter");
	if(starting_pc > -1
	&& starting_pc < 65536)
		return;
	fprintf(stderr, "%sProgram counter out of range (0-0xffff).\n", cliargs_error);
	exit(EXIT_FAILURE);
}

// set initial memory contents
static void set_mem_contents(void) {
	fill_value = cliargs_get_long("initmem value");
	if(fill_value >= -128
	&& fill_value <= 255)
		return;
	fprintf(stderr, "%sInitmem value out of range (0-0xff).\n", cliargs_error);
	exit(EXIT_FAILURE);
}

// handle long options (like "--example"). Return unknown string.
static const char* long_option(const char* string) {
	if(strcmp(string, OPTION_HELP) == 0)
		show_help_and_exit();
	else if(strcmp(string, OPTION_FORMAT) == 0)
		set_output_format();
	else if(strcmp(string, OPTION_OUTFILE) == 0)
		output_filename = cliargs_get_string(name_outfile);
	else if(strcmp(string, OPTION_LABELDUMP) == 0)
		labeldump_filename = cliargs_get_string(name_dumpfile);
	else if(strcmp(string, OPTION_SETPC) == 0)
		set_starting_pc();
	else if(strcmp(string, OPTION_CPU) == 0)
		set_starting_cpu();
	else if(strcmp(string, OPTION_INITMEM) == 0)
		set_mem_contents();
	else if(strcmp(string, OPTION_MAXERRORS) == 0)
		max_errors = cliargs_get_long("maximum error count");
	else if(strcmp(string, OPTION_MAXDEPTH) == 0)
		macro_recursions_left = (source_recursions_left = cliargs_get_long("recursion depth"));
//	else if(strcmp(string, "strictsyntax") == 0)
//		strict_syntax = TRUE;
	PLATFORM_LONGOPTION_CODE
	else if(strcmp(string, OPTION_VERSION) == 0)
		show_version(TRUE);
	else return(string);
	return(NULL);
}

// Handle short options (like "-e"). Return unknown character.
static char short_option(const char* argument) {
	while(*argument) {
		switch(*argument) {

			case 'h':	// "-h" shows help
			show_help_and_exit();

			case 'f':	// "-f" selects output format
			set_output_format();
			break;

			case 'o':	// "-o" selects output filename
			output_filename = cliargs_get_string(name_outfile);
			break;

			case 'l':	// "-l" selects label dump filename
			labeldump_filename = cliargs_get_string(name_dumpfile);
			break;

			case 'v':	// "-v" changes verbosity
			Process_verbosity++;
			if((argument[1] >= '0') && (argument[1] <= '9'))
				Process_verbosity = *(++argument) - '0';
			break;

			// platform specific switches are inserted here
			PLATFORM_SHORTOPTION_CODE

			case 'V':	// "-V" shows version
			show_version(TRUE);
			break;

			default:	// unknown ones: program termination
			return(*argument);

		}
		argument++;
	}
	return('\0');
}

// guess what
int main(int argc, const char *argv[]) {
	// if called without any arguments, show usage info (not full help)
	if(argc == 1)
		show_help_and_exit();
	cliargs_init(argc, argv);
	DynaBuf_init();// inits *global* dynamic buffer - important, so first
	// Init platform-specific stuff.
	// For example, this could read the library path from an
	// environment variable, which in turn may need DynaBuf already.
	PLATFORM_INIT;
	// init some keyword trees needed for argument handling
	CPUtype_init();
	Outputfile_init();
	// handle command line arguments
	cliargs_handle_options(short_option, long_option);
	// generate list of files to process
	cliargs_get_rest(&toplevel_src_count, &toplevel_sources,
		"No top level sources given");
	// Init modules (most of them will just build keyword trees)
	ALU_init();
	CPU_init();
	Encoding_init();
	Flow_init();
	Input_init();
	Label_init();
	Macro_init();
	Mnemo_init();
	Output_init(fill_value);
	Section_init();
	if(do_actual_work())
		save_output_file();
	return(ACME_finalize(EXIT_SUCCESS));	// dump labels, if wanted
}
