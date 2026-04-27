# Documentation

This directory contains architectural documentation and planning materials for the hkgbox-rs project.

## Documents

### [Architecture-Analysis.md](Architecture-Analysis.md)
Comprehensive analysis of the current codebase architecture and improvement roadmap.

**Contents:**
- Current architecture overview
- Critical issues and technical debt
- Proposed module reorganization
- Three-phase improvement plan
- Performance optimization strategies
- Migration guidelines

### Quick Reference

#### Current Issues Summary
- **Large Files**: `api_utils.rs` (19K lines), `responser.rs` (13K lines)
- **Tight Coupling**: Direct dependencies throughout
- **Missing Service Layer**: Business logic scattered

#### Immediate Actions
1. Split `main.rs` into logical modules
2. Extract configuration to dedicated module
3. Add domain-specific error types
4. Create service layer for business logic

#### Long-term Goals
1. Migrate to async/await runtime
2. Implement event-driven architecture
3. Add comprehensive testing
4. Optimize memory and performance

## Contributing

When making architectural changes:
1. Update this documentation first
2. Create RFC for major changes
3. Get team approval before implementation
4. Update docs after implementation

## Related Resources

- [Wiki](../wiki/) - User-facing documentation
- [Cargo.toml](../Cargo.toml) - Dependencies and metadata
- [src/](../src/) - Source code
