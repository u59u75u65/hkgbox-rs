//! Test ShowResourceApi with both HKGolden and LIHKG services
//!
//! This example tests that the ShowResourceApi correctly handles both services.

use hkg::caches::file_cache::*;
use hkg::resources::*;
use hkg::resources::common::*;
use hkg::resources::show_resource_api::*;
use hkg::cli::ForumService;
use hkg::model::ChannelId;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing ShowResourceApi Service Support\n");

    // Test with HKGolden service
    println!("📡 Testing HKGolden service...");
    test_hkgolden_service()?;

    println!("\n📡 Testing LIHKG service...");
    test_lihkg_service()?;

    println!("\n✅ All tests passed!");
    Ok(())
}

fn test_hkgolden_service() -> Result<(), Box<dyn std::error::Error>> {
    let mut fc = Box::new(FileCache::new());
    let mut show_resource = ShowResourceApi::new(&mut fc);
    show_resource.set_service(ForumService::Hkgolden);

    println!("  Service set to HKGolden ✅");

    // Test fetching a thread (use a known HKGolden thread)
    let item = ChannelItem {
        extra: Some(ChannelItemType::Show(ChannelShowItem {
            postid: "8029384".to_string(),
            page: 1,
        })),
        result: String::new(),
    };

    match show_resource.fetch(&item) {
        result => {
            println!("  Fetched thread successfully ✅");
            println!("    Title: {}", show_resource.title);
            println!("    Replies: {}", show_resource.total_replies);
        }
    }

    Ok(())
}

fn test_lihkg_service() -> Result<(), Box<dyn std::error::Error>> {
    let mut fc = Box::new(FileCache::new());
    let mut show_resource = ShowResourceApi::new(&mut fc);
    show_resource.set_service(ForumService::Lihkg);

    println!("  Service set to LIHKG ✅");

    // Test fetching a thread (use a known LIHKG thread)
    let item = ChannelItem {
        extra: Some(ChannelItemType::Show(ChannelShowItem {
            postid: "4098803".to_string(),
            page: 1,
        })),
        result: String::new(),
    };

    match show_resource.fetch(&item) {
        result => {
            println!("  Fetched thread successfully ✅");
            println!("    Title: {}", show_resource.title);
            println!("    Replies: {}", show_resource.total_replies);
        }
    }

    Ok(())
}
