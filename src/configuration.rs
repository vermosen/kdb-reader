use clap::{arg, Command};

#[derive(Debug, Clone)]
pub struct Configuration {
  pub host: String,
  pub port: i32,
  pub user: String,
  pub pwd: String,
  pub debug: bool,  
  pub sep: String,
  pub nohdr: bool,
  pub limit: i32,
  pub query: String
}

impl Configuration {
  pub fn new() -> Self {
    let args = Command::new("kdb-reader")
      .version("0.0.1")
      .author("jean-mathieu vermosen <vermosen@yahoo.com>")
      .about("A simple kdb client")
      .arg(arg!(--host <VALUE>).required(true).help("kdb host"))
      .arg(arg!(--port <VALUE>).required(true).help("kdb port"))
      .arg(arg!(--user <VALUE>).required(true).help("kdb user"))
      .arg(arg!(--password <VALUE>).required(true).help("kdb password"))
      .arg(arg!(--debug).required(false).help("Enable debug mode"))
      .arg(arg!(--sep <SEP>).required(false).default_value("\t").help("Field separator (default: tab)"))
      .arg(arg!(--nohdr).required(false).help("Do not print header in output"))
      .arg(arg!(--limit <LIMIT>).required(false).default_value("-1").help("limit output (default: none)"))
      .arg(arg!(--query <QUERY>).required(true).help("kdb query to execute"))
      .arg(arg!(--nohdr).required(false).help("remove header from output"))
      .get_matches();

    Self {
      host: args.get_one::<String>("host").unwrap().to_string(),
      port: args.get_one::<i32>("port").unwrap().to_owned(),
      user: args.get_one::<String>("user").unwrap().to_string(),
      pwd: args.get_one::<String>("password").unwrap().to_string(),
      debug: args.get_flag("debug"),
      sep: args.get_one::<String>("sep").unwrap().to_string(),
      nohdr: args.get_flag("nohdr"),
      limit: args.get_one::<i32>("limit").unwrap().to_owned(),
      query: args.get_one::<String>("query").unwrap().to_string()
    }
  }
}