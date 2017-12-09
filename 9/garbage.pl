#!/usr/bin/env perl

use strict;
use warnings;

use Path::Tiny;
use autodie;

sub get_input {
	my $input = path("./input")->child("input.txt")->slurp_utf8;
	$input =~ s/^\s+|\s+$//g;
	return $input;
}

sub part1 {
	my $input = get_input();

	my $val = 0;
	my $current_depth = 0;
	my $in_garbage = 0;
	my $ignore = 0;
	foreach my $char (split //, $input) {
		if ($ignore) {
			$ignore = 0;
		} else {
			if ($char eq "!") {
				$ignore = 1;
			} else {
				if ($in_garbage) {
					if ($char eq ">") {
						$in_garbage = 0;
					}
				} else {
					if ($char eq "<") {
						$in_garbage = 1;
					} else {
						if ($char eq "{") {
							$current_depth++;
						} elsif ($char eq "}") {
							$val += $current_depth;
							$current_depth--;
						}
					}
				}
			}
		}
	}

	print "Part 1: $val\n"
}

sub part2 {
	my $input = get_input();

	my $deleted_chars = 0;
	my $in_garbage = 0;
	my $ignore = 0;
	foreach my $char (split //, $input) {
		if ($ignore) {
			$ignore = 0;
		} else {
			if ($char eq "!") {
				$ignore = 1;
			} else {
				if ($in_garbage) {
					if ($char eq ">") {
						$in_garbage = 0;
					} else {
						$deleted_chars++;
					}
				} elsif ($char eq "<") {
					$in_garbage = 1;
				}
			}
		}
	}

	print "Part 2: $deleted_chars\n"
}

part1();
part2();
