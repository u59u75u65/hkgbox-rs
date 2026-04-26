# Project Modernization - April 2026

## Overview

This document describes the comprehensive modernization of hkgbox-rs completed in April 2026, bringing the project from approximately 2016-2017 era dependencies to modern Rust standards.

## Problem Statement

The project was using extremely outdated dependencies that could not be built with modern Rust toolchains:

- **hyper 0.10** (current: 1.9+)
- **rustc-serialize 0.3** (deprecated, replaced by serde)
- **chrono 0.2** (current: 0.4+)
- **regex 0.1** (current: 1.11+)
- **termion 1.0** (current: 4.0+)
- **time 0.1** (current: 0.3+)

The project had been dormant since ~2017 and required complete dependency modernization to compile.

## Changes Made

### 1. Rust Edition Upgrade

**Before**: Implicit Rust 2015 edition
**After**: Rust 2021 edition

```toml
[package]
name = "hkg"
version = "1.0.2"
authors = ["u59u75u65"]
edition = "2021"  # ← Added
```

### 2. Dependency Updates

#### HTTP Client
| Old | New | Reason |
|-----|-----|--------|
| hyper 0.10 | reqwest 0.12 | hyper 1.x has async-only API; reqwest provides blocking API |
| hyper-native-tls | (removed) | reqwest includes TLS support |

#### Serialization
| Old | New | Reason |
|-----|-----|--------|
| rustc-serialize 0.3 | serde 1.0 | rustc-serialize deprecated in 2016 |
| (none) | serde_json 1.0 | Modern JSON serialization |
| rustc-serialize::base64 | base64 0.22 | Modern base64 encoding |

#### Other Dependencies
| Old | New | Reason |
|-----|-----|--------|
| chrono 0.2 | chrono 0.4 | API updates |
| regex 0.1 | regex 1.11 | Match API changes |
| termion 1.0 | termion 4.0 | API improvements |
| time 0.1 | time 0.3 | Complete API rewrite |
| cancellation 0.1 | (removed) | Abandoned crate; timeouts now in reqwest |
| crossbeam 0.2 | (removed) | No longer needed |

#### Logging
| Old | New | Reason |
|-----|-----|--------|
| log 0.3 | log 0.4 | Modern version |
| log4rs 0.6 | log4rs 1.3 | Major version update |

### 3. Code Changes

#### Module Imports (Rust 2021 Edition)

**Before:**
```rust
use status::*;
use model::ShowItem;
use screen::common::*;
```

**After:**
```rust
use crate::status::*;
use crate::model::ShowItem;
use crate::screen::common::*;
```

All module imports now require `crate::` prefix in Rust 2021 edition.

#### Serialization Derives

**Before:**
```rust
#[derive(RustcDecodable, RustcEncodable)]
pub struct ShowItem { ... }
```

**After:**
```rust
#[derive(Serialize, Deserialize)]
pub struct ShowItem { ... }
```

#### Base64 Encoding

**Before:**
```rust
use rustc_serialize::base64::{ToBase64};
let encoded = data.to_base64(base64::STANDARD);
```

**After:**
```rust
use base64::{Engine as _, engine::general_purpose};
let encoded = general_purpose::STANDARD.encode(&data);
```

#### HTTP Client

**Before (hyper 0.10):**
```rust
let client = Client::new();
client.set_read_timeout(Some(Duration::from_secs(5)));
let response = client.get(url).headers(headers).send()?;
```

**After (reqwest 0.12):**
```rust
let client = reqwest::blocking::Client::builder()
    .timeout(Duration::from_secs(5))
    .build()?;
let response = client.get(url)
    .header("User-Agent", "...")
    .send()?;
```

#### Regex Match API

**Before:**
```rust
let key = cap.name("key").unwrap_or("").to_string();
```

**After:**
```rust
let key = cap.name("key").map(|m| m.as_str()).unwrap_or("").to_string();
```

