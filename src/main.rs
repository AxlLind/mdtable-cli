#[macro_use]
extern crate text_io;
extern crate clap;
extern crate pad;

use clap::{App, Arg, ArgMatches};
use pad::PadStr;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;
use std::cmp;
use std::fs;

#[derive(Debug)]
struct Config {
  quiet: bool,
  minimize: bool,
  file: String,
  outfile: String,
}

#[derive(Debug)]
struct Table {
  rows: usize,
  cols: usize,
  data: Vec<Vec<String>>,
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

fn read_data_stdin(rows: usize, cols: usize) -> Vec<Vec<String>> {
  let mut v = Vec::with_capacity(rows);
  let mut curr = 0;
  v.push(Vec::with_capacity(cols));
  loop {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Error reading a row");
    if s.trim().is_empty() { continue; }
    for elem in s.split(",") {
      v[curr].push(elem.trim().to_string());
      if v[curr].len() == cols {
        if v.len() == rows+1 { return v; }
        curr += 1;
        v.push(Vec::with_capacity(cols));
      }
    }
  }
}

fn read_data_file(rows: usize, cols: usize, filename: &String) -> Vec<Vec<String>> {
  let size = cols * (rows + 1);
  let s = fs::read_to_string(filename).expect("Error reading in file");
  let mut v = Vec::with_capacity(size);
  let (mut added, mut curr) = (0,0);
  v.push(Vec::with_capacity(cols));
  for elem in s.split(",").take(size) {
    v[curr].push(elem.trim().to_string());
    if v[curr].len() == cols {
      if v.len() == rows+1 { return v; }
      curr += 1;
      v.push(Vec::with_capacity(cols));
    }
  }
  assert!(added == size, "File contained less than {} elements", size);
  v
}

fn format_minimize(t: &Table) -> String {
  let mut s = String::new();
  s.push_str(&t.data[0][0]);
  // header
  for elem in t.data[0].iter().skip(1) {
    s.push('|');
    s.push_str(&elem);
  }
  // seperation row
  s.push_str("\n---");
  s.push_str(&"|---".repeat(t.cols-1));
  // all data rows
  for row in t.data.iter().skip(1) {
    s.push('\n');
    s.push_str(&row[0]);
    for elem in row.iter().skip(1) {
      s.push('|');
      s.push_str(elem);
    }
  }
  s
}

fn format_pretty(t: &Table) -> String {
  let mut lengths = Vec::with_capacity(t.cols);
  for _ in 0..t.cols { lengths.push(1); }
  for row in &t.data {
    for (i, elem) in row.iter().enumerate() {
      lengths[i] = cmp::max(elem.len(), lengths[i]);
    }
  }
  let mut row_length = 1;
  for len in &lengths { row_length += len + 3; }
  let mut s = String::with_capacity(row_length * (t.data.len()+1));
  // header
  for (i, elem) in t.data[0].iter().enumerate() {
    s.push_str("| ");
    s.push_str(&elem.pad_to_width(lengths[i]+1));
  }
  s.push_str("|\n");
  // seperation row
  for i in 0..t.cols {
    s.push_str("|-");
    s.push_str(&"".pad_to_width_with_char(lengths[i]+1, '-'));
  }
  s.push('|');
  // all data rows
  for row in t.data.iter().skip(1) {
    s.push_str("\n| ");
    for (i, elem) in row.iter().enumerate() {
      s.push_str(&elem.pad_to_width(lengths[i]+1));
      s.push_str("| ");
    }
  }
  s.push('\n');
  s
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

  if !args.quiet {
    print!("Number of rows and columns: ");
    stdout().flush().expect("Error flushing stdout");
  }
  let (rows, cols) = ( read!(), read!() );
  assert!(rows > 0, "Need at least one row (got {})", rows);
  assert!(cols > 1, "Need at least 2 columns (got {})", cols);
  let table = Table {
    rows: rows,
    cols: cols,
    data: match args.file.is_empty() {
      true  => read_data_stdin(rows, cols),
      false => read_data_file(rows, cols, &args.file),
    },
  };
  println!("{:?}", table);
  let s = match args.minimize {
    true  => format_minimize(&table),
    false => format_pretty(&table),
  };
  match args.outfile.is_empty() {
    true  => print!("\n{}", s),
    false => fs::write(args.outfile, s).expect("Error writing to file"),
  };
}
