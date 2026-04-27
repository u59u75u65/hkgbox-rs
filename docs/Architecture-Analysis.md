# Architecture Analysis & Improvement Plan

## Executive Summary

This document outlines the current state of the hkgbox-rs codebase architecture and provides actionable recommendations for improvement. The analysis identifies critical issues with large files, tight coupling, and proposes a phased approach to refactoring.

**Current Status**: Functional but needs architectural improvements for long-term maintainability.

## Current Architecture Overview

### Architecture Patterns
The project implements a **Terminal User Interface (TUI)** application using:

- **Layered Architecture**: Data → Business Logic → Presentation
- **Controller Pattern**: Separate `control/` modules for user input handling
- **Repository Pattern**: `resources/` modules for data access and caching
- **Builder Pattern**: For complex object construction

### Technology Stack
- **Language**: Rust (Edition 2021)
- **Terminal UI**: termion 4.0
- **HTTP Client**: reqwest 0.12 (blocking)
- **HTML Parsing**: kuchiki 0.8
- **Concurrency**: std::thread + channels

## Critical Issues Identified

### 🚨 Extremely Large Files

| File | Lines | Issue |
|------|-------|-------|
| `src/api_utils.rs` | 19,078 | Critical - needs immediate splitting |
| `src/responser.rs` | 13,607 | Critical - multiple responsibilities |
| `src/main.rs` | 1,934 | High - too much application logic |

### Architectural Problems

1. **Tight Coupling**
   - `App` struct has too many direct dependencies
   - Screen modules tightly coupled to control modules
   - Direct state manipulation throughout codebase

2. **Missing Service Layer**
   - No clear business logic abstraction
   - Business logic scattered across controllers and resources
   - Difficult to test and maintain

3. **Inconsistent Organization**
   - Mixed naming conventions
   - Some utilities in `utility/`, others inline
   - Missing proper modularity for resource handling

## Proposed Architecture

### New Module Structure

```
src/
├── main.rs                 # Minimal entry point (<200 lines)
├── lib.rs                  # Library exports
│
├── config/                 # Application configuration
│   ├── mod.rs
│   ├── app_config.rs       # App configuration struct
│   └── constants.rs        # Application constants
│
├── domain/                 # Domain models and business logic
│   ├── mod.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── topic.rs        # Topic domain models
│   │   ├── channel.rs      # Channel domain models
│   │   └── user.rs         # User domain models
│   ├── services/
│   │   ├── mod.rs
│   │   ├── topic_service.rs
│   │   ├── channel_service.rs
│   │   └── image_service.rs
│   └── errors/
│       ├── mod.rs
│       ├── topic_error.rs
│       └── app_error.rs
│
├── infrastructure/         # External dependencies
│   ├── mod.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── client.rs       # API client
│   │   ├── models.rs       # API request/response models
│   │   └── utils.rs        # API utilities (refactored)
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── file_cache.rs
│   │   └── memory_cache.rs
│   ├── storage/
│   │   ├── mod.rs
│   │   └── repository.rs
│   └── terminal/
│       ├── mod.rs
│       └── io.rs
│
├── application/            # Application-specific logic
│   ├── mod.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── command.rs      # Command trait
│   │   ├── list_command.rs
│   │   ├── show_command.rs
│   │   └── channel_command.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── input_handler.rs
│   ├── state/
│   │   ├── mod.rs
│   │   └── state_manager.rs
│   └── events/
│       ├── mod.rs
│       └── event_bus.rs
│
├── presentation/           # UI layer
│   ├── mod.rs
│   ├── screens/
│   │   ├── mod.rs
│   │   ├── index.rs
│   │   ├── show.rs
│   │   └── dialog.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── header.rs
│   │   └── status_bar.rs
│   └── controls/
│       ├── mod.rs
│       ├── list_control.rs
│       ├── show_control.rs
│       └── dialog_control.rs
│
└── utils/                  # Shared utilities
    ├── mod.rs
    ├── string.rs
    └── conversion.rs
```

## Improvement Phases

### Phase 1: Quick Wins (Low Risk, High Impact)

**Timeline**: 1-2 weeks
**Risk**: Low
**Impact**: High

#### 1.1 Split `main.rs`
```rust
// Extract into separate modules:
mod application_loop;   // Main event loop logic
mod input_handler;      // Handle key input and thread management
mod screen_coordinator; // Screen rendering coordination
```

