// Example: Parse HTML content and display images using imgcat
//
// Run with: cargo run --example html_parser

use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== HTML Content Parser with imgcat ===\n");

    // Fetch the thread data
    let url = "https://api.hkgolden.com/v1/view/8045388/1?sensormode=Y&hideblock=N";

    println!("📡 Fetching thread from API...\n");

    let output = Command::new("curl")
        .arg("-s")
        .arg(url)
        .output()?;

    if !output.status.success() {
        println!("❌ Failed to fetch data");
        return Ok(());
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&json_str)?;

    // Get the HTML content
    let content = json["data"]["content"]
        .as_str()
        .unwrap_or("");

    println!("📝 Content preview (first 200 chars):\n");
    println!("{}...\n", content.chars().take(200).collect::<String>());

    // Parse HTML to extract image URLs using regex
    println!("🖼️  Extracting image URLs...\n");

    let image_urls = extract_image_urls_regex(content);

    if image_urls.is_empty() {
        println!("❌ No images found in content");
        return Ok(());
    }

    println!("✅ Found {} image(s):\n", image_urls.len());

    for (i, img_url) in image_urls.iter().enumerate() {
        println!("{}. {}", i + 1, truncate_url(img_url));
        println!();
    }

    // Now fetch and display images using imgcat
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Displaying images using imgcat...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    for (i, img_url) in image_urls.iter().enumerate() {
        println!("Image {} of {}:", i + 1, image_urls.len());
        println!("URL: {}\n", truncate_url(img_url));

        match display_image(img_url) {
            Ok(_) => println!("\n✅ Image displayed successfully\n"),
            Err(e) => println!("\n❌ Failed to display: {}\n", e),
        }

        // Add delay between images
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

/// Extract image URLs from HTML content using regex
fn extract_image_urls_regex(html: &str) -> Vec<String> {
    use regex::Regex;

    let mut urls = Vec::new();

    // Pattern to match <img src="..." />
    let img_regex = Regex::new(r#"<img[^>]+src="([^"]+)""#).unwrap();

    for capture in img_regex.captures_iter(html) {
        if let Some(url_match) = capture.get(1) {
            let url = url_match.as_str().to_string();
            urls.push(url);
        }
    }

    urls
}

/// Display an image using imgcat
fn display_image(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    // Generate cache filename from URL
    use base64::{Engine as _, engine::general_purpose};
    let cache_key = general_purpose::URL_SAFE.encode(url.as_bytes());
    let cache_path = format!("data/cache/img/{}", cache_key);

    // Check if image is already cached
    if let Some(cached_image) = display_from_cache(&cache_path) {
        print!("{}", cached_image);
        std::io::stdout().flush()?;
        return Ok(());
    }

    // Fetch the image
    print!("⬇️  Fetching...");
    std::io::stdout().flush()?;

    let response = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?
        .get(url)
        .send()?;

    if !response.status().is_success() {
        println!(" Failed! HTTP {}", response.status());
        return Err(format!("HTTP {}", response.status()).into());
    }

    let image_data = response.bytes()?.to_vec();

    println!(" ✓ ({} bytes)", image_data.len());

    // Save to cache
    if let Some(parent) = std::path::Path::new(&cache_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&cache_path, &image_data)?;

    // Display using imgcat
    let imgcat_output = imgcat_from_data(&image_data, 80);
    print!("{}", imgcat_output);
    std::io::stdout().flush()?;

    Ok(())
}

/// Display image from cache
fn display_from_cache(path: &str) -> Option<String> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;

    println!("📁 From cache");

    Some(imgcat_from_data(&buffer, 80))
}

/// Create imgcat escape sequence from image data
fn imgcat_from_data(data: &[u8], width: usize) -> String {
    use base64::{Engine as _, engine::general_purpose};

    let encoded = general_purpose::STANDARD.encode(data);
    format!("\x1b]1337;File=inline=1;width={};content:{};\x07", width, encoded)
}

/// Truncate URL for display
fn truncate_url(url: &str) -> String {
    if url.len() > 70 {
        format!("{}...", &url[..70])
    } else {
        url.to_string()
    }
}
