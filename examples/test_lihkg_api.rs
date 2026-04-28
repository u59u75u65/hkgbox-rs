//! Test LIHKG API with proper headers
//!
//! This example demonstrates that the LIHKG API works with the correct headers
//! discovered through Playwright investigation.

use hkg::repository::{LihkgTopicRepository, TopicRepository};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing LIHKG API with proper headers...\n");

    // Create LIHKG repository for category 1 (吹水台)
    let repo = LihkgTopicRepository::new(1)?;

    println!("✅ Created LIHKG repository for category 1 (吹水台)\n");

    // Fetch topics from page 1
    println!("📡 Fetching topics from LIHKG API...\n");

    match repo.fetch_topics("BW", 1) {
        Ok((topics, max_page)) => {
            println!("🎉 SUCCESS! LIHKG API is working!\n");
            println!("📊 Results:");
            println!("   - Topics fetched: {}", topics.len());
            println!("   - Max pages: {}", max_page);
            println!("   - Total replies: {}", topics.iter().map(|t| t.total_replies).sum::<i32>());
            println!();

            // Show first 3 topics
            println!("📝 Sample topics:");
            for (i, topic) in topics.iter().take(3).enumerate() {
                println!("   {}. {}", i + 1, topic.title);
                println!("      作者: {} | 回覆: {} | 評分: {:?}",
                    topic.author_name,
                    topic.total_replies,
                    topic.rating
                );
                println!();
            }

            println!("✅ LIHKG API integration is working perfectly!");
            println!("   The proper headers (x-li-device-type, x-li-device, x-li-load-time)");
            println!("   discovered via Playwright investigation are the key!\n");

            Ok(())
        }
        Err(e) => {
            println!("❌ Error: {}\n", e);
            println!("🔧 Troubleshooting:");
            println!("   1. Check your internet connection");
            println!("   2. Verify LIHKG website is accessible");
            println!("   3. Try again in a few minutes (rate limiting?)");
            println!();
            Err(e.into())
        }
    }
}
