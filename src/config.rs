use clap::{App, Arg};

pub struct Config {
  pub minimize:  bool,
  pub separator: String,
  pub file:      Option<String>,
  pub outfile:   Option<String>,
}

impl Config {
  pub fn from_args() -> Config {
    let args = App::new("mdtable")
      .version("1.0.2")
      .author("Axel Lindeberg")
      .about("Makes creating tables in markdown much easier!")
      .arg(Arg::with_name("minimize")
        .help("Minimizes table output")
        .long("minimize")
        .short("m")
      )
      .arg(Arg::with_name("file")
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
      file:      args.value_of("file").map(String::from),
      outfile:   args.value_of("outfile").map(String::from),
    }
  }
}
