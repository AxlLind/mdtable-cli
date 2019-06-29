extern crate clap;
extern crate pad;

mod config;

use pad::PadStr;
use std::io::{self, Result, BufRead, BufReader};
use std::fs::{self, File};
use std::cmp::max;
use config::Config;

fn read_lines(file: &Option<String>) -> Result<Vec<String>> {
  match file {
    None => {
      let stdin = io::stdin();
      let lines = stdin.lock()
        .lines()
        .take_while(|line| match line {
          Ok(s)  => !s.trim().is_empty(),
          Err(_) => false,
        })
        .collect();
      lines
    },
    Some(f) => BufReader::new( File::open(f)? )
      .lines()
      .collect()
  }
}

fn parse_table_data(lines: &Vec<String>, separator: &String) -> Vec<Vec<String>> {
  let mut rows: Vec<Vec<String>> = lines.iter()
    .map(|line| line
      .split(separator)
      .map(|word| word.trim())
      .map(String::from)
      .collect()
    )
    .collect();
  let max_len = rows.iter()
    .map(|row| row.len())
    .max()
    .unwrap_or(0);
  for row in &mut rows {
    row.resize(max_len, String::new());
  }
  rows
}

fn format_minimized(rows: &Vec<Vec<String>>) -> String {
  [
    rows[0].join("|"),
    vec!["---"; rows[0].len()].join("|"),
    rows[1..].iter()
      .map(|row| row.join("|"))
      .collect::<Vec<_>>()
      .join("\n"),
  ].join("\n")
}

fn format_pretty(data: &Vec<Vec<String>>) -> String {
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

  let header = format_row(&data[0]);
  let separator = lengths.iter()
    .map(|len| "-".repeat(*len))
    .collect::<Vec<_>>()
    .join("-|-");
  let rows = data[1..].iter()
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
  let config = Config::from_args();
  let lines  = read_lines(&config.file)?;
  let data   = parse_table_data(&lines, &config.separator);

  if data.len() < 2 || data[0].len() == 0 {
    eprintln!("Bad Input: Table requires at least 2 rows (including header) and 1 column.");
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
