# LIHKG API Documentation

This document provides comprehensive information about the LIHKG (LIHKG.com) API structure, endpoints, and how it differs from the HKGolden API.

## Overview

LIHKG (連登) is a popular Hong Kong forum platform. Its API structure differs significantly from HKGolden, requiring separate repository implementations.

## Base URL

```
https://lihkg.com/api_v2
```

## Authentication

Most public endpoints (thread listings, thread views) do not require authentication.
Private endpoints (posting, liking) would require session cookies/tokens.

## Key Differences from HKGolden API

### Response Wrapper

**LIHKG:**
```json
{
  "success": 1,
  "response": { ... }
}
```

**HKGolden:**
```json
{
  "result": true,
  "data": { ... }
}
```

### Thread Listing

**LIHKG:**
- Endpoint: `/thread/latest`
- Response structure: `response.items` array
- Thread ID field: `thread_id`
- User info: nested `user` object
- Category info: nested `category` object
- Timestamp: Unix seconds (not milliseconds)
- Ratings: `like_count` and `dislike_count` separate fields

**HKGolden:**
- Endpoint: `/view/{channel}/{page}`
- Response structure: `data.list` array
- Thread ID field: `id`
- User info: flat structure with `authorId`, `authorName`
- Channel: flat `channel` field
- Timestamp: milliseconds
- Rating: single `like` field

### Thread View

**LIHKG:**
- Endpoint: `/thread/{thread_id}/page/{page}?order=reply_time`
- Response structure: `response.thread` object
- Main content and replies in separate response structures
- User info nested in `user` object

**HKGolden:**
- Endpoint: `/view/{thread_id}/{page}`
- Response structure: `data` object
- Main content in `data`, replies in `data.replies`
- User info flat structure

## Endpoints

### 1. Latest Threads (Thread Listing)

Fetch the latest threads from a category.

**Endpoint:**
```
GET /thread/latest
```

**Query Parameters:**
- `cat_id` (integer, required): Category ID
  - `1`: 吹水台
  - Other categories available via system property API
- `page` (integer, required): Page number (1-indexed)
- `count` (integer, optional): Items per page (default: 60, max: 60)
- `type` (string, optional): Sort type
  - `now`: Latest (default)
  - `hot`: Hot topics

**Example Request:**
```
GET https://lihkg.com/api_v2/thread/latest?cat_id=1&page=1&count=60&type=now
```

**Response Structure:**
```json
{
  "success": 1,
  "response": {
    "items": [
      {
        "thread_id": 4098308,
        "title": "Thread title here",
        "user": {
          "user_id": 123456,
          "nickname": "Username",
          "gender": "M"
        },
        "category": {
          "cat_id": 1,
          "name": "吹水台"
        },
        "create_time": 1715865600,
        "reply_count": 42,
        "like_count": 10,
        "dislike_count": 2
      }
    ],
    "total_page": 10
  }
}
```

**Field Descriptions:**
- `thread_id`: Unique thread identifier
- `title`: Thread title
- `user.user_id`: Author's user ID
- `user.nickname`: Author's display name
- `user.gender`: Author's gender ("M", "F", or empty)
- `category.cat_id`: Category ID
- `category.name`: Category name (e.g., "吹水台")
- `create_time`: Unix timestamp in **seconds**
- `reply_count`: Number of replies
- `like_count`: Number of likes
- `dislike_count`: Number of dislikes
- `total_page`: Total number of pages

### 2. Thread View

Fetch a specific thread with replies.

**Endpoint:**
```
GET /thread/{thread_id}/page/{page}
```

**Path Parameters:**
- `thread_id` (integer, required): Thread ID
- `page` (integer, required): Page number (1-indexed)

**Query Parameters:**
- `order` (string, optional): Sort order for replies
  - `reply_time`: By reply time (default)
  - `like_time`: By like count

**Example Request:**
```
GET https://lihkg.com/api_v2/thread/4098308/page/1?order=reply_time
```

