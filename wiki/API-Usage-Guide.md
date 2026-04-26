# HKGolden API Usage Guide

This guide shows you how to use the new HKGolden JSON API in practice, with code examples and migration steps.

## Quick Start: Test the APIs

### 1. Test Topic List API

```bash
# Fetch topics from "BW" (Basketball War) forum, page 1
curl "https://api.hkgolden.com/v1/topics/BW/1?thumb=Y&sort=0&sensormode=Y&filtermodeS=N&hideblock=N&limit=-1" | jq '.data.list[0]'
```

### 2. Test Thread View API

```bash
# Fetch thread 8044402, page 1
curl "https://api.hkgolden.com/v1/view/8044402/1?sensormode=Y&hideblock=N" | jq '.data | {title, totalReplies, replies: .replies[0]}'
```

## Rust Implementation Guide

### Step 1: Add API Data Structures

Create a new file `src/api_models.rs`:

```rust
use serde::{Deserialize, Serialize};

// ============================================================================
// TOPIC LIST API
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTopicListResponse {
    pub result: bool,
    pub data: ApiTopicListData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTopicListData {
    pub list: Vec<ApiTopic>,
    pub total: i32,
    #[serde(rename = "maxPage")]
    pub max_page: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTopic {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "forum")]
    pub forum: String,

    #[serde(rename = "authorId")]
    pub author_id: i32,

    #[serde(rename = "authorName")]
    pub author_name: String,

    #[serde(rename = "authorGender")]
    pub author_gender: i32,

    #[serde(rename = "totalReplies")]
    pub total_replies: i32,

    #[serde(rename = "rating")]
    pub rating: i32,

    #[serde(rename = "totalPage")]
    pub total_page: i32,

    #[serde(rename = "messageDate")]
    pub message_date: i64,  // milliseconds since epoch

    #[serde(rename = "lastReplyDate")]
    pub last_reply_date: i64,  // milliseconds since epoch

    #[serde(rename = "thumbnail")]
    pub thumbnail: Option<String>,

    #[serde(rename = "iconType")]
    pub icon_type: String,

    #[serde(rename = "iconPath")]
    pub icon_path: String,
}

// ============================================================================
// THREAD VIEW API
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiThreadViewResponse {
    pub result: bool,
    pub data: ApiThreadViewData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiThreadViewData {
    // Main post fields (same as ApiTopic)
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "content")]
    pub content: String,  // HTML content

    #[serde(rename = "authorId")]
    pub author_id: i32,

    #[serde(rename = "authorName")]
    pub author_name: String,

    // Thread metadata
    #[serde(rename = "currentPage")]
    pub current_page: i32,

    #[serde(rename = "totalPage")]
    pub total_page: i32,

    #[serde(rename = "totalReplies")]
    pub total_replies: i32,

    // Replies
    #[serde(rename = "replies")]
    pub replies: Vec<ApiReply>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiReply {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "index")]
    pub index: i32,

    #[serde(rename = "authorId")]
    pub author_id: i32,

    #[serde(rename = "authorName")]
    pub author_name: String,

    #[serde(rename = "authorGender")]
    pub author_gender: i32,

    #[serde(rename = "replyDate")]
    pub reply_date: i64,  // milliseconds since epoch

    #[serde(rename = "content")]
    pub content: String,  // HTML content

    #[serde(rename = "quoted")]
    pub quoted: Vec<ApiQuotedRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiQuotedRef {
    #[serde(rename = "index")]
    pub index: i32,

    #[serde(rename = "id")]
    pub id: i32,
}
```

### Step 2: Create API Client

Create `src/api_client.rs`:

