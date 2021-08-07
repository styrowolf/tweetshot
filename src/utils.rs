use thirtyfour;
use url;

use thirtyfour::prelude::*;
use url::Url;

use std::process::Command;

pub async fn get_driver() -> WebDriverResult<thirtyfour::GenericWebDriver<thirtyfour::http::reqwest_async::ReqwestDriverAsync>> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--window-size=1920,1080")?;
    match WebDriver::new("http://localhost:4444", &caps).await {
        Ok(d) => Ok(d),
        Err(_) => {
            Command::new("chromedriver")
                .arg("--port=4444")
                .spawn()?;
            quick_sleep(1);
            WebDriver::new("http://localhost:4444", &caps).await
        }
     }
}

pub fn quick_sleep(secs: u32) {
    std::thread::sleep(std::time::Duration::from_secs(secs as u64));
}

pub fn screenshot_name_from_url(url: &Url) -> String {
    match url.path_segments() {
        Some(s) => {
            let mut name = String::new();
            let s: Vec<&str> = s.collect();
            for i in 0..s.len() {
                name += s[i];
                if i + 1 != s.len() {
                    name += "-";
                }
            }
            name + ".png"
        },
        None => {
            eprintln!("Error: unable to generate screenshot name.");
            std::process::exit(1);
        },
    }
}