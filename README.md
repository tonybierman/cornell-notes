# Cornell Notes Library

A Rust library for reading, writing, and validating Cornell Notes in standardized JSON format, implementing the [Cornell Notes Data Schema RFC](https://github.com/tonybierman/cornell-notes-schema-rfc).

## Features

- **Type-safe data structures** for Cornell Notes components
- **Comprehensive validation** of all schema requirements
- **JSON serialization/deserialization** with serde
- **Markdown export** for easy reading and sharing
- **File I/O operations** for reading and writing notes
- **Support for structured content** (Markdown, HTML, plain text)
- **Cue-to-note linking** for relational study tools
- **UUID-based identification** for distributed creation
- **ISO 8601 datetime handling** with chrono

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cornell-notes = "0.1.0"
```

## Quick Start

```rust
use cornell_notes::{CornellNote, Section, Cue, Note, Content, CueType, NoteType};

// Create a new Cornell Note
let mut note = CornellNote::new("My Study Notes".to_string());

// Create a section
let mut section = Section::new(0);

// Add a cue (left column)
let cue = Cue::new(
    Content::Simple("What is Rust?".to_string()),
    CueType::Question,
    0,
);
section.add_cue(cue);

// Add a note (right column)
let note_entry = Note::new(
    Content::Simple("A systems programming language".to_string()),
    NoteType::Text,
    0,
);
section.add_note(note_entry);

// Add section to note
note.add_section(section);

// Add summary
note.update_summary(Content::Simple("Rust is great!".to_string()));

// Validate the note
note.validate()?;

// Save to file
cornell_notes::write_to_file(&note, "my_notes.json")?;

// Load from file
let loaded = cornell_notes::read_from_file("my_notes.json")?;

// Export to Markdown
let markdown = cornell_notes::to_markdown(&note);
println!("{}", markdown);

// Or save directly to file
cornell_notes::write_to_markdown_file(&note, "my_notes.md")?;
```

## Markdown Export

The library supports exporting Cornell Notes to Markdown format, following the RFC specification:

```rust
use cornell_notes::{CornellNote, to_markdown, write_to_markdown_file};

let note = CornellNote::new("My Study Notes".to_string());
// ... build your note ...

// Convert to Markdown string
let markdown = to_markdown(&note);

// Or write directly to file
write_to_markdown_file(&note, "output.md")?;
```

The markdown format organizes content hierarchically:

```markdown
# [Title]

**Subject:** [Subject]
**Topic:** [Topic]
**Author:** [Author]
**Tags:** [tag1, tag2, ...]

## Section 1

### Cues

- [Cue 1]
- [Cue 2]

### Notes

[Notes content]

## Section 2

### Cues

- [Cue 3]

### Notes

[Notes content]

## Summary

[Summary content]
```

This format is ideal for:
- **Reading**: Easy to read in any markdown viewer
- **Sharing**: Works on GitHub, GitLab, and other platforms
- **Editing**: Simple text format for quick edits
- **Printing**: Convert to PDF via markdown tools

## Core Types

### CornellNote
The main document structure containing:
- `version`: Schema version (e.g., "1.0")
- `metadata`: Document metadata (title, author, timestamps, etc.)
- `sections`: Array of note sections
- `summary`: Summary section

### Section
A logical grouping of cues and notes:
- `id`: UUID identifier
- `cues`: Array of cue entries (left column)
- `notes`: Array of note entries (right column)
- `position`: Order within the document

### Cue
A prompt or keyword in the left column:
- `id`: UUID identifier
- `content`: String or structured content
- `type`: CueType (Text, Question, Keyword, Custom)
- `position`: Order within section
- `created`, `modified`: Timestamps

### Note
A detailed note in the right column:
- `id`: UUID identifier
- `content`: String or structured content
- `type`: NoteType (Text, List, Code, Image, Link, Custom)
- `position`: Order within section
- `cue_links`: Optional links to related cues
- `created`, `modified`: Timestamps

### Content
Flexible content representation:
```rust
// Simple string content
Content::Simple("Plain text".to_string())

// Structured content with formatting
Content::Structured(StructuredContent {
    format: ContentFormat::Markdown,
    data: "# Header\n\nContent".to_string(),
    attachments: None,
})
```

## Validation

The library provides comprehensive validation:

```rust
use cornell_notes::CornellNote;

let note = CornellNote::new("Test".to_string());

match note.validate() {
    Ok(_) => println!("Note is valid"),
    Err(e) => println!("Validation error: {}", e),
}
```

Validation checks include:
- Version format matches `^[0-9]+\.[0-9]+$`
- Title is non-empty
- At least one section exists
- No duplicate IDs or positions
- Cue links reference existing cues
- Modified dates are not before created dates

## JSON Format

The library produces JSON conforming to the Cornell Notes schema:

```json
{
  "version": "1.0",
  "metadata": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Introduction to Rust",
    "created": "2025-01-06T10:00:00Z",
    "modified": "2025-01-06T10:00:00Z",
    "subject": "Computer Science",
    "topic": "Rust Programming",
    "author": "Student Name",
    "tags": ["programming", "rust"]
  },
  "sections": [
    {
      "id": "650e8400-e29b-41d4-a716-446655440000",
      "position": 0,
      "cues": [
        {
          "id": "750e8400-e29b-41d4-a716-446655440000",
          "content": "What is Rust?",
          "type": "question",
          "position": 0,
          "created": "2025-01-06T10:00:00Z",
          "modified": "2025-01-06T10:00:00Z"
        }
      ],
      "notes": [
        {
          "id": "850e8400-e29b-41d4-a716-446655440000",
          "content": "A systems programming language",
          "type": "text",
          "position": 0,
          "created": "2025-01-06T10:00:00Z",
          "modified": "2025-01-06T10:00:00Z",
          "cueLinks": ["750e8400-e29b-41d4-a716-446655440000"]
        }
      ]
    }
  ],
  "summary": {
    "content": "Rust is a modern systems language",
    "created": "2025-01-06T10:00:00Z",
    "modified": "2025-01-06T10:00:00Z"
  }
}
```

## Examples

Run the basic usage example:

```bash
cargo run --example basic_usage
```

This will create an example note and save it to `example_note.json`.

## API Reference

### Top-level Functions

#### JSON Operations
- `from_json(json: &str) -> Result<CornellNote>` - Parse and validate JSON
- `from_json_unchecked(json: &str) -> Result<CornellNote>` - Parse without validation
- `to_json(note: &CornellNote) -> Result<String>` - Serialize to compact JSON
- `to_json_pretty(note: &CornellNote) -> Result<String>` - Serialize to formatted JSON
- `read_from_file(path: &str) -> Result<CornellNote>` - Read and validate from file
- `write_to_file(note: &CornellNote, path: &str) -> Result<()>` - Write to file

#### Markdown Export
- `to_markdown(note: &CornellNote) -> String` - Export to Markdown string
- `write_to_markdown_file(note: &CornellNote, path: &str) -> Result<()>` - Write to Markdown file

### Error Handling

All operations return `cornell_notes::Result<T>` which uses the `CornellNoteError` enum:

- `JsonError` - JSON parsing errors
- `ValidationError` - Schema validation failures
- `IoError` - File I/O errors
- `InvalidVersion` - Version format errors
- `InvalidUuid` - UUID format errors
- `InvalidDatetime` - Datetime format errors

## Testing

Run the test suite:

```bash
cargo test
```

The library includes comprehensive tests for:
- Structure creation and manipulation
- Validation logic
- JSON serialization/deserialization
- File I/O operations
- Error conditions

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## References

- [Cornell Notes Data Schema RFC](https://github.com/tonybierman/cornell-notes-schema-rfc)
- [Cornell Note-taking System](https://en.wikipedia.org/wiki/Cornell_Notes)