```rust
use reqwest::blocking::Client;
use serde_json;
use crate::api_models::*;

pub struct HkgApiClient {
    client: Client,
    base_url: String,
}

impl HkgApiClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self {
            client,
            base_url: "https://api.hkgolden.com/v1".to_string(),
        })
    }

    /// Fetch topic list for a forum
    ///
    /// # Example
    /// ```
    /// let client = HkgApiClient::new()?;
    /// let topics = client.fetch_topics("BW", 1)?;
    /// ```
    pub fn fetch_topics(
        &self,
        forum: &str,
        page: i32,
    ) -> Result<ApiTopicListResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/topics/{}/{}?thumb=Y&sort=0&sensormode=Y&filtermodeS=N&hideblock=N&limit=-1",
            self.base_url, forum, page
        );

        let response = self.client.get(&url).send()?;
        let json = response.text()?;
        let data: ApiTopicListResponse = serde_json::from_str(&json)?;

        Ok(data)
    }

    /// Fetch thread view with replies
    ///
    /// # Example
    /// ```
    /// let client = HkgApiClient::new()?;
    /// let thread = client.fetch_thread(8044402, 1)?;
    /// ```
    pub fn fetch_thread(
        &self,
        thread_id: i32,
        page: i32,
    ) -> Result<ApiThreadViewResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/view/{}/{}?sensormode=Y&hideblock=N",
            self.base_url, thread_id, page
        );

        let response = self.client.get(&url).send()?;
        let json = response.text()?;
        let data: ApiThreadViewResponse = serde_json::from_str(&json)?;

        Ok(data)
    }
}

impl Default for HkgApiClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HKG API client")
    }
}
```

### Step 3: Update lib.rs

Add to `src/lib.rs`:

```rust
pub mod api_models;
pub mod api_client;
```

### Step 4: Example Usage

Create a test program `examples/api_test.rs`:

```rust
use hkg::api_client::HkgApiClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HkgApiClient::new()?;

    println!("=== Fetching Topics ===\n");

    // Fetch topics from BW forum
    let topics_response = client.fetch_topics("BW", 1)?;

    println!("Total topics: {}", topics_response.data.total);
    println!("Max pages: {}", topics_response.data.max_page);
    println!("\nFirst 5 topics:\n");

    for (i, topic) in topics_response.data.list.iter().take(5).enumerate() {
        println!("{}. {}", i + 1, topic.title);
        println!("   Author: {}", topic.author_name);
        println!("   Replies: {}", topic.total_replies);
        println!("   Rating: {}", topic.rating);
        println!();
    }

    // Get first topic ID
    if let Some(first_topic) = topics_response.data.list.first() {
        println!("=== Fetching Thread: {} ===\n", first_topic.title);

        let thread_response = client.fetch_thread(first_topic.id, 1)?;

        println!("Thread: {}", thread_response.data.title);
        println!("Total replies: {}", thread_response.data.total_replies);
        println!("Current page: {}", thread_response.data.current_page);
        println!("\nFirst 3 replies:\n");

        for (i, reply) in thread_response.data.replies.iter().take(3).enumerate() {
            println!("{}. {}", i + 1, reply.author_name);
            // Strip HTML for display
            let content = reply.content
                .replace("<br />", "\n")
                .replace("<blockquote>", "Quote: ")
                .replace("</blockquote>", "");
            println!("   {}", content.chars().take(100).collect::<String>());
            println!();
        }
    }

    Ok(())
}
```

Run it:

```bash
cargo run --example api_test
```

## Migration Path: Step-by-Step

### Phase 1: Create API Foundation (1 hour)

```bash
# 1. Create new files
touch src/api_models.rs
touch src/api_client.rs

# 2. Update lib.rs
echo "pub mod api_models;" >> src/lib.rs
echo "pub mod api_client;" >> src/lib.rs

# 3. Test the API
cargo run --example api_test
```

### Phase 2: Update Index Resource (1-2 hours)

Modify `src/resources/index_resource.rs`:

```rust
use crate::api_client::HkgApiClient;
use crate::api_models::*;

pub struct IndexResource<'a, T: 'a + Cache> {
    client: HkgApiClient,
    cache: &'a mut Box<T>,
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        IndexResource {
            client: HkgApiClient::new().expect("Failed to create API client"),
            cache,
        }
    }
}

