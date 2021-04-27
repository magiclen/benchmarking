#!/usr/bin/env perl

# This program replaces parts of the benchmarking crate's
# README.md and src/lib.rs with sample code from examples/

use 5.10.0;
use strict;
use warnings;
use FindBin '$Bin';

# Work with paths relative to this program's location
chdir $Bin or die;

my ($f, $body);

# Read in the example programs in desired order, then any new ones
my (@e, %seen);
my @files =
    map { "examples/$_" }
    qw(
        read_a_number.rs
        fill_numbers_0_to_99.rs
        fill_a_number.rs
        read_and_write.rs
    );
push @files, glob("examples/*.rs");
for $f (@files) {
    next if $seen{$f}++;
    @ARGV = $f;
    # Wrap code with Markdown ``` code blocks
    push @e, '```' . "rust\n";
    my $in;
    while (<>) {
        # Extract code from fn main and outdent
        $in = 1, next if /^fn main/;
        $in = 0, next if /^}/;
        s/^ {4}// if $in;
        push @e, $_;
    }
    push @e, '```' . "\n\n";
}
my $examples = join('', @e);

# Replace examples in the README
$f = 'README.md';
$body = slurp($f);
$body =~ s{^(## Examples\n\n).+?^(\*)}{$1$examples$2}ms;
save($f, $body);

# Extract part of README we want to copy into source code documentation
my $readme = slurp($f);
$readme =~ s{.+?^(This crate .+?)\n^## Crates\.io.+}{$1}ms;
$readme =~ s{^(?:\* )?}{//! }mg;
$readme =~ s/ *$//msg;

# Replace inner-line documentation with README extract
$f = 'src/lib.rs';
$body = slurp($f);
$body =~ s{^//! This crate .+?\n(^[^/])}{$readme$1}ms;
save($f, $body);


sub slurp {
    local @ARGV = $_[0];
    local $/;
    scalar <>
}

sub save {
    open my $out, '>', $_[0] or die;
    print $out $_[1];
    close $out or die;
}
