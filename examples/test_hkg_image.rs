//! Test HKGolden image rendering

use hkg::image_utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing HKGolden Image Rendering\n");

    // Test with a common HKGolden image URL pattern
    let hkg_image_url = "https://www.hkgolden.com/images/logo.png";

    println!("📷 Testing HKGolden image URL:");
    println!("URL: {}\n", hkg_image_url);

    match image_utils::imgcat_from_url(hkg_image_url, 40) {
        Ok(imgcat_output) => {
            println!("✅ Successfully fetched and encoded image");
            println!("Output length: {} bytes", imgcat_output.len());
            println!("\nFirst 200 chars of output:");
            println!("{}\n", &imgcat_output.chars().take(200).collect::<String>());
        }
        Err(e) => {
            println!("❌ Failed to fetch image: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
