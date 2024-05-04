use clap::{self, Parser};
use core::fmt;
use std::fmt::Display;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Wpa3WifiArgs {
  // TODO: permit only 0 | 1 | 2 | 3
  #[arg(long)]
  encryption_transition: Option<u8>,

  #[arg(long)]
  hidden: bool,

  // TODO: Percent URL encode non printable chars:
  // printable chars (non-semi): 0x20(space)-3a(:) + 0x3c(<)-7e(~)
  // otherwise percent encoded
  #[arg(long)]
  ssid: String,

  // TODO: Percent encode (https://www.ascii-code.com/)
  #[arg(short, long)]
  password: Option<String>,

  // TODO: public-key
  // TODO: Id???
}

fn fmt_uri_item<T: Display + Clone>(id: &str, value: &Option<T>) -> String {
  value.clone().map(|val| String::from(format!("{id}:{val};"))).unwrap_or(String::from(""))
}

impl Display for Wpa3WifiArgs {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let type_field = &self.password.clone().map(|_| String::from("WPA"));
    let type_str = fmt_uri_item("T", type_field);
    let tr_disable_str = fmt_uri_item("R", &self.encryption_transition);
    let ssid_str = fmt_uri_item("S", &Some(&self.ssid));
    let hidden_field = match &self.hidden {
      true => Some(String::from("true")),
      false => None
    };
    let hidden_str = fmt_uri_item("H", &hidden_field);
    let password_str = fmt_uri_item("P", &self.password);
    write!(f, "WIFI:{type_str}{tr_disable_str}{ssid_str}{hidden_str}{password_str};")
  }
}

fn main() {
    let args_result = Wpa3WifiArgs::try_parse();

    match args_result {
        Ok(args) => {
            println!("{args}");
            exit(0)
        }
        Err(failure) => {
            eprintln!("{failure}");
            exit(1)
        }
    }

}
