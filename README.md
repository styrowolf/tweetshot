# tweetshot

tweetshot is a command line tool to take screenshots of tweets automatically.

## Installation

Use the package manager [homewbrew](https://brew.sh/) to install chromedriver first.

```bash
brew install --cask chromedriver
```

Create the file src/credentials.rs and two constants called USERNAME and PASSWORD. These are your Twitter login information, used as defaults.

Run the following command to build tweetshot.

```bash
cargo build --release
```

## Usage

```
Oguzs-MacBook-Pro:tweetshot oguzkurt$ tweetshot -h
tweetshot

USAGE:
  tweetshot [OPTIONS] [LINK(S)]
FLAGS:
  -h, --help                Prints help information
  -l, --login               Logs in to Twitter
OPTIONS:
  -o, --output PATH         Sets an output path
  -u, --username USERNAME   Uses USERNAME for login [default: REDACTED]
  -p, --password PASSWORD   Uses PASSWORD for login [default: REDACTED]
ARGS:
  <LINK(S)>                 Comma-seperated tweet links
```

## Contributing

Pull requests and feedback are welcome.

## License
[MIT](https://choosealicense.com/licenses/mit/)