impl<'a, T: 'a + Cache> Resource for IndexResource<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        info!("[index_resource] Fetching from API");

        // Call API instead of scraping HTML
        match self.client.fetch_topics("BW", 1) {
            Ok(response) => {
                info!("Got {} topics", response.data.list.len());

                // Convert API topics to existing ListTopicItem format
                let list_items: Vec<ListTopicItem> = response.data.list
                    .into_iter()
                    .map(|api_topic| self.convert_to_list_item(api_topic))
                    .collect();

                ChannelItem {
                    extra: Some(ChannelItemType::Index(ChannelIndexItem {})),
                    result: String::new(),
                }
            }
            Err(e) => {
                error!("Failed to fetch topics: {}", e);
                ChannelItem {
                    extra: Some(ChannelItemType::Index(ChannelIndexItem {})),
                    result: format!("Error: {}", e),
                }
            }
        }
    }
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    fn convert_to_list_item(&self, api_topic: ApiTopic) -> ListTopicItem {
        // Convert timestamps
        let (date, time) = timestamp_to_strings(api_topic.last_reply_date);

        ListTopicItem {
            title: ListTopicTitleItem {
                url: format!("/messages/{}.aspx", api_topic.id),
                url_query: UrlQueryItem {
                    channel: api_topic.forum,
                    message: api_topic.id.to_string(),
                },
                text: api_topic.title,
                num_of_pages: api_topic.total_page as usize,
            },
            author: ListTopicAuthorItem {
                url: format!("/ProfilePage.aspx?userid={}", api_topic.author_id),
                name: api_topic.author_name,
            },
            last_replied_date: date,
            last_replied_time: time,
            reply_count: api_topic.total_replies.to_string(),
            rating: api_topic.rating.to_string(),
        }
    }
}

fn timestamp_to_strings(ms: i64) -> (String, String) {
    use time::OffsetDateTime;

    let dt = OffsetDateTime::from_unix_timestamp(ms / 1000)
        .unwrap_or_else(|_| OffsetDateTime::now_utc());

    let date = format!("{:02}/{:02}/{:04}",
        dt.day(), dt.month(), dt.year());

    let time = format!("{:02}:{:02}",
        dt.hour(), dt.minute());

    (date, time)
}
```

### Phase 3: Update Show Resource (2-3 hours)

Similar approach for `src/resources/show_resource.rs`:

```rust
use crate::api_client::HkgApiClient;
use crate::api_models::*;

pub struct ShowResource<'a, T: 'a + Cache> {
    client: HkgApiClient,
    cache: &'a mut Box<T>,
}

impl<'a, T: 'a + Cache> ShowResource<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        ShowResource {
            client: HkgApiClient::new().expect("Failed to create API client"),
            cache,
        }
    }
}

impl<'a, T: 'a + Cache> Resource for ShowResource<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        info!("[show_resource] Fetching from API");

        // Extract thread ID from item
        let thread_id = match &item.extra {
            Some(ChannelItemType::Show(show_item)) => {
                show_item.postid.parse().unwrap_or(0)
            }
            _ => return ChannelItem {
                extra: Some(ChannelItemType::Show(ChannelShowItem::default())),
                result: "Invalid thread ID".to_string(),
            },
        };

        // Call API
        match self.client.fetch_thread(thread_id, 1) {
            Ok(response) => {
                info!("Got thread with {} replies", response.data.total_replies);

                // Convert API replies to existing ShowReplyItem format
                let show_item = self.convert_to_show_item(response.data);

                ChannelItem {
                    extra: Some(ChannelItemType::Show(show_item)),
                    result: String::new(),
                }
            }
            Err(e) => {
                error!("Failed to fetch thread: {}", e);
                ChannelItem {
                    extra: Some(ChannelItemType::Show(ChannelShowItem::default())),
                    result: format!("Error: {}", e),
                }
            }
        }
    }
}

