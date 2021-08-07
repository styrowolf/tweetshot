use thirtyfour;
use url;

use thirtyfour::prelude::*;
use url::Url;

use std::path::PathBuf;

use crate::utils;

pub async fn twitter_login(
    driver: &thirtyfour::GenericWebDriver<thirtyfour::http::reqwest_async::ReqwestDriverAsync>,
    username: &str,
    password: &str,
) -> WebDriverResult<()> {

    driver.get("https://twitter.com/login").await?;
    utils::quick_sleep(3);

    let username_input = driver.find_element(By::Name("session[username_or_email]")).await?;
    username_input.send_keys(username).await?;

    let password_input = driver.find_element(By::Name("session[password]")).await?;
    password_input.send_keys(password).await?;

    utils::quick_sleep(1);

    let login_button = driver.find_element(By::XPath("/html/body/div/div/div/div[2]/main/div/div/div[2]/form/div/div[3]/div")).await?;
    login_button.click().await?;

    utils::quick_sleep(1);

    Ok(())
}

pub async fn tweets_screenshot(
    driver: &thirtyfour::GenericWebDriver<thirtyfour::http::reqwest_async::ReqwestDriverAsync>,
    urls: &[Url],
    output_path: &PathBuf,
) -> WebDriverResult<()> {

    for url in urls {
        let link = url.as_str();
        driver.get(link).await?;
        utils::quick_sleep(2);
        let tweet = driver.find_element(By::XPath("/html/body/div/div/div/div[2]/main/div/div/div/div/div/div[2]/div/section/div/div/div[1]/div/div/article")).await?;
        let save_path = match output_path.is_dir() {
            true => {
                let mut path = output_path.clone();
                path.push(utils::screenshot_name_from_url(&url));
                path
            },
            false => output_path.clone(),
        };
        tweet.screenshot(&save_path).await?;
    }
    
    Ok(())
}