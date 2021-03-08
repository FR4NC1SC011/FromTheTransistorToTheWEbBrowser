
#!/usr/bin/perl
# -*- perl -*-
# $Header: /home/johnl/book/linker/code/RCS/readobj.pl,v 1.4 2001/07/23 05:08:23 johnl Exp $
# Project code for Linkers and Loaders by John R. Levine,
# published by Morgan-Kauffman in October 1999, ISBN 1-55860-496-0.
#
# This code is copyright 2001, John R. Levine. Permission is granted
# to individuals to use this code for non-commercial purposes in
# unmodified or modified form.  Permission is also granted to
# educational institutions to use this code for educational purposes
# in unmodified or modified form.  Other uses, such as including it
# in a product or service offered for sale, require permission from
# the author. 
#
################################################################
# readobj: read and write object files
#
# In each of the hashes below, there may be extra fields beyond
# the ones listed
#
# An object file is a hash with these fields:
# name => file or archive name, if any
# nseg => # of segments
# nsym => # of symbols
# nrel => # of relocs
# segnames => hash names to segment numbers (below)
# segs => [] array of segments
# symnames => hash names to symbol numbers
# syms => [] array of symbols (below)
# rels => [] array of relocs (below)
#
# A segment is a hash with these fields:
# segno => segment number
# base => base address as a number (not a hex string)
# size => size as a number (not a hex string)
# flags => flag characters
# data => data as a byte string (not a hex string)
#
# A symbol is a hash with these fields:
# name => symbol name
# symno => symbol number
# value => symbol value as a number (not a hex string)
# seg => segment number
# type => type
#
# A reloc is a hash with these fields:
# loc => location
# seg => segment number
# ref => reference segment or symbol number
# type => relocation type
#

# get a line, skip blank and comments, trim off newline
sub getl(*) {
    my ($handle) = @_;

    while(1) {
	my $l = <$handle>;

	die "Unexpected EOF" unless defined($l);

	next if $l =~ /^#/; # comment
	next if $l =~ /^\s*$/; # blank line
	chomp($l);
	return $l;
    }
}

sub readobject($) {
    my ($filename) = @_;
    my ($l, $i);

    open(OBJ, $filename) or die "cannot open $filename";

    $l = getl(OBJ);
    # check the header, get the counts
    die "$filename not an object file" unless $l eq "LINK";

    my %o;

    $o{name} = $filename;

    ($o{nseg}, $o{nsym}, $o{nrel}) = split ' ',getl(OBJ);

    # read in the segment descriptions
    for $i (1..$o{nseg}) {
	my ($segname, $base, $size, $flags) = split ' ',getl(OBJ);

	$o{segs}->[$i] = {
	    name => $segname,
	    segno => $i,
	    base => hex($base),
	    size => hex($size),
	    flags => $flags
	    };

	$o{segnames}->{$segname} = $i;
    }

    # read in the symbol table
    for $i (1..$o{nsym}) {
	my ($symname, $value, $seg, $type) = split ' ',getl(OBJ);

	$o{syms}->[$i] = {
	    name => $symname,
	    symno => $i,
	    value => hex($value),
	    seg => $seg,
	    type => $type
	    };
	$o{symnames}->{$symname} = $i;
    }
    # read in the relocations
    for $i (1..$o{nrel}) {
	my ($loc, $seg, $ref, $type, $extra) = split ' ',getl(OBJ), 5;

	$o{rels}->[$i] = {
	    loc => hex($loc),
	    seg => $seg,
	    ref => $ref,
	    type => $type,
	    extra => $extra
	    };
    }
    # slurp in the data
    for $i (1..$o{nseg}) {
	my $s = $o{segs}->[$i];

	next if $s->{flags} !~ /P/;  # bss type not present

	my $t = getl(OBJ);
	my $slen = 2 * $s->{size};

	die "data for $s->{name} is wrong size" unless length($t) == $slen;

	$s->{data} = pack "H$slen", $t;
    }
    close OBJ;
    \%o;
}


################################################################
sub writeobject($%) {
    my ($filename, $oref) = @_;
    my ($l, $i);

    my %o = %$oref;

    open(OBJ, ">$filename") or die "cannot open $filename";

    print OBJ "LINK\n";
    print OBJ "$o{nseg} $o{nsym} $o{nrel}\n";

    # write out the segment descriptions
    for $i (1..$o{nseg}) {
	my $s = $o{segs}->[$i];

	printf OBJ "%s %x %x %s\n", $s->{name}, $s->{base}, $s->{size}, $s->{flags};
    }

    # write out the symbol table
    for $i (1..$o{nsym}) {
	my $s = $o{syms}->[$i];

	printf OBJ "%s %x %d %s\n", $s->{name}, $s->{value}, $s->{seg}, $s->{type};
    }

    # write out the relocations
    for $i (1..$o{nrel}) {
	my $r = $o{rels}->[$i];

	printf OBJ "%x %d %d %s", $r->{loc}, $r->{seg}, $r->{ref}, $r->{type};
	print OBJ " $r->{extra}" if $r->{extra} != "";
	print OBJ "\n";
    }

    # dump out the data
    for $i (1..$o{nseg}) {
	my $s = $o{segs}[$i];

	next if $s->{flags} !~ /P/;  # bss type not present

	print OBJ unpack "H*",$s->{data};
	print OBJ "\n";
    }
    close OBJ;
}

1;
