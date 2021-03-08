#!/usr/bin/perl
# -*- perl -*-
# $Header: /home/johnl/book/linker/code/RCS/linkproj04-1.pl,v 1.2 2001/07/23 05:08:38 johnl Exp $
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

# Project 4-1: Unix style storage allocation

use integer;

require 'readobj.pl';

# some parameters

$textbase = 0x1000;		# where the text starts
$pagealign = 0x1000;		# round up for data
$wordalign = 0x4;		# round up for bss and concat'ed segments

# round up a value to a 
sub roundup($$) {
    my ($value, $roundval) = @_;

    return ($value+$roundval-1) & -$roundval;
}

# first read in all of the object files

foreach $fn (@ARGV) {
    push @objects, readobject($fn);
}

# now collect the total sizes of each segment

$tsize = $dsize = $bsize = 0;

foreach $o (@objects) {
    print "visit $o->{name}, ";
    my $t = $o->{segs}[$o->{segnames}->{".text"}];
    $tsize += roundup($t->{size}, $wordalign) if $t;

    my $d = $o->{segs}[$o->{segnames}->{".data"}];

    $dsize += roundup($d->{size}, $wordalign) if $d;

    my $b = $o->{segs}[$o->{segnames}->{".bss"}];

    $bsize += roundup($b->{size}, $wordalign) if $b;
    printf "%X %X %X\n", $tsize, $dsize, $bsize;
}

# set the base of each segment

$tbase = $textbase;

$dbase = roundup($tbase+$tsize, $pagealign); # data is page aligned

$bbase = roundup ($dbase+$dsize, $wordalign); # bss is word aligned

printf "base %X %X %X\n", $tbase, $dbase, $bbase;

# now set the new base values for each module

# running current base for each segment
$tcbase = $tbase; $dcbase = $dbase; $bcbase = $bbase;

foreach $o (@objects) {
    print "revisit $o->{name}, ";
    my $t = $o->{segs}[$o->{segnames}->{".text"}];
   
    if($t) {
	$t->{oldbase} = $t->{base};
	$t->{base} = $tcbase;

	$tcbase += roundup($t->{size}, $wordalign);
    }

    my $d = $o->{segs}[$o->{segnames}->{".data"}];

    if($d) {
	$d->{oldbase} = $d->{base};
	$d->{base} = $dcbase;

	$dcbase += roundup($d->{size}, $wordalign);
    }

    my $b = $o->{segs}[$o->{segnames}->{".bss"}];

    if($b) {
	$b->{oldbase} = $b->{base};
	$b->{base} = $bcbase;

	$bcbase += roundup($b->{size}, $wordalign);
    }

    printf "%X %X %X\n", $tcbase, $dcbase, $bcbase;
}

# now create the output object

%out = (
    name => "a.out.lk",
    nseg => 3,
    nsym => 0,
    nrel => 0,
    segnames => {
	".text" => 1,
	".data" => 2,
	".bss" => 3
	},
    segs => [
	     undef,
	     {
		 name => ".text",
		 segno => 1,
		 base => $tbase,
		 size => $tsize,
		 flags => "RP",
	     },
	     {
		 name => ".data",
		 segno => 2,
		 base => $dbase,
		 size => $dsize,
		 flags => "RWP",
	     },
	     {
		 name => ".bss",
		 segno => 3,
		 base => $bbase,
		 size => $bsize,
		 flags => "RW",
	     }
	     ]
);

writeobject($out{name}, \%out);

