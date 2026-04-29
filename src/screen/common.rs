use log::{info, error};
use base64::{Engine as _, engine::general_purpose};

use std::fs::File;
use std::io::Read;

fn imgcat(buffer: Vec<u8>, size_key: &str, size_value: usize) -> String {
    let e = general_purpose::STANDARD.encode(&buffer);
    return String::from(format!("\x1b]1337;File=inline=1;{size_key}={size_value};:{code}\x07", size_key = size_key, size_value = size_value, code = e));
}

pub fn imgcat_from_path(path: &str, width: usize) -> String {
    let mut f = match File::open(path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("fail to read image");

    return imgcat(buffer, &"width", width);
}

pub fn imgcat_from_url(url: &str, height: usize) -> Result<String, String> {
    use base64::{Engine as _, engine::general_purpose};

    info!("imgcat_from_url: Fetching image from URL: {}", url);

    let key = general_purpose::URL_SAFE.encode(url.as_bytes());
    let path = format!("data/cache/img/{file_name}", file_name = key);

    // Try to load from cache first
    match File::open(&path) {
        Ok(mut file) => {
            info!("imgcat_from_url: Using cached image: {}", path);
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("fail to read image");
            Ok(imgcat(buffer, &"height", height))
        },
        Err(_why) => {
            info!("imgcat_from_url: Not cached, fetching from network...");

            // Check if this is a LIHKG image and add required headers
            let is_lihkg_image = url.contains("lihkg.com") || url.contains("lih.kg");

            // Fetch from network
            let response = if is_lihkg_image {
                // Use LIHKG headers for LIHKG images
                info!("imgcat_from_url: Using LIHKG headers");
                let device_id = "89ee2a48214807d7762894d2ae9d07500e339171";
                let load_time = {
                    use std::time::SystemTime;
                    let now = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64();
                    let fractional = now.fract();
                    (fractional * 4.0) + 1.0
                };

                match reqwest::blocking::Client::new()
                    .get(url)
                    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
                    .header("Referer", "https://lihkg.com/")
                    .header("x-li-device-type", "browser")
                    .header("x-li-device", device_id)
                    .header("x-li-load-time", &load_time.to_string())
                    .send()
                {
                    Ok(resp) => resp,
                    Err(e) => {
                        error!("imgcat_from_url: Network error: {}", e);
                        return Err(format!("Network error: {}", e));
                    }
                }
            } else {
                // Basic fetch for other images (including HKGolden)
                match reqwest::blocking::get(url) {
                    Ok(resp) => resp,
                    Err(e) => {
                        error!("imgcat_from_url: Network error: {}", e);
                        return Err(format!("Network error: {}", e));
                    }
                }
            };

            if !response.status().is_success() {
                error!("imgcat_from_url: HTTP error {}", response.status());
                return Err(format!("HTTP {}", response.status()));
            }

            let image_data = match response.bytes() {
                Ok(data) => data.to_vec(),
                Err(e) => {
                    error!("imgcat_from_url: Failed to read response: {}", e);
                    return Err(format!("Failed to read response: {}", e));
                }
            };

            info!("imgcat_from_url: Downloaded {} bytes", image_data.len());

            // Save to cache
            if let Some(parent) = std::path::Path::new(&path).parent() {
                std::fs::create_dir_all(parent).unwrap_or_else(|_| {
                    error!("imgcat_from_url: Failed to create cache directory");
                });
            }

            match std::fs::write(&path, &image_data) {
                Ok(_) => info!("imgcat_from_url: Cached to {}", path),
                Err(e) => error!("imgcat_from_url: Failed to cache: {}", e),
            }

            Ok(imgcat(image_data, &"height", height))
        },
    }
}

pub fn reset_screen() {
    print!("{}{}{}", ::termion::clear::All, ::termion::style::Reset, ::termion::cursor::Show);
}

pub fn clear_screen () {
    print!("{}", ::termion::clear::All);
}
