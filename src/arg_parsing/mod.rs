use url;
use tokio;
use const_format;

use const_format::formatc;
use url::Url;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;

use std::path::PathBuf;

use crate::credentials;

const HELP: &str = formatc!("\
tweetshot

USAGE:
  tweetshot [OPTIONS] [LINK(S)]
FLAGS:
  -h, --help                Prints help information
  -l, --login               Logs in to Twitter
OPTIONS:
  -o, --output PATH         Sets an output path
  -u, --username USERNAME   Uses USERNAME for login [default: {}]
  -p, --password PASSWORD   Uses PASSWORD for login [default: REDACTED]
ARGS:
  <LINK(S)>                 Comma-seperated tweet links or path to file with links
", credentials::USERNAME);

#[derive(Debug)]
pub struct AppArgs {
    pub links: Vec<Url>,
    pub output_path: PathBuf,
    pub login: bool,
    pub username: String,
    pub password: String,
}

pub async fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let links = match parse_links(&pargs.free_from_str::<String>()?).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let path = match pargs.opt_value_from_str::<[&str; 2], String>(["-o", "--output"])? {
        Some(s) => PathBuf::from(s),
        None => PathBuf::from("."),
    };

    let path = match path.is_dir(){
        true => path,
        false => {
            if links.len() == 1 {
                path
            } else {
                eprintln!("Error: one output path given for multiple tweets. Use a output directory instead.");
                std::process::exit(1);
            }
        }
    };

    let login = pargs.contains(["-l", "--login"]);

    let username = pargs.opt_value_from_str(["-u", "--username"])?.unwrap_or(String::from(credentials::USERNAME));
    
    let password = pargs.opt_value_from_str(["-p", "--password"])?.unwrap_or(String::from(credentials::PASSWORD));

    let args = AppArgs {
        links: links,
        output_path: path,
        login: login,
        username: username,
        password: password,
    };

    Ok(args)
}

async fn parse_links(s: &str) -> Result<Vec<Url>, std::io::Error> {
    let path = PathBuf::from(s);
    match path.is_file() {
        true => {
            let mut file = OpenOptions::new().read(true).open(path).await?;
            let mut string = String::new();
            file.read_to_string(&mut string).await?;
            let links: Vec<_> = string.split_whitespace()
                .map(|s| Url::parse(s).expect(&format!("Error: couldn't parse link ({}).", s)))
                .collect();

            Ok(links)
        },
        false => {
            let links: Vec<_> = s.split(",")
            .map(|s| Url::parse(s).expect(&format!("Error: couldn't parse link ({}).", s)))
            .collect();

            Ok(links)
        },
    }
}