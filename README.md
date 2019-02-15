# mdtable-cli
Command line tool for generating markdown tables. It takes in CSV-formatted data and converts it into a markdown table.

# Usage
```
mdtable -f table.csv -o mdtable.md
mdtable -f table.csv > mdtable.md
cat table.csv | mdtable > mdtable.md
```
The application expects each row of the table as rows in the input, i.e seperated by newlines. Each element of the row is seperated by a "," (or another value, see flag `-s`).

This can either come from a file with the `-f` flag or from stdin. If from stdin you signal end of table by either EOF or by an empty line (i.e hitting enter twice).

See examples of input and output [here](./examples.md).

# Installation
[Coming]

# Options
```
USAGE:
    mdtable-cli [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
    -m, --minimize    Minimizes table output
    -V, --version     Prints version information

OPTIONS:
    -f, --file <infile>            Reads table values from this file if given, stdin otherwise.
    -o, --out <outfile>            Prints output to this if given, stdout otherwise.
    -s, --separator <separator>    String that separates values. [default: ,]
```