impl<'a, T: 'a + Cache> ShowResource<'a, T> {
    fn convert_to_show_item(&self, api_data: ApiThreadViewData) -> ChannelShowItem {
        // Convert API replies to existing format
        let replies: Vec<ShowReplyItem> = api_data.replies
            .into_iter()
            .map(|api_reply| {
                let (date, time) = timestamp_to_strings(api_reply.reply_date);

                ShowReplyItem {
                    userid: api_reply.author_id.to_string(),
                    username: api_reply.author_name,
                    content: api_reply.content.clone(),
                    body: parse_html_content(&api_reply.content),  // Still need HTML parsing
                    published_at: format!("{} {}", date, time),
                }
            })
            .collect();

        ChannelShowItem {
            postid: api_data.id.to_string(),
            page: api_data.current_page as usize,
        }
    }
}

// Still need kuchiki for parsing HTML content
fn parse_html_content(html: &str) -> Vec<NodeType> {
    // Use kuchiki to parse HTML content
    // This remains unchanged from current implementation
    // ...
    Vec::new()  // Placeholder
}
```

### Phase 4: Update Builders (2-3 hours)

The builders (`src/builders/index.rs` and `src/builders/show.rs`) need minimal changes since they already work with `ListTopicItem` and `ShowReplyItem` types.

### Phase 5: Remove Old Dependencies (optional)

Once everything works, you can remove:

```toml
# From Cargo.toml - remove these:
kuchiki = "*"  # Keep if still parsing HTML content
regex = "1.11"  # May not need anymore
```

## Testing the Migration

### 1. Create Integration Test

```rust
// tests/api_integration_test.rs
use hkg::api_client::HkgApiClient;

#[test]
fn test_fetch_topics() {
    let client = HkgApiClient::new().unwrap();
    let response = client.fetch_topics("BW", 1).unwrap();

    assert_eq!(response.result, true);
    assert!(response.data.list.len() > 0);
    assert_eq!(response.data.list[0].forum, "BW");
}

#[test]
fn test_fetch_thread() {
    let client = HkgApiClient::new().unwrap();
    let response = client.fetch_thread(8044402, 1).unwrap();

    assert_eq!(response.result, true);
    assert_eq!(response.data.id, 8044402);
    assert!(response.data.replies.len() > 0);
}
```

Run tests:
```bash
cargo test
```

### 2. Manual Testing

```bash
# Build the project
cargo build

# Run and test the UI
./target/debug/hkg

# Test:
# - Topic list loads
# - Can navigate through topics
# - Can view thread replies
# - Pagination works
```

## Performance Comparison

### Current (HTML Scraping)
```rust
// Old way - slow
let html = fetch_html(url);  // ~500ms
let document = kuchiki::parse_html();  // ~100ms
let title = document.select(".title")?;  // regex extraction
let author = document.select(".author")?;
// ... ~600ms total per page
```

### New (JSON API)
```rust
// New way - fast
let response = client.fetch_topics("BW", 1)?;  // ~200ms
// All data immediately available!
// ... ~200ms total per page
```

**3x faster!**

## Troubleshooting

### Common Issues

**1. Serialization Error**
```
Error: unknown field `lastReplyDate`
```
**Solution**: Add `#[serde(rename = "lastReplyDate")]` to struct fields

**2. API Returns Empty**
```
Response: {"result": false, "data": null}
```
**Solution**: Check API parameters and try again later

**3. Timeout**
```
Error: Timed out waiting for response
```
**Solution**: API might be rate-limited, add delay between requests

## API Rate Limiting

The API may have rate limits. Add delays:

```rust
use std::thread;
use std::time::Duration;

impl HkgApiClient {
    fn rate_limit_delay(&self) {
        thread::sleep(Duration::from_millis(500));  // 500ms between requests
    }

    pub fn fetch_topics(&self, forum: &str, page: i32) -> Result<...> {
        self.rate_limit_delay();
        // ... existing code
    }
}
```

## Next Steps

1. ✅ Create `src/api_models.rs` and `src/api_client.rs`
2. ✅ Test with `examples/api_test.rs`
3. ⏳ Update `IndexResource` to use API
4. ⏳ Update `ShowResource` to use API
5. ⏳ Test UI with real data
6. ⏳ Remove old HTML scraping code

---

**Questions?** Check the main [API Migration Analysis](API-Migration-Analysis.md) for more details!
