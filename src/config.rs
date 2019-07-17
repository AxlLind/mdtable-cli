use clap::{App, Arg};

pub struct Config {
  pub minimize: bool,
  pub separator: String,
  pub file: Option<String>,
  pub out:  Option<String>,
}

impl Config {
  pub fn from_args() -> Config {
    let args = App::new("mdtable")
      .version("1.1.1")
      .author("Axel Lindeberg")
      .about("Makes creating tables in markdown much easier!")
      .args(&[
        Arg::from_usage("[FILE] 'Reads table tables from this. [default: stdin]'"),
        Arg::from_usage("-o --out       [FILE]   'Prints output to this.  [default: stdout]'"),
        Arg::from_usage("-s --separator [STRING] 'Separates values.       [default: ,]'"),
        Arg::from_usage("-m --minimize           'Minimizes table output'"),
      ])
      .get_matches();
    Config {
      minimize:  args.is_present("minimize"),
      separator: args.value_of("separator").unwrap_or(",").to_string(),
      file:      args.value_of("FILE").map(String::from),
      out:       args.value_of("out").map(String::from),
    }
  }
}
