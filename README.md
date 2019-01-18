# mdtable-cli
Command line tool for generating markdown tables. It takes in CSV-formatted data and converts it into a markdown table.

# Usage
```
mdtable -f table.csv -o mdtable.md
mdtable -f table.csv | mdtable.md
```

The application first expects two numbers: the numbers of rows (not including the header) and number of columns. Then the data of the table, seperated by ',' (optionally another given separator string) and/or newlines.

This can either come from a file with the `-f` flag or from stdin.

See examples of input and output [here](./examples.md)

# Installation
[Coming]

# Options
```
USAGE:
    mdtable-cli [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
    -m, --minimize    Minimizes table output
    -q, --quiet       Supresses program instructions.
    -V, --version     Prints version information

OPTIONS:
    -f, --file <infile>            Reads table values from this file if given, stdin otherwise.
    -o, --out <outfile>            Prints output to this if given, stdout otherwise.
    -s, --separator <separator>    String that separates values. [default: ,]
```
