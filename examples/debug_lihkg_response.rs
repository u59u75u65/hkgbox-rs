//! Debug LIHKG API response structure
//!
//! This example fetches the raw LIHKG API response and prints it to debug the structure.

use reqwest::blocking::Client;

fn generate_device_id() -> String {
    use std::time::SystemTime;
    use std::process;

    let input = format!("{}-{}-lihkg",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        process::id()
    );

    format!("{:016x}{:016x}",
        input.len() as u128,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u128 % 0xFFFFFFFFFFFFFFFF
    )
}

fn calculate_load_time() -> f64 {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    let fractional = now.fract();
    (fractional * 4.0) + 1.0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Debugging LIHKG API response structure...\n");

    let device_id = generate_device_id();
    let load_time = calculate_load_time();

    let url = "https://lihkg.com/api_v2/thread/latest?cat_id=1&page=1&count=5&type=now";

    println!("📡 Fetching: {}\n", url);

    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Referer", "https://lihkg.com/category/1")
        .header("x-li-device-type", "browser")
        .header("x-li-device", &device_id)
        .header("x-li-load-time", &load_time.to_string())
        .header("sec-ch-ua", r#""Chromium";v="147""#)
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", r#""macOS""#)
        .send()?;

    println!("Status: {}\n", response.status());

    let json: serde_json::Value = response.json()?;

    println!("📄 Full JSON response structure:\n");
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}