#### 1.2 Extract Configuration
```rust
// config/app_config.rs
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppConfig {
    pub api_url: String,
    pub cache_dir: PathBuf,
    pub max_concurrent_requests: usize,
    pub image_cache_size: usize,
    pub terminal_timeout: Duration,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_url: "https://api.hkgolden.com/v1".to_string(),
            cache_dir: PathBuf::from("data/cache"),
            max_concurrent_requests: 10,
            image_cache_size: 1000,
            terminal_timeout: Duration::from_secs(5),
        }
    }
}
```

#### 1.3 Domain-Specific Errors
```rust
// domain/errors/topic_error.rs
#[derive(Debug, thiserror::Error)]
pub enum TopicError {
    #[error("Failed to fetch topics: {0}")]
    FetchFailed(String),

    #[error("Invalid page number: {0}")]
    InvalidPage(usize),

    #[error("API returned error: {0}")]
    ApiError(String),

    #[error("Cache error: {0}")]
    CacheError(String),
}

// Type alias for convenience
pub type TopicResult<T> = Result<T, TopicError>;
```

#### 1.4 Improve Documentation
- Add rustdoc comments to all public APIs
- Document architectural decisions
- Add examples for complex functions

### Phase 2: Structural Improvements (Medium Risk, High Impact)

**Timeline**: 3-4 weeks
**Risk**: Medium
**Impact**: High

#### 2.1 Create Service Layer

```rust
// domain/services/topic_service.rs
pub struct TopicService {
    api_client: Arc<HkgApiClient>,
    cache: Arc<dyn Cache>,
}

impl TopicService {
    pub async fn fetch_topics(
        &self,
        channel: &str,
        page: usize,
    ) -> TopicResult<Vec<Topic>> {
        // Business logic for fetching topics
        // Includes caching, error handling, pagination
    }

    pub async fn search_topics(
        &self,
        query: &str,
        channel: &str,
    ) -> TopicResult<Vec<Topic>> {
        // Search logic
    }
}
```

#### 2.2 Implement Dependency Injection

```rust
// application/context.rs
pub struct ApplicationContext {
    pub config: Arc<AppConfig>,
    pub state: Arc<StateManager>,
    pub cache: Arc<dyn Cache>,
    pub api_client: Arc<HkgApiClient>,
    pub event_bus: Arc<EventBus>,
}

// Usage in screens
impl Index {
    pub fn new(ctx: Arc<ApplicationContext>) -> Self {
        Index {
            context: ctx,
            // ... other fields
        }
    }
}
```

#### 2.3 Refactor Large Files

**Split `api_utils.rs` (19,078 lines):**
```rust
// Create focused modules:
mod request_builder;   // Build API requests
mod response_parser;   // Parse API responses
mod timestamp_utils;   // Time conversion utilities
mod html_parser;       // HTML parsing helpers
```

**Split `responser.rs` (13,607 lines):**
```rust
mod response_processor; // Process incoming responses
mod state_updater;     // Update application state
mod image_handler;     // Handle image processing
mod cache_coordinator; // Coordinate caching
```

#### 2.4 Command Pattern for Input

```rust
// application/commands/mod.rs
pub trait Command {
    fn execute(&mut self, app: &mut App) -> Result<(), CommandError>;
}

// application/commands/navigation_command.rs
pub struct NavigationCommand {
    direction: NavigationDirection,
}

impl Command for NavigationCommand {
    fn execute(&mut self, app: &mut App) -> Result<(), CommandError> {
        match self.direction {
            NavigationDirection::Next => app.next_page(),
            NavigationDirection::Previous => app.previous_page(),
        }
        Ok(())
    }
}
```

### Phase 3: Advanced Improvements (High Risk, High Impact)

**Timeline**: 6-8 weeks
**Risk**: High
**Impact**: High

#### 3.1 Migrate to Async/Await

```rust
// Replace blocking I/O with async
use tokio::sync::mpsc;
use reqwest::Client; // async client

pub struct AsyncAppService {
    topic_service: TopicService,
    image_service: ImageService,
    state_manager: StateManager,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = AsyncAppService::new().await?;
    app.run().await?;
    Ok(())
}
```

#### 3.2 Event-Driven Architecture

