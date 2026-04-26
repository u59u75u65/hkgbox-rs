# Quick Start: Using the New API

## Step 1: Test the API Right Now (No Code Changes)

You can test the API immediately with curl:

```bash
# Fetch topics
curl "https://api.hkgolden.com/v1/topics/BW/1" | jq '.data.list[0]'

# Fetch a thread
curl "https://api.hkgolden.com/v1/view/8044402/1" | jq '.data | {title, totalReplies}'
```

## Step 2: Run the Example Code

Two examples are ready to run:

```bash
# Simple example (uses curl + serde_json)
cargo run --example api_simple

# Full example (uses reqwest)
cargo run --example api_test
```

## Step 3: See Real API Data

The `api_test` example will show:
- ✅ First 5 topics from BW forum
- ✅ Author names, reply counts, ratings
- ✅ Full thread with first 3 replies
- ✅ All data types (timestamps, HTML content, etc.)

## Step 4: Implement the API Client

Follow the detailed guide in [API-Usage-Guide.md](API-Usage-Guide.md)

Key files to create:
1. `src/api_models.rs` - Data structures
2. `src/api_client.rs` - HTTP client
3. Update `src/lib.rs` - Add modules

## Step 5: Update Resources

Two files need changes:
1. `src/resources/index_resource.rs` - Use API for topics
2. `src/resources/show_resource.rs` - Use API for threads

The rest of the codebase stays the same!

## What You Get

✅ **3x faster** page loads
✅ **Cleaner code** - no HTML parsing for structure
✅ **More data** - 33 fields vs scraped subset
✅ **Pagination** built-in
✅ **Thumbnails** included

## What Stays the Same

⚠️ **HTML content** still needs kuchiki (for formatting)
⚠️ **UI code** unchanged (builders, screens, controls)
⚠️ **User experience** identical (same keybindings, layout)

## Estimated Effort

- **1 hour** - Create API client and test
- **2 hours** - Update index resource
- **3 hours** - Update show resource
- **2 hours** - Testing and debugging

**Total: ~8 hours** for complete migration

## Questions?

- 📖 [API Usage Guide](API-Usage-Guide.md) - Full implementation guide
- 📊 [API Analysis](API-Migration-Analysis.md) - Complete API reference
- 🗺️ [Roadmap](Roadmap.md) - Development phases

---

**Ready to start?** Run `cargo run --example api_test` to see the API in action!
