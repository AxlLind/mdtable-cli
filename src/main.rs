extern crate clap;
extern crate pad;

use clap::{App, Arg};
use pad::PadStr;

use std::io::{self, Result, BufRead, BufReader};
use std::fs::{self, File};
use std::cmp::max;

type TableData = Vec<Vec<String>>;

struct Config {
  minimize:  bool,
  separator: String,
  file:      Option<String>,
  outfile:   Option<String>,
}

fn get_config() -> Config {
  let args = App::new("mdtable")
    .version("1.0.1")
    .author("Axel Lindeberg")
    .about("Makes creating tables in markdown much easier!")
    .arg(Arg::with_name("minimize")
      .help("Minimizes table output")
      .long("minimize")
      .short("m")
    )
    .arg(Arg::with_name("infile")
      .help("Reads table values from this if given, stdin otherwise.")
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
    .get_matches();

  Config {
    minimize:  args.is_present("minimize"),
    separator: args.value_of("separator").map(String::from).unwrap(),
    file:      args.value_of("infile").map(String::from),
    outfile:   args.value_of("outfile").map(String::from),
  }
}

fn get_lines_stdin() -> Result<Vec<String>> {
  let mut lines = Vec::new();
  let stdin = io::stdin();
  for s in stdin.lock().lines() {
    let line = s?;
    if line.trim().is_empty() { break; }
    lines.push(line)
  }
  Ok(lines)
}

fn get_lines_file(file: &String) -> Result<Vec<String>> {
  let mut lines = Vec::new();
  for line in BufReader::new( File::open(file)? ).lines() {
    lines.push(line?)
  }
  Ok(lines)
}

fn get_table_data(separator: &String, file: &Option<String>) -> Result<TableData> {
  let lines = match file {
    None    => get_lines_stdin()?,
    Some(f) => get_lines_file(f)?,
  };
  let mut data: TableData = lines.iter()
    .map(|line| line
      .split(separator)
      .map(|word| word.trim())
      .map(String::from)
      .collect()
    )
    .collect();
  let max_len = data.iter()
    .map(|row| row.len())
    .max()
    .unwrap();
  for row in &mut data {
    row.resize(max_len, String::new());
  }
  Ok(data)
}

fn format_minimized(rows: &TableData) -> String {
  [
    rows[0].join("|"),
    vec!["---"; rows[0].len()].join("|"),
    rows[1..].iter()
      .map(|row| row.join("|"))
      .collect::<Vec<_>>()
      .join("\n"),
  ].join("\n")
}

fn format_pretty(data: &TableData) -> String {
  let lengths = data.iter().fold(
    vec![1; data[0].len()],
    |lens, row| row.iter()
      .zip(lens)
      .map(|(s,len)| max(s.len(), len))
      .collect()
  );
  let format_row = |row: &Vec<String>| row.iter()
    .zip(&lengths)
    .map(|(s, len)| s.pad_to_width(*len))
    .collect::<Vec<_>>()
    .join(" | ");

  let header = &format_row(&data[0]);
  let separator = &lengths.iter()
    .map(|len| "-".repeat(*len))
    .collect::<Vec<_>>()
    .join("-|-");
  let rows = &data[1..].iter()
    .map(format_row)
    .collect::<Vec<_>>()
    .join(" |\n| ");

  [
    format!("| {} |", header),
    format!("|-{}-|", separator),
    format!("| {} |", rows),
  ].join("\n")
}

fn main() -> Result<()> {
  let config = get_config();
  let data = get_table_data(&config.separator, &config.file)?;

  if data.len() < 2 || data[0].len() == 0 {
    eprintln!("Table requires at least 2 rows (including header) and 1 column.");
    std::process::exit(1);
  }

  let table = match config.minimize {
    true  => format_minimized(&data),
    false => format_pretty(&data),
  } + "\n";
  match config.outfile {
    None    => print!("{}", &table),
    Some(f) => fs::write(f, &table)?,
  };
  Ok(())
}
