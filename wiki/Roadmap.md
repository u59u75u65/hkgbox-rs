# hkgbox-rs Development Roadmap

## Current Status

✅ **Phase 1 Complete**: Project Modernization (April 2026)
- Updated to Rust 2021 edition
- All dependencies modernized
- Successfully compiles with Rust 1.95+

🚧 **Phase 2 In Progress**: API Migration Planning
- JSON API analysis complete
- Migration strategy documented
- Implementation pending

## Development Phases

### Phase 1: Foundation ✅ (COMPLETE)
**Status**: Completed April 26, 2026

**Goals**:
- [x] Update to Rust 2021 edition
- [x] Modernize all dependencies
- [x] Fix compilation errors
- [x] Ensure project builds successfully

**Details**: See [Project Modernization 2026](Project-Modernization-2026.md)

---

### Phase 2: API Migration 🚧 (PLANNED)
**Status**: Analysis complete, implementation pending

**Goals**:
- [ ] Migrate from HTML scraping to JSON API
- [ ] Remove kuchiki HTML parser dependency
- [ ] Improve performance and reliability
- [ ] Add better error handling

**Sub-phases**:

#### 2.1 Data Models (1-2 hours)
- [ ] Create API response structs
- [ ] Add serde derives
- [ ] Implement conversion functions
- [ ] Add timestamp utilities

#### 2.2 HTTP Client (1 hour)
- [ ] Update WebResource for JSON
- [ ] Replace HTML fetching
- [ ] Update error handling

#### 2.3 Resource Layer (2-3 hours)
- [ ] Refactor IndexResource
- [ ] Refactor ShowResource (if API available)
- [ ] Remove HTML parsing
- [ ] Update caching

#### 2.4 Builder Layer (2-3 hours)
- [ ] Update index builder
- [ ] Remove regex patterns
- [ ] Simplify data flow

#### 2.5 Testing (2-3 hours)
- [ ] Unit tests for models
- [ ] Integration tests
- [ ] Manual UI testing
- [ ] Performance benchmarks

**Details**: See [API Migration Analysis](API-Migration-Analysis.md)

---

### Phase 3: Testing & Quality 📋 (PLANNED)
**Status**: Not started

**Goals**:
- [ ] Add comprehensive test suite
- [ ] Set up CI/CD pipeline
- [ ] Add benchmarking
- [ ] Document test coverage

**Tasks**:
- [ ] Unit tests for data models
- [ ] Integration tests for API calls
- [ ] UI component tests
- [ ] Performance regression tests
- [ ] GitHub Actions workflow

---

### Phase 4: Feature Enhancements 💡 (PLANNED)
**Status**: Not started

**Potential Features**:
- [ ] Configurable keybindings
- [ ] Theme customization
- [ ] Multiple forum support
- [ ] Search functionality
- [ ] Bookmark management
- [ ] Offline mode (cached browsing)
- [ ] Image preview improvements
- [ ] Notification support

**Community Requests**:
- [ ] Linux support (currently macOS-only)
- [ ] Cross-platform terminal support
- [ ] Touch gesture support

---

### Phase 5: Architecture Improvements 🏗️ (PLANNED)
**Status**: Not started

**Goals**:
- [ ] Better error handling (custom error types)
- [ ] Configuration file support
- [ ] Plugin system
- [ ] State management improvements

**Technical Debt**:
- [ ] Replace String errors with proper error types
- [ ] Reduce unwrap() calls
- [ ] Improve logging strategy
- [ ] Optimize memory usage

---

## Release Planning

### v1.1.0 - API Migration
**Target**: Q2 2026
**Focus**: JSON API integration

**Features**:
- Migrate to HKGolden JSON API
- Remove HTML scraping
- Performance improvements
- Better error messages

### v1.2.0 - Quality & Stability
**Target**: Q3 2026
**Focus**: Testing and bug fixes

**Features**:
- Comprehensive test suite
- CI/CD pipeline
- Documentation improvements
- Bug fixes

### v1.3.0 - Feature Enhancements
**Target**: Q4 2026
**Focus**: User experience

**Features**:
- Configurable keybindings
- Theme support
- Linux support
- Enhanced search

### v2.0.0 - Major Rewrite
**Target**: 2027
**Focus**: Modern architecture

**Features**:
- Async/await implementation
- Modern UI framework
- Plugin system
- Cross-platform support

---

## Immediate Priorities (Next 2 Weeks)

1. **API Migration Proof of Concept**
   - Fetch topics from JSON API
   - Display in terminal UI
   - Verify performance gains

2. **Show Thread API Investigation**
   - Find replies endpoint
   - Document response format
   - Plan migration strategy

3. **Basic Testing**
   - Manual testing of current build
   - Verify all features work
   - Document any issues

---

## Long-term Vision

**Goal**: Make hkgbox-rs the best terminal-based forum client

**Objectives**:
- 🚀 Fast and responsive
- 🎨 Customizable appearance
- 🔧 Configurable behavior
- 📱 Cross-platform
- 🧪 Well-tested
- 📚 Well-documented
- 🤝 Community-driven

**Success Metrics**:
- Active users
- Community contributions
- Bug reports/fixes ratio
- Feature requests
- Performance benchmarks

---

## Contributing

We welcome contributions! Areas where help is needed:

1. **Testing**: Manual testing on different systems
2. **Documentation**: Improving wiki and code comments
3. **Features**: Implementing planned features
4. **Bug Fixes**: Squashing bugs
5. **Performance**: Optimizing critical paths

See [Home](Home.md) for project overview.

---

**Last Updated**: 2026-04-26
**Maintainer**: martin + community contributors
