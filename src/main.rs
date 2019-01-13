#[macro_use]
extern crate text_io;
extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::io::stdout;
use std::io::stdin;
use std::io::Write;

#[derive(Debug)]
struct Config {
  quiet: bool,
  minimize: bool,
  file: String,
  outfile: String,
}

#[derive(Debug)]
struct Table {
  rows: u32,
  cols: u32,
  data: Vec<String>,
}

fn config_from_args(args: ArgMatches) -> Config {
  Config {
    quiet:    args.is_present("quiet"),
    minimize: args.is_present("minimize"),
    file: match args.value_of("infile") {
      Some(f) => f.to_string(),
      None => String::new(),
    },
    outfile: match args.value_of("outfile") {
      Some(f) => f.to_string(),
      None => String::new(),
    },
  }
}

fn read_data_stdin(rows: u32, cols: u32) -> Vec<String> {
  let size = cols * (rows + 1);
  let mut v = Vec::with_capacity(size as usize);
  let mut added = 0;
  while added < size {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Error reading a row");
    for elem in s.split(",") {
      if added >= size { break; }
      v.push(elem.trim().to_string());
      added += 1;
    }
  }
  v
}

fn read_data_file(rows: u32, cols: u32, file: &String) -> Vec<String> {
  let mut v = Vec::with_capacity(size as usize);
  v
}

fn main() {
  let args = config_from_args(
    App::new("mdtable")
      .version("0.01")
      .author("Axel Lindeberg")
      .about("Makes creating tables in markdown easier!")
      .arg(Arg::with_name("quiet")
        .help("Supresses program instructions")
        .long("quiet")
        .short("q")
      )
      .arg(Arg::with_name("minimize")
        .help("Minimizes table output")
        .long("minimize")
        .short("m")
      )
      .arg(Arg::with_name("infile")
        .help("Reads table values from this file")
        .long("file")
        .short("f")
        .takes_value(true)
      )
      .arg(Arg::with_name("outfile")
        .help("Prints output to this if specified, stdout otherwise")
        .long("out")
        .short("o")
        .takes_value(true)
      )
      .get_matches()
  );

  println!("{:?}", args);
  if !args.quiet {
    print!("Number of rows and columns: ");
    stdout().flush().expect("Error flushing stdout");
  }
  let (rows, cols) = ( read!(), read!() );
  let mut table = Table {
    rows: rows,
    cols: cols,
    data: match args.file.is_empty() {
      true  => read_data_stdin(rows, cols),
      false => read_data_file(rows, cols, &args.file),
    },
  };
  println!("{:?}", table);
}