**Response Structure:**
```json
{
  "success": 1,
  "response": {
    "thread": {
      "thread_id": 4098308,
      "title": "Thread title",
      "user": {
        "user_id": 123456,
        "nickname": "OriginalPoster",
        "gender": "M"
      },
      "category": {
        "cat_id": 1,
        "name": "吹水台"
      },
      "create_time": 1715865600,
      "reply_count": 42,
      "like_count": 15,
      "dislike_count": 3,
      "msg": "<p>Thread content HTML</p>"
    },
    "total_page": 3
  }
}
```

**Note:** The actual thread view response structure may vary. The LIHKG API typically returns the main thread content and replies separately. Check the actual API response for the exact structure of reply items.

## Common Category IDs

- `1`: 吹水台
- Other categories can be fetched from the system property API

## Data Type Mapping

### Timestamp Conversion

LIHKG uses Unix timestamps in **seconds**, while HKGolden uses **milliseconds**.

**Conversion (Rust):**
```rust
// LIHKG to milliseconds
let timestamp_ms = lihkg_timestamp_seconds * 1000;

// Milliseconds to domain model
domain_topic.message_date = timestamp_ms;
```

### Rating Calculation

LIHKG provides separate `like_count` and `dislike_count` fields.

**Net rating calculation:**
```rust
let net_rating = (like_count - dislike_count).max(0);
```

### Rating Field

- **HKGolden**: Single `like` field (may be positive or negative)
- **LIHKG**: Separate `like_count` and `dislike_count` (calculate net)

## Implementation Notes

### Repository Implementation

The `LihkgTopicRepository` and `LihkgThreadRepository` in `src/repository/lihkg.rs` implement the `TopicRepository` and `ThreadRepository` traits, handling all LIHKG-specific conversions.

### Domain Model Mapping

**Topic Mapping:**
- `thread_id` → `Topic.id`
- `title` → `Topic.title`
- `category.name` → `Topic.forum`
- `user.user_id` → `Topic.author_id`
- `user.nickname` → `Topic.author_name`
- `reply_count` → `Topic.total_replies`
- `(like_count - dislike_count)` → `Topic.rating`
- `create_time * 1000` → `Topic.message_date`
- `total_page` → `Topic.total_page`

**Reply Mapping:**
- `post_id` → `Reply.id`
- `user.user_id` → `Reply.author_id`
- `user.nickname` → `Reply.author_name`
- `msg` → `Reply.content`
- `create_time * 1000` → `Reply.reply_date`

### Error Handling

The LIHKG API returns a `success` field:
- `1`: Success
- Other values: Error

Always check the `success` field before processing the response.

### Pagination

- Page numbers are **1-indexed** (first page is 1)
- Maximum items per page: 60
- Use `total_page` from response for pagination controls

## Testing

Example test usage:

```rust
use hkg::repository::LihkgTopicRepository;

#[tokio::test]
async fn test_lihkg_fetch() {
    let repo = LihkgTopicRepository::new(1).unwrap(); // 吹水台
    let (topics, max_page) = repo.fetch_topics("BW", 1).unwrap();

    assert!(!topics.is_empty());
    assert!(max_page > 0);

    let first = &topics[0];
    assert!(!first.title.is_empty());
    assert!(first.id > 0);
}
```

## References

- LIHKG API Base URL: https://lihkg.com/api_v2
- Example thread: https://lihkg.com/api_v2/thread/4098308/page/1?order=reply_time
- Category listing: https://lihkg.com/api_v2/system/property

## Future Enhancements

Potential improvements to the LIHKG implementation:

1. **Full Thread Reply Parsing**: Parse actual reply items from thread view response
2. **Category Discovery**: Fetch available categories from system property API
3. **Reply Indexing**: Properly assign reply indices based on position
4. **Quoted Reply Support**: Parse nested quoted reply structures
5. **Authentication**: Support for authenticated requests (posting, liking)
6. **Caching**: Add caching layer similar to HKGolden implementation
7. **Rate Limiting**: Implement proper rate limiting for API requests