```rust
// application/events/event_bus.rs
pub enum ApplicationEvent {
    TopicFetched(Vec<Topic>),
    ChannelChanged(String),
    ImageLoaded(Image),
    ErrorOccurred(AppError),
}

pub struct EventBus {
    subscribers: Vec<Box<dyn Fn(ApplicationEvent) + Send>>,
}

impl EventBus {
    pub fn subscribe(&mut self, handler: Box<dyn Fn(ApplicationEvent) + Send>) {
        self.subscribers.push(handler);
    }

    pub fn publish(&self, event: ApplicationEvent) {
        for subscriber in &self.subscribers {
            subscriber(event.clone());
        }
    }
}
```

#### 3.3 Advanced Caching

```rust
// infrastructure/cache/lru_cache.rs
use lru::LruCache;

pub struct LruImageCache {
    cache: Arc<Mutex<LruCache<String, Vec<u8>>>>,
    max_size: usize,
}

impl LruImageCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(max_size))),
            max_size,
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }

    pub fn put(&self, key: String, value: Vec<u8>) {
        let mut cache = self.cache.lock().unwrap();
        cache.put(key, value);
    }
}
```

#### 3.4 Comprehensive Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        HkgApiClient {}
        impl HkgApiClient for HkgApiClient {
            fn fetch_topics(&self, forum: &str, page: i32)
                -> Result<ApiTopicListResponse, Box<dyn std::error::Error>>;
        }
    }

    #[tokio::test]
    async fn test_fetch_topics_success() {
        let mut mock_client = MockHkgApiClient::new();
        mock_client
            .expect_fetch_topics()
            .returning(|_, _| Ok(mock_response()));

        let service = TopicService::new(Arc::new(mock_client));
        let result = service.fetch_topics("BW", 1).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 30);
    }
}
```

## Performance Improvements

### Current Issues
1. **Blocking I/O**: Blocks entire threads during HTTP requests
2. **Memory Usage**: Large data structures held in memory
3. **Manual Thread Management**: No async runtime
4. **No Cache Expiration**: Cache grows indefinitely

### Recommendations

#### Memory Management
```rust
// Implement pagination for large datasets
pub struct PaginatedResult<T> {
    items: Vec<T>,
    total: usize,
    page: usize,
    per_page: usize,
}

// Use weak references where appropriate
use std::sync::Weak;
```

#### Concurrency
```rust
// Replace manual threads with tokio
use tokio::task::JoinSet;

pub async fn fetch_multiple_pages(
    &self,
    channel: &str,
    pages: Vec<usize>,
) -> Result<Vec<Topic>, TopicError> {
    let mut join_set = JoinSet::new();

    for page in pages {
        let client = self.client.clone();
        let channel = channel.to_string();
        join_set.spawn(async move {
            client.fetch_topics(&channel, page as i32).await
        });
    }

    // Collect results
    let mut topics = Vec::new();
    while let Some(result) = join_set.join_next().await {
        topics.extend(result??);
    }

    Ok(topics)
}
```

## Migration Strategy

### Step-by-Step Approach

1. **Create New Structure Alongside Old**
   - Add new modules without removing old code
   - Gradually migrate functionality
   - Run both in parallel during transition

2. **Feature Flags**
   ```rust
   #[cfg(feature = "new_architecture")]
   use new::module::Path;

   #[cfg(not(feature = "new_architecture"))]
   use old::module::Path;
   ```

3. **Incremental Refactoring**
   - Start with least critical modules
   - Test thoroughly at each step
   - Maintain backward compatibility

4. **Testing Strategy**
   - Add integration tests before refactoring
   - Use property-based testing
   - Benchmark performance before/after

## Success Metrics

### Code Quality
- [ ] No file > 2000 lines
- [ ] 80%+ test coverage
- [ ] All public APIs documented
- [ ] Zero unsafe code without justification

### Performance
- [ ] < 100ms response time for page navigation
- [ ] < 500MB memory usage
- [ ] Support 100+ concurrent image downloads

### Maintainability
- [ ] New features can be added in < 1 day
- [ ] Onboarding time < 2 hours for new developers
- [ ] Zero circular dependencies

## Conclusion

This architecture improvement plan will transform hkgbox-rs from a functional but tightly-coupled application into a well-structured, maintainable, and performant TUI application. The phased approach minimizes risk while delivering continuous improvements.

**Next Steps**:
1. Review and approve this plan
2. Prioritize phases based on team capacity
3. Create detailed task breakdowns
4. Begin Phase 1 implementation

---

*Document Version: 1.0*
*Last Updated: 2026-04-27*
*Author: Architecture Analysis Team*
