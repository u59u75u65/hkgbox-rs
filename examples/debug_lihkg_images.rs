//! Debug LIHKG thread images
//!
//! This example fetches a LIHKG thread and shows what image URLs are present,
//! helping us understand why images might fail to display.

use hkg::repository::{LihkgThreadRepository, ThreadRepository};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Debugging LIHKG Thread Images\n");

    // Create LIHKG thread repository
    let repo = LihkgThreadRepository::new()?;

    // Fetch a thread (using a sample thread ID)
    let thread_id = 4098863; // From earlier investigation
    println!("📡 Fetching thread {}...\n", thread_id);

    match repo.fetch_thread(thread_id, 1) {
        Ok(thread_view) => {
            println!("✅ Thread fetched successfully!\n");
            println!("📝 Thread: {}", thread_view.title);
            println!("📄 Content length: {} characters\n", thread_view.content.len());

            // Look for image URLs in the content
            println!("🖼️  Looking for image URLs in content...\n");

            let content = &thread_view.content;

            // Find all image tags
            let mut image_count = 0;
            let mut sample_urls = Vec::new();

            // Look for img tags
            for line in content.lines() {
                if line.contains("<img") {
                    image_count += 1;

                    // Extract src attribute
                    if let Some(start) = line.find("src=\"") {
                        if let Some(end) = line[start + 5..].find("\"") {
                            let url = &line[start + 5..start + 5 + end];
                            println!("   Found image: {}", url);

                            if sample_urls.len() < 3 {
                                sample_urls.push(url.to_string());
                            }
                        }
                    }
                }
            }

            println!("\n📊 Image Summary:");
            println!("   Total <img> tags found: {}", image_count);

            if !sample_urls.is_empty() {
                println!("\n🔍 Sample Image URLs:\n");
                for (i, url) in sample_urls.iter().enumerate() {
                    println!("   {}. {}", i + 1, url);

                    // Analyze the URL
                    println!("      Protocol: {}", if url.starts_with("https://") { "HTTPS" } else if url.starts_with("http://") { "HTTP" } else { "Other" });
                    println!("      Domain: {}", extract_domain(url));
                    println!("      Extension: {}", extract_extension(url));
                    println!();
                }
            } else {
                println!("\n   No images found in this thread.");
            }

            // Try to fetch one image to see what error we get
            if let Some(first_image) = sample_urls.first() {
                println!("🌐 Testing image fetch...\n");
                println!("   URL: {}", first_image);

                match test_image_fetch(first_image) {
                    Ok(_) => println!("   ✅ Image fetch successful!"),
                    Err(e) => println!("   ❌ Image fetch failed: {}", e),
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to fetch thread: {}\n", e);
            println!("🔧 This might be because:");
            println!("   1. Thread ID doesn't exist");
            println!("   2. LIHKG API is blocking requests");
            println!("   3. Network issues\n");
            return Err(Box::new(e) as Box<dyn std::error::Error>);
        }
    }

    Ok(())
}

fn extract_domain(url: &str) -> &str {
    if let Some(start) = url.find("://") {
        let after_protocol = &url[start + 3..];
        if let Some(end) = after_protocol.find('/') {
            &after_protocol[..end]
        } else {
            after_protocol
        }
    } else {
        "unknown"
    }
}

fn extract_extension(url: &str) -> &str {
    if let Some(dot) = url.rfind('.') {
        &url[dot..]
    } else {
        "(no extension)"
    }
}

fn test_image_fetch(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("   📡 Attempting basic fetch (no headers)...");

    let response = reqwest::blocking::get(url)?;
    println!("   Status: {}", response.status());

    if !response.status().is_success() {
        println!("   Trying with LIHKG headers...");

        // Try with LIHKG headers
        let device_id = generate_device_id();
        let load_time = calculate_load_time();

        let response = reqwest::blocking::Client::new()
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .header("Referer", "https://lihkg.com/")
            .header("x-li-device-type", "browser")
            .header("x-li-device", &device_id)
            .header("x-li-load-time", &load_time.to_string())
            .send()?;

        println!("   Status with headers: {}", response.status());

        if !response.status().is_success() {
            return Err(format!("HTTP {}", response.status()).into());
        }
    }

    Ok(())
}

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
