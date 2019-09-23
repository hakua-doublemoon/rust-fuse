#!/usr/bin/perl

# https://qiita.com/xtetsuji/items/db10ac66f427680ff15c

use strict;
use warnings;

use Term::ANSIColor qw(colored);

my $color = shift || "blue";

while (<>) {
    print colored($_, $color);
}


