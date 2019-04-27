extern crate clap;
extern crate pad;

use std::io::{self, BufRead, BufReader};
use std::fs::{self, File};
use std::cmp::max;

use clap::{App, Arg, ArgMatches};
use pad::PadStr;

struct Config {
  minimize:  bool,
  separator: String,
  file:      Option<String>,
  outfile:   Option<String>,
}

fn get_config() -> Config {
  let args = App::new("mdtable")
    .version("0.01")
    .author("Axel Lindeberg")
    .about("Makes creating tables in markdown easier!")
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
    .get_matches();
  Config {
    minimize:  args.is_present("minimize"),
    separator: args.value_of("separator").map(String::from).unwrap(),
    file:      args.value_of("infile").map(String::from),
    outfile:   args.value_of("outfile").map(String::from),
  }
}

fn resize_rows(v: &mut Vec<Vec<String>>) {
  let max = v.iter()
    .map(|line: &Vec<String>| line.len())
    .max()
    .unwrap();
  for line in v {
    line.resize(max, String::new());
  }
}

fn read_data_stdin(separator: &String) -> io::Result< Vec<Vec<String>> > {
  let mut v = Vec::new();
  for s in io::stdin().lock().lines() {
    let line = s?;
    if line.trim().is_empty() { break; }
    v.push(line
      .split(separator)
      .map(|word| word.trim())
      .map(String::from)
      .collect()
    );
  }
  resize_rows(&mut v);
  Ok(v)
}

fn read_data_file(separator: &String, file: &String) -> io::Result< Vec<Vec<String>> > {
  let mut v = Vec::new();
  for line in BufReader::new( File::open(file)? ).lines() {
    v.push(line?
      .split(separator)
      .map(|word| word.trim())
      .map(String::from)
      .collect()
    );
  }
  resize_rows(&mut v);
  Ok(v)
}

fn format_minimize(rows: &Vec<Vec<String>>) -> String {
  [
    rows[0].join("|"),
    vec!["---"; rows[0].len()].join("|"),
    rows[1..].iter()
      .map(|row| row.join("|"))
      .collect::<Vec<_>>()
      .join("\n"),
  ].join("\n")
}

fn format_pretty(rows: &Vec<Vec<String>>) -> String {
  let lengths = rows.iter().fold(
    vec![1; rows[0].len()],
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
  [
    format!("| {} |", &format_row(&rows[0])),
    format!("|-{}-|", &lengths.iter()
      .map(|len| "-".repeat(*len))
      .collect::<Vec<_>>()
      .join("-|-")
    ),
    format!("| {} |", &rows[1..].iter()
      .map(format_row)
      .collect::<Vec<_>>()
      .join(" |\n| ")
    ),
  ].join("\n")
}

fn main() -> io::Result<()> {
  let config = get_config();
  let data = match config.file {
    None    => read_data_stdin(&config.separator)?,
    Some(f) => read_data_file(&config.separator, &f)?,
  };
  if data.len() < 2 || data[0].len() == 0 {
    println!("Table requires at least 2 rows (including header) and 1 column.");
    std::process::exit(1);
  }
  let table = match config.minimize {
    true  => format_minimize(&data),
    false => format_pretty(&data),
  } + "\n";
  match config.outfile {
    None    => print!("{}", &table),
    Some(f) => fs::write(f, &table)?,
  };
  Ok(())
}
