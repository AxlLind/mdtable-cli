# mdtable-cli
Command line tool for generating markdown tables. It takes in CSV-formatted data and converts it into a markdown table.

# Usage
```
mdtable -f table.csv -o mdtable.md
mdtable -f table.csv > mdtable.md
cat table.csv | mdtable > mdtable.md
```
See `Options` for additional flags.

The application expects each row of the table as rows in the input, i.e rows separated by newlines. Each element of the row is seperated by a `,` (or another value, see flag `-s`).

This can either come from a file with the `-f` flag or from stdin. If from stdin you signal end of table by either `EOF` or by an empty line (i.e hitting enter twice).

See examples of input and output [here](./examples.md).

# Installation
If you're a rust programmer (e.g you have `rustc` and `cargo`) simply do:
```
cargo install mdtable-cli
```

If you don't want to download `rustc` and `cargo` you can [download the binary](https://github.com/AxlLind/mdtable-cli/raw/master/mdtable) directly from this repository. Put it in a folder in your path, for example `/usr/local/bin`.

## Building from source
If you don't trust random binaries from github you can build it yourself from source:
```
git clone git@github.com:AxlLind/mdtable-cli.git
cd mdtable-cli
cargo build --release
cp target/release/mdtable /usr/local/bin/
```


# Options
```
USAGE:
    mdtable-cli [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
    -m, --minimize    Minimizes table output
    -V, --version     Prints version information

OPTIONS:
    -f, --file <infile>            Reads table values from this if given, stdin otherwise.
    -o, --out <outfile>            Prints output to this if given, stdout otherwise.
    -s, --separator <separator>    String that separates values. [default: ,]
```