#### Time Formatting

**Before:**
```rust
let tm = time::now();
let formatted = tm.strftime("%Y%m%d%H%M")?.to_string();
```

**After:**
```rust
let now = time::OffsetDateTime::now_utc();
let formatted = format!("{:04}{:02}{:02}{:02}{:02}",
    now.year(), now.month() as u8, now.day(),
    now.hour(), now.minute());
```

#### Error Handling

**Before:**
```rust
let file = try!(File::open(path).map_err(|e| e.to_string()));
```

**After:**
```rust
let file = File::open(path).map_err(|e| e.to_string())?;
```

The `try!()` macro is deprecated; use `?` operator instead.

#### Cancellation Token Removal

**Before:**
```rust
let ct = CancellationTokenSource::new();
ct.cancel_after(Duration::new(10, 0));
ct.run(|| { ... }, || { ... });
```

**After:**
```rust
// Timeout now handled by reqwest
let client = reqwest::blocking::Client::builder()
    .timeout(Duration::from_secs(10))
    .build()?;
```

### 4. Files Modified

- **Cargo.toml** - Complete dependency rewrite
- **Cargo.lock** - Regenerated with modern dependencies
- **All 24 Rust source files** - Updated imports, APIs, and syntax

### 5. Build Results

**Before:**
```
error: failed to parse manifest
error[E0432]: unresolved imports (multiple)
error: could not compile
```

**After:**
```
✅ Successfully compiled
⚠️ 62 warnings (mostly unused variables - non-critical)
📦 Binary size: 23MB (debug)
```

## Migration Lessons

### What Worked Well

1. **Incremental approach**: Fixing errors one category at a time
2. **Script assistance**: Using sed scripts for repetitive changes
3. **Modern replacements**: reqwest instead of hyper for blocking API
4. **Edition upgrade**: Clean break from old patterns

### Challenges

1. **Breaking changes**: Every major dependency had API changes
2. **Import paths**: Rust 2021 requires explicit `crate::` prefix
3. **Deprecated macros**: `try!()` no longer available
4. **Time crate**: Complete API rewrite required new approach
5. **Regex API**: `Match` type now requires `.as_str()` call

### Dependencies Avoided

We intentionally **did not** update to these versions:
- **hyper 1.x**: Async-only API would require major architecture changes
- **tokio**: Would require converting entire app to async runtime

Instead, we chose **reqwest with blocking features** for minimal architectural changes while still getting modern HTTP client.

## Testing

The application has not been functionally tested post-modernization. The compilation succeeds, but runtime behavior should be verified:

- [ ] Terminal UI renders correctly
- [ ] Forum topics load successfully
- [ ] Navigation works as expected
- [ ] Image display functions
- [ ] Caching operates correctly

## Future Improvements

1. **API Migration**: Migrate to HKGolden's JSON API (see [API Migration Analysis](API-Migration-Analysis.md))
2. **Dependency Cleanup**: Remove kuchiki (HTML parser) after API migration
3. **Async Conversion**: Consider async/await for better concurrency
4. **Error Handling**: Implement proper error types instead of String
5. **Testing**: Add unit and integration tests
6. **Clippy**: Fix all clippy warnings

## Resources

- [Rust Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2021/index.html)
- [serde documentation](https://serde.rs/)
- [reqwest documentation](https://docs.rs/reqwest/)
- [From Rustc-Serialize to Serche](https://serde.rs/rustc-serialize.html)

## Commit

All changes committed as: `f343c3b25eeebf7faae64b61e155483d737f54a9`

```
Modernize dependencies and migrate to Rust 2021 edition

This project was using extremely outdated dependencies (hyper 0.10, rustc-serialize,
chrono 0.2, etc.) from ~2016-2017 that could not be built with modern Rust toolchains.

[... full commit message in git log ...]
```

---

**Last Updated**: 2026-04-26
**Modernized By**: Claude Sonnet 4.6 + martin
