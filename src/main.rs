#[macro_use]
extern crate text_io;
extern crate clap;
extern crate pad;

use std::io::{self, BufRead, BufReader, Write};
use std::fs::{self, File};
use std::cmp::max;
use std::iter;

use clap::{App, Arg, ArgMatches};
use pad::PadStr;

struct Config {
  quiet:     bool,
  minimize:  bool,
  separator: String,
  file:      Option<String>,
  outfile:   Option<String>,
}

fn config_from_args(args: ArgMatches) -> Config {
  Config {
    quiet:     args.is_present("quiet"),
    minimize:  args.is_present("minimize"),
    separator: args.value_of("separator").unwrap().to_string(),
    file:      args.value_of("infile").map(String::from),
    outfile:   args.value_of("outfile").map(String::from),
  }
}

fn read_data_stdin(rows: usize, cols: usize, separator: &String) -> io::Result< Vec<Vec<String>> > {
  let mut curr = 0;
  let mut v = vec![Vec::with_capacity(cols); rows+1];
  for s in io::stdin().lock().lines() {
    let line = s?;
    if line.trim().is_empty() { continue; }
    for elem in line.split(separator) {
      v[curr].push(elem.trim().to_string());
      if v[curr].len() == cols {
        if curr == rows { return Ok(v); }
        curr += 1;
      }
    }
  }
  Ok(v)
}

fn read_data_file(rows: usize, cols: usize, separator: &String, file: &String) -> io::Result< Vec<Vec<String>> > {
  let mut curr = 0;
  let mut v = vec![Vec::with_capacity(cols); rows+1];
  for s in BufReader::new(File::open(file)?).lines() {
    let line = s?;
    if line.trim().is_empty() { continue; }
    for elem in line.split(separator) {
      v[curr].push(elem.trim().to_string());
      if v[curr].len() == cols {
        if curr == rows { return Ok(v); }
        curr += 1;
      }
    }
  }
  assert!(false, "File '{}' contained less than {} elements", &file, cols * (rows + 1));
  Ok(v)
}

fn format_minimize(cols: usize, rows: &Vec<Vec<String>>) -> String {
  [
    rows[0].join("|"),  // header
    iter::repeat("---") // separation row
      .take(cols)
      .collect::<Vec<_>>()
      .join("|"),
    rows[1..].iter()    // all data rows
      .map(|row| row.join("|"))
      .collect::<Vec<_>>()
      .join("\n"),
  ].join("\n")
}

fn format_pretty(cols: usize, rows: &Vec<Vec<String>>) -> String {
  let mut lengths = vec![1; cols];
  for row in rows {
    lengths = row.iter()
      .zip(&lengths)
      .map(|(e,len)| max(e.len(), *len))
      .collect();
  }
  let format_row = |row: &Vec<String>| row.iter()
    .zip(&lengths)
    .map(|(e, len)| e.pad_to_width(*len))
    .collect::<Vec<_>>()
    .join(" | ");
  [
    format!("| {} |", &format_row(&rows[0])), // header
    format!("|-{}-|", &lengths.iter()         // separation row
      .map(|len| "-".repeat(*len))
      .collect::<Vec<_>>()
      .join("-|-")
    ),
    format!("| {} |", &rows[1..].iter()       // all data rows
      .map(format_row)
      .collect::<Vec<_>>()
      .join(" |\n| ")
    ),
  ].join("\n")
}

fn main() -> io::Result<()> {
  let config = config_from_args(
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
      .arg(Arg::with_name("separator")
        .help("String that separates values.")
        .long("separator")
        .short("s")
        .default_value(",")
      )
      .get_matches()
  );

  if !config.quiet {
    eprint!("Number of rows (not including header) and columns: ");
    io::stdout().flush()?;
  }
  let (rows, cols) = ( read!(), read!() );
  assert!(rows > 0, "Need at least one row (got {})", rows);
  assert!(cols > 1, "Need at least 2 columns (got {})", cols);
  if !config.quiet {
    eprintln!("Enter {} values separated by '{}' and/or line breaks.",
      cols*(rows+1),
      &config.separator,
    );
  }
  let data = match config.file {
    None    => read_data_stdin(rows, cols, &config.separator)?,
    Some(f) => read_data_file(rows, cols, &config.separator, &f)?,
  };
  let table = match config.minimize {
    true  => format_minimize(cols, &data),
    false => format_pretty(cols, &data),
  } + "\n";
  match config.outfile {
    None    => print!("{}", table),
    Some(f) => fs::write(f, table)?,
  };
  Ok(())
}
