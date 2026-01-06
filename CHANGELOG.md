# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-06

### Fixed
- Fixed Windows compatibility in `test_markdown_file_write` by using `std::env::temp_dir()` instead of hardcoded `/tmp` path

### Added
- Initial release of cornell-notes library
- Core data structures for Cornell Notes (CornellNote, Section, Cue, Note, Summary)
- Comprehensive validation system
  - Version format validation
  - UUID format validation
  - Timestamp consistency checks
  - Duplicate ID and position detection
  - Cue link integrity verification
- JSON serialization and deserialization
  - `from_json()` - Parse and validate JSON
  - `from_json_unchecked()` - Parse without validation
  - `to_json()` - Serialize to compact JSON
  - `to_json_pretty()` - Serialize to formatted JSON
  - `read_from_file()` - Read from JSON file
  - `write_to_file()` - Write to JSON file
- Markdown export functionality
  - `to_markdown()` - Export to Markdown string
  - `write_to_markdown_file()` - Write to Markdown file
- Support for multiple content types (Text, List, Code, Image, Link, Custom)
- Support for multiple cue types (Text, Question, Keyword, Custom)
- Structured content support with Markdown, HTML, and plain text formats
- Comprehensive test suite with 13 tests
- Example application demonstrating basic usage
- Complete documentation with README and inline docs

### Features
- Type-safe Rust implementation
- UUID-based identification using uuid crate
- ISO 8601 datetime handling with chrono
- Flexible content representation (simple strings or structured objects)
- Cue-to-note linking for relational organization
- Position-based ordering for sections, cues, and notes
- Optional metadata fields (subject, topic, author, tags)

[0.1.0]: https://github.com/tonybierman/cornell-notes/releases/tag/v0.1.0
