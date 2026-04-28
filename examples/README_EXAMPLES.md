# LIHKG Debug Examples

This directory contains examples that demonstrate how the LIHKG API works and how to debug it.

## Available Examples

### 1. `debug_lihkg_response.rs`
Basic example that fetches and displays the raw LIHKG API response structure.

**Usage:**
```bash
cargo run --example debug_lihkg_response
```

**What it does:**
- Fetches threads from LIHKG category 1 (吹水台)
- Shows the full JSON response structure
- Displays all fields in the API response

**When to use:**
- Understanding LIHKG API response format
- Debugging parsing issues
- Learning the data structure

---

### 2. `debug_lihkg_index.rs` ⭐ **RECOMMENDED**
Comprehensive debug example that demonstrates the complete LIHKG index request with all proper headers.

**Usage:**
```bash
cargo run --example debug_lihkg_index
```

**What it does:**
- ✅ Shows exact headers required for LIHKG API
- ✅ Demonstrates device ID generation
- ✅ Shows load time calculation
- ✅ Displays request parameters and URL
- ✅ Parses and displays thread information
- ✅ Compares working vs failing requests
- ✅ Provides troubleshooting tips

**When to use:**
- Learning how LIHKG API works
- Understanding header requirements
- Debugging Cloudflare 1020 errors
- Implementing LIHKG integration in other projects

**Sample Output:**
```
🔍 Debugging LIHKG Index Request
═════════════════════════════════════════

📋 Request Parameters:
   Category ID: 1 (吹水台)
   Page: 1
   Count: 60 (max per page)
   Type: now (latest threads)

🔑 Generated Headers:
   x-li-device-type: browser
   x-li-device: 000000270000000018aa74317be62998
   x-li-load-time: 4.328252

📊 Response Status: 200 OK
✅ Success! LIHKG API responded successfully.

📦 Response Summary:
   Threads received: 35
   Category: 吹水台
```

---

### 3. `test_lihkg_api.rs`
Integration test using the actual repository implementation.

**Usage:**
```bash
cargo run --example test_lihkg_api
```

**What it does:**
- Uses the real `LihkgTopicRepository`
- Tests the complete integration
- Shows domain model conversion
- Demonstrates error handling

**When to use:**
- Testing the repository implementation
- Verifying end-to-end integration
- Checking domain model mapping

---

## Key Findings from These Examples

### The "Secret Sauce" - Critical Headers

Through Playwright investigation, we discovered that LIHKG requires these **mandatory headers**:

```rust
.header("x-li-device-type", "browser")        // ⭐ MOST IMPORTANT
.header("x-li-device", device_id)              // 32-char hex string
.header("x-li-load-time", load_time.to_string()) // 1.0-5.0 seconds
.header("Referer", "https://lihkg.com/category/1") // Must be lihkg.com
```

**Without these headers:** ❌ Cloudflare Error 1020
**With these headers:** ✅ Success 200 with real data

### Device ID Generation

The `x-li-device` header requires a 32-character hexadecimal string:

```rust
fn generate_device_id() -> String {
    // Create unique identifier based on time and process ID
    let input = format!("{}-{}-lihkg", timestamp, process_id());

    // Generate 32-char hex string (simulating MD5)
    format!("{:032x}", hash_value)
}
```

Example: `000000270000000018aa74317be62998`

### Load Time Calculation

The `x-li-load_time` header should be a realistic page load time:

```rust
fn calculate_load_time() -> f64 {
    // Generate value between 1.0-5.0 seconds
    let fractional = now_time.fract();
    (fractional * 4.0) + 1.0  // Range: 1.0 to 5.0
}
```

Example: `4.328252` (4.33 seconds)

---

## Common Issues and Solutions

### ❌ Error 1020 (Cloudflare)
**Cause:** Missing or incorrect headers
**Solution:** Ensure all critical headers are present
**Check:** Run `debug_lihkg_index` to see proper headers

### ❌ "missing field" parsing errors
**Cause:** API response structure changed
**Solution:** Run `debug_lihkg_response` to see current structure
**Check:** Compare actual vs expected field names

### ❌ Empty response
**Cause:** Invalid category ID or page number
**Solution:** Try category 1 (吹水台) with page 1
**Check:** Verify URL parameters are correct

---

## Using These Examples with the Main App

Once you understand how the LIHKG API works, you can use it in the main application:

```bash
# Use HKGolden (default)
./hkg --service hkgolden

# Use LIHKG (now working!)
./hkg --service lihkg
```

The multi-service architecture automatically uses the correct headers and parsing for each service.

---

## Running All Examples

Test all LIHKG examples:

```bash
# Basic response structure
cargo run --example debug_lihkg_response

# Comprehensive index debug (recommended)
cargo run --example debug_lihkg_index

# Integration test
cargo run --example test_lihkg_api
```

---

## Contributing

If you discover changes in the LIHKG API:

1. **Run the debug examples** to capture new behavior
2. **Update the repository implementation** if needed
3. **Test with the main app** to verify end-to-end functionality
4. **Document your findings** in this README

The examples are designed to be your first diagnostic tool when investigating API changes.

---

## Related Documentation

- `docs/LIHKG_API.md` - Complete LIHKG API documentation
- `docs/LIHKG_API_INVESTIGATION.md` - Playwright investigation results
- `docs/LIHKG_STATUS.md` - Current integration status
- `src/repository/lihkg.rs` - Repository implementation

---

## Credits

These examples were created through extensive Playwright investigation that revealed the exact headers and request patterns used by the LIHKG website. The investigation showed that the LIHKG API is fully accessible when proper headers are used.
