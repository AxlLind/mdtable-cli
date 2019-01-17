#[macro_use]
extern crate text_io;
extern crate clap;
extern crate pad;

use std::io;
use std::fs;
use std::iter;
use std::io::Write;
use std::cmp::max;

use clap::{App, Arg, ArgMatches};
use pad::PadStr;

struct Config {
  quiet: bool,
  minimize: bool,
  separator: String,
  file: Option<String>,
  outfile: Option<String>,
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
  let mut v = Vec::with_capacity(rows+1);
  let mut curr = 0;
  v.push(Vec::with_capacity(cols));
  loop {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    if s.trim().is_empty() { continue; }
    for elem in s.split(separator) {
      v[curr].push(elem.trim().to_string());
      if v[curr].len() == cols {
        if v.len() == rows+1 { return Ok(v); }
        curr += 1;
        v.push(Vec::with_capacity(cols));
      }
    }
  }
}

fn read_data_file(rows: usize, cols: usize, separator: &String, filename: &String) -> io::Result< Vec<Vec<String>> > {
  let size = cols * (rows + 1);
  let mut v = Vec::with_capacity(size);
  let mut curr = 0;
  let s = fs::read_to_string(filename)?;
  v.push(Vec::with_capacity(cols));
  for elem in s.split(separator).take(size) {
    v[curr].push(elem.trim().to_string());
    if v[curr].len() == cols {
      if v.len() == rows+1 { return Ok(v); }
      curr += 1;
      v.push(Vec::with_capacity(cols));
    }
  }
  assert!(false, "File contained less than {} elements", size);
  Ok(v)
}

fn format_minimize(cols: usize, rows: Vec<Vec<String>>) -> String {
  vec![
    // header
    rows[0].join("|"),
    // separation row
    iter::repeat("---")
      .take(cols)
      .collect::<Vec<_>>()
      .join("|"),
    // all data rows
    rows[1..].iter()
      .map(|row| row.join("|"))
      .collect::<Vec<_>>()
      .join("\n"),
  ].join("\n")
}

fn format_pretty(cols: usize, rows: Vec<Vec<String>>) -> String {
  let mut lengths = vec![1; cols];
  for row in &rows {
    lengths = row.iter()
      .zip(&lengths)
      .map(|(e,len)| max(e.len(), *len))
      .collect();
  }
  vec![
    // header
    format!("| {} |", &rows[0].iter()
      .zip(&lengths)
      .map(|(e, len)| e.pad_to_width(*len))
      .collect::<Vec<_>>()
      .join(" | ")
    ),
    // separation row
    format!("|-{}-|", &lengths.iter()
      .map(|len| "-".repeat(*len))
      .collect::<Vec<_>>()
      .join("-|-")
    ),
    // all data rows
    format!("| {} |", &rows[1..].iter()
      .map(|row| row.iter()
        .zip(&lengths)
        .map(|(e, len)| e.pad_to_width(*len))
        .collect::<Vec<_>>()
        .join(" | ")
      )
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
    eprint!("Enter {} values separated by '{}' and/or line breaks:\n",
      (rows+1)*cols,
      &config.separator
    );
  }
  let data = match config.file {
    None => read_data_stdin(rows, cols, &config.separator)?,
    Some(f) => read_data_file(rows, cols, &config.separator, &f)?,
  };
  let table = match config.minimize {
    true  => format_minimize(cols, data),
    false => format_pretty(cols, data),
  };
  match config.outfile {
    None => println!("{}", table),
    Some(f) => fs::write(f, table + "\n")?,
  };
  Ok(())
}
