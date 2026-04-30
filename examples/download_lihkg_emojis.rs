//! Download LIHKG emojis and convert to local icon format
//!
//! This script downloads all LIHKG emoji stickers from the CDN
//! and converts them to the local icon format used by the app

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use serde_json;

#[derive(Debug, serde::Deserialize)]
struct EmojiCategory {
    key: String,
    name: String,
    thumbnail: String,
    stickers: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Downloading LIHKG Emojis\n");

    // Read the emoji JSON file
    let json_path = "wiki/lihkg_emoji.json";
    println!("📂 Reading emoji data from {}...\n", json_path);

    let json_content = fs::read_to_string(json_path)?;
    let categories: Vec<EmojiCategory> = serde_json::from_str(&json_content)?;

    println!("✅ Found {} emoji categories\n", categories.len());

    // Create output directory
    let output_dir = "data/icon/lihkg";
    fs::create_dir_all(output_dir)?;

    println!("📥 Downloading emojis to {}...\n", output_dir);

    let mut icon_map = HashMap::new();
    let mut total_downloaded = 0;

    for category in &categories {
        println!("📁 Category: {} ({})", category.name, category.key);

        for sticker_url in &category.stickers {
            // Extract filename from URL
            // Format: https://cdn.lihkg.com/upload/stickers/{category}/{filename}.png
            if let Some(filename) = sticker_url.split('/').last() {
                // Remove extension to get emoji name
                let emoji_name = filename.trim_end_matches(".png");

                // Generate local filename
                let local_filename = format!("lihkg_{}_{}.png", category.key, emoji_name);
                let local_path = format!("{}/{}", output_dir, local_filename);

                // Download the emoji
                match download_emoji(sticker_url, &local_path) {
                    Ok(_) => {
                        total_downloaded += 1;
                        // Map the old filename to local path
                        icon_map.insert(
                            format!("/assets/faces/{}/{}.gif", category.key, emoji_name),
                            local_filename.clone()
                        );

                        // Also map the PNG version
                        icon_map.insert(
                            format!("/assets/stickers/{}/{}.png", category.key, emoji_name),
                            local_filename.clone()
                        );
                    }
                    Err(e) => {
                        println!("  ❌ Failed to download {}: {}", sticker_url, e);
                    }
                }
            }
        }

        println!();
    }

    println!("✅ Downloaded {} emojis total\n", total_downloaded);

    // Generate icon collection mapping
    println!("📝 Generating icon collection...\n");

    // Read existing icon collection if it exists
    let existing_icons = if Path::new("data/icon/icons.json").exists() {
        let json_content = fs::read_to_string("data/icon/icons.json")?;
        serde_json::from_str::<HashMap<String, String>>(&json_content)?
    } else {
        HashMap::new()
    };

    // Merge new icons
    let mut all_icons = existing_icons.clone();
    for (key, value) in icon_map {
        all_icons.insert(key, value);
    }

    // Write updated icon collection
    let icons_json = serde_json::to_string_pretty(&all_icons)?;
    fs::write("data/icon/lihkg_icons.json", &icons_json)?;

    println!("✅ Generated lihkg_icons.json with {} mappings\n", all_icons.len());

    Ok(())
}

fn download_emoji(url: &str, local_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Use LIHKG headers for CDN requests
    let response = reqwest::blocking::Client::new()
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .header("Referer", "https://lihkg.com/")
        .send()?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()).into());
    }

    let image_data = response.bytes()?.to_vec();

    // Write to file
    let mut file = File::create(local_path)?;
    file.write_all(&image_data)?;

    println!("  ✓ Downloaded: {}", local_path);

    Ok(())
}
