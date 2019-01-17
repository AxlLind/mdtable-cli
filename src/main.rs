#[macro_use]
extern crate text_io;
extern crate clap;
extern crate pad;

use clap::{App, Arg, ArgMatches};
use pad::PadStr;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;
use std::iter;
use std::cmp;
use std::fs;

struct Config {
  quiet: bool,
  minimize: bool,
  seperator: String,
  file: Option<String>,
  outfile: Option<String>,
}

fn config_from_args(args: ArgMatches) -> Config {
  Config {
    quiet:     args.is_present("quiet"),
    minimize:  args.is_present("minimize"),
    seperator: args.value_of("seperator").unwrap().to_string(),
    file:      args.value_of("infile").map(String::from),
    outfile:   args.value_of("outfile").map(String::from),
  }
}

fn read_data_stdin(rows: usize, cols: usize, seperator: &String) -> Vec<Vec<String>> {
  let mut v = Vec::with_capacity(rows);
  let mut curr = 0;
  v.push(Vec::with_capacity(cols));
  loop {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Error reading a row");
    if s.trim().is_empty() { continue; }
    for elem in s.split(seperator) {
      v[curr].push(elem.trim().to_string());
      if v[curr].len() == cols {
        if v.len() == rows+1 { return v; }
        curr += 1;
        v.push(Vec::with_capacity(cols));
      }
    }
  }
}

fn read_data_file(rows: usize, cols: usize, seperator: &String, filename: &String) -> Vec<Vec<String>> {
  let size = cols * (rows + 1);
  let s = fs::read_to_string(filename).expect("Error reading in file");
  let mut v = Vec::with_capacity(size);
  let mut curr = 0;
  v.push(Vec::with_capacity(cols));
  for elem in s.split(seperator).take(size) {
    v[curr].push(elem.trim().to_string());
    if v[curr].len() == cols {
      if v.len() == rows+1 { return v; }
      curr += 1;
      v.push(Vec::with_capacity(cols));
    }
  }
  assert!(false, "File contained less than {} elements", size);
  v
}

fn format_minimize(cols: usize, rows: Vec<Vec<String>>) -> String {
  vec![
    // header
    rows[0].join("|"),
    // seperation row
    iter::repeat("---")
      .take(cols)
      .collect::<Vec<_>>()
      .join("|"),
    // all data rows
    rows.iter()
      .skip(1)
      .map(|row| row.join("|"))
      .collect::<Vec<_>>()
      .join("\n"),
  ].join("\n") + "\n"
}

fn format_pretty(cols: usize, rows: Vec<Vec<String>>) -> String {
  let mut lengths = vec![1; cols];
  for row in &rows {
    lengths = row.iter()
      .zip(&lengths)
      .map(|(e,len)| cmp::max(e.len(), *len))
      .collect();
  }
  let row_length : usize = 1 + lengths.iter().fold(0, |s,l| s+l+3);
  let mut s = String::with_capacity(row_length * (rows.len() + 1));
  // header
  s.push_str(&format!("| {} |\n", &rows[0].iter()
    .zip(&lengths)
    .map(|(e, len)| e.pad_to_width(*len))
    .collect::<Vec<_>>()
    .join(" | ")
  ));
  // seperation row
  s.push_str(&format!("|-{}-|\n", &lengths.iter()
    .map(|len| "-".repeat(*len))
    .collect::<Vec<_>>()
    .join("-|-")
  ));
  // all data rows
  s.push_str(&format!("| {} |\n", &rows.iter()
    .skip(1)
    .map(|row| row.iter()
      .zip(&lengths)
      .map(|(e, len)| e.pad_to_width(*len))
      .collect::<Vec<_>>()
      .join(" | ")
    )
    .collect::<Vec<_>>()
    .join(" |\n| ")
  ));
  s
}

fn main() {
  let args = config_from_args(
    App::new("mdtable")
      .version("0.01")
      .author("Axel Lindeberg")
      .about("Makes creating tables in markdown easier!")
      .arg(Arg::with_name("quiet")
        .help("Supresses program instructions.")
        .long("quiet")
        .short("q")
      )
      .arg(Arg::with_name("minimize")
        .help("Minimizes table output")
        .long("minimize")
        .short("m")
      )
      .arg(Arg::with_name("infile")
        .help("Reads table values from this file if given, stdin otherwise.")
        .long("file")
        .short("f")
        .takes_value(true)
      )
      .arg(Arg::with_name("outfile")
        .help("Prints output to this if given, stdout otherwise.")
        .long("out")
        .short("o")
        .takes_value(true)
      )
      .arg(Arg::with_name("seperator")
        .help("String that seperates values.")
        .long("seperator")
        .short("s")
        .default_value(",")
      )
      .get_matches()
  );

  if !args.quiet {
    print!("Number of rows and columns: ");
    stdout().flush().expect("Error flushing stdout");
  }
  let (rows, cols) = ( read!(), read!() );
  assert!(rows > 0, "Need at least one row (got {})", rows);
  assert!(cols > 1, "Need at least 2 columns (got {})", cols);
  let data = match args.file {
    None => read_data_stdin(rows, cols, &args.seperator),
    Some(file) => read_data_file(rows, cols, &args.seperator, &file),
  };
  let s = match args.minimize {
    true  => format_minimize(cols, data),
    false => format_pretty(cols, data),
  };
  match args.outfile {
    None => print!("{}", s),
    Some(file) => fs::write(file, s).expect("Error writing to file"),
  };
}
