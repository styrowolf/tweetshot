#![feature(const_mut_refs)]

extern crate tokio;
extern crate thirtyfour;
extern crate url;
extern crate const_format;

use thirtyfour::prelude::*;

mod utils;
mod arg_parsing;
mod credentials;
mod tweet_parsing;

use arg_parsing::AppArgs;

#[tokio::main]
async fn main() -> WebDriverResult<()> {

    let AppArgs { links, output_path, login, username, password } = match arg_parsing::parse_args().await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let driver = utils::get_driver().await?;

    if login {
        tweet_parsing::twitter_login(&driver, &username, &password).await?;
    }

    tweet_parsing::tweets_screenshot(&driver, &links, &output_path).await?;

    // Always explicitly close the browser. There are no async destructors.
    driver.quit().await?;

    Ok(())
}

