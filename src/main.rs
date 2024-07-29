use std::fs::create_dir_all;
use chrono::{Datelike, Utc};
use env_logger::{Builder, Target};
use headless_chrome::{Browser, LaunchOptions};
use log::{error, info, LevelFilter};
use std::error::Error;
use std::fs::{self, File as StdFile};
use std::io::Write;
use std::sync::Arc;
use reqwest::{Client, cookie::Jar, header};
use url::Url;

const BASE_URL: &str = "https://apps.elections.virginia.gov/SBE_CSV/CF/";
const FILES: &[&str] = &[
    "Report.csv",
    "ScheduleA.csv",
    "ScheduleB.csv",
    "ScheduleC.csv",
    "ScheduleD.csv",
    "ScheduleE.csv",
    "ScheduleF.csv",
    "ScheduleG.csv",
    "ScheduleH.csv",
    "ScheduleI.csv",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Builder::from_default_env()
        .target(Target::Stderr)
        .filter(None, LevelFilter::Info)
        .init();

    println!("Starting the application");
    info!("Starting the application");

    // Create a directory to store the reports
    info!("Creating 'reports' directory");
    println!("Creating 'reports' directory");
    fs::create_dir_all("reports")?;

    // Start date
    let mut year = 2012;
    let mut month = 3;

    // Prepare the broken links file
    let mut broken_links = StdFile::create("reports/broken.txt")?;

    // Initialize headless chrome browser
    let browser = Browser::new(LaunchOptions::default())?;

    // Loop until we reach the current month
    let now = Utc::now();
    while year < now.year() || (year == now.year() && month <= now.month()) {
        let dir = format!("{:04}_{:02}", year, month);
        let url = format!("{}{}/", BASE_URL, dir);
        info!("Processing directory: {}", url);
        println!("Processing directory: {}", url);

        // Preload directory page to establish session cookies
        let cookies = match preload_directory_page(&browser, &url) {
            Ok(cookies) => cookies,
            Err(err) => {
                error!("Failed to load directory page {}: {}", url, err);
                writeln!(broken_links, "Failed to load {}", url)?;
                continue;
            }
        };

        // Download each file concurrently
        let mut handles = vec![];
        for &file in FILES {
            let file_url = format!("{}{}", url, file);
            let file_path = format!("reports/{}/{}", dir, file);
            let cookies_clone = cookies.clone();
            let mut broken_link_writer = broken_links.try_clone().expect("Failed to clone broken links file handle");

            handles.push(tokio::spawn(async move {
                info!("Downloading {} to {}", file_url, file_path);
                println!("Downloading {} to {}", file_url, file_path);

                if let Err(err) = download_file_directly(&cookies_clone, &file_url, &file_path).await {
                    error!("Failed to download {}: {}", file_url, err);
                    eprintln!("Failed to download {}: {}", file_url, err);
                    writeln!(broken_link_writer, "{}", file_url).unwrap();
                } else {
                    info!("Successfully downloaded {}", file_url);
                    println!("Successfully downloaded {}", file_url);
                }
            }));
        }

        // Wait for all download tasks to complete
        for handle in handles {
            handle.await?;
        }

        // Increment the month
        if month == 12 {
            month = 1;
            year += 1;
        } else {
            month += 1;
        }
    }

    info!("Finished processing");
    println!("Finished processing");

    Ok(())
}

fn preload_directory_page(browser: &Browser, url: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let tab = browser.new_tab()?;
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    let cookies = tab.get_cookies()?;
    let cookie_pairs: Vec<(String, String)> = cookies.iter()
        .map(|c| (c.name.clone(), c.value.clone()))
        .collect();

    Ok(cookie_pairs)
}

async fn download_file_directly(
    cookie_pairs: &[(String, String)],
    url: &str,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let jar = Jar::default();
    let parsed_url = Url::parse(url)?;

    // Setup the aggressive request headers to mimic a real browser
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36"));
    headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip, deflate, br"));
    headers.insert(header::CONNECTION, header::HeaderValue::from_static("keep-alive"));
    headers.insert(header::REFERER, header::HeaderValue::from_str(url).unwrap());
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"));

    for (name, value) in cookie_pairs {
        let cookie = format!("{}={}; Domain={}", name, value, parsed_url.domain().unwrap());
        jar.add_cookie_str(&cookie, &parsed_url);
    }

    let client = Client::builder()
        .default_headers(headers)
        .cookie_provider(Arc::new(jar))
        .build()?;

    let res = client.get(url).send().await?;
    let status = res.status();
    let bytes = res.bytes().await?;

    if status.is_success() {
        let dir_path = std::path::Path::new(path).parent().unwrap();
        create_dir_all(dir_path)?;
        let mut file = StdFile::create(path)?;
        file.write_all(&bytes)?;

        info!("File saved to {}", path);
        println!("File saved to {}", path);
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("HTTP request failed with status: {}", status),
        )));
    }

    Ok(())
}