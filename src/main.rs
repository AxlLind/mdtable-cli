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
    Some(f) => BufReader::new( File::open(f)? )
      .lines()
      .collect(),
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
  }
}

fn parse_table_data(lines: &Vec<String>, separator: &String) -> Vec<Vec<String>> {
  let mut rows = lines.iter()
    .map(|line| line
      .split(separator)
      .map(|word| word.trim())
      .map(String::from)
      .collect::<Vec<_>>()
    )
    .collect::<Vec<_>>();
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

  let rows = data.iter()
    .map(|row| row.iter()
      .zip(&lengths)
      .map(|(s, len)| s.pad_to_width(*len))
      .collect::<Vec<_>>()
      .join(" | ")
    )
    .collect::<Vec<_>>();
  let separator = lengths.iter()
    .map(|len| "-".repeat(*len))
    .collect::<Vec<_>>()
    .join("-|-");
  [
    format!("| {} |", rows[0]),
    format!("|-{}-|", separator),
    format!("| {} |", rows[1..].join(" |\n| ")),
  ].join("\n")
}

fn main() -> Result<()> {
  let config = Config::from_args();
  let lines = read_lines(&config.file)?;
  let data = parse_table_data(&lines, &config.separator);

  if data.len() < 2 || data[0].len() == 0 {
    eprintln!("Bad Input: Table requires at least 2 rows (including header) and 1 column.");
    std::process::exit(1);
  }

  let table = match config.minimize {
    true  => format_minimized(&data),
    false => format_pretty(&data),
  } + "\n";
  match config.out {
    Some(f) => fs::write(f, &table)?,
    None    => print!("{}", &table),
  };
  Ok(())
}
