//! # Cornell Notes Library
//!
//! A Rust library for reading, writing, and validating Cornell Notes in standardized JSON format.
//!
//! This library implements the Cornell Notes Data Schema RFC, providing type-safe structures
//! and validation for the three-section note-taking system.
//!
//! ## Example
//!
//! ```no_run
//! use cornell_notes::{CornellNote, Metadata};
//! use std::fs;
//!
//! // Read a Cornell Note from file
//! let json = fs::read_to_string("note.json").unwrap();
//! let note = cornell_notes::from_json(&json).unwrap();
//!
//! // Validate the note
//! note.validate().unwrap();
//!
//! // Write back to JSON
//! let output = cornell_notes::to_json(&note).unwrap();
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur when working with Cornell Notes
#[derive(Error, Debug)]
pub enum CornellNoteError {
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid version format: {0}")]
    InvalidVersion(String),

    #[error("Invalid UUID: {0}")]
    InvalidUuid(String),

    #[error("Invalid datetime: {0}")]
    InvalidDatetime(String),
}

pub type Result<T> = std::result::Result<T, CornellNoteError>;

/// Type of cue entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CueType {
    Text,
    Question,
    Keyword,
    Custom,
}

/// Type of note entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NoteType {
    Text,
    List,
    Code,
    Image,
    Link,
    Custom,
}

/// Content format for structured content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContentFormat {
    Markdown,
    Html,
    Plain,
}

/// Attachment metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attachment {
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Structured content with format and optional attachments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructuredContent {
    pub format: ContentFormat,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
}

/// Content can be either a simple string or structured object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Content {
    Simple(String),
    Structured(StructuredContent),
}

/// Metadata for the Cornell Note document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    pub id: Uuid,
    pub title: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// A cue entry in the left column
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cue {
    pub id: Uuid,
    pub content: Content,
    #[serde(rename = "type")]
    pub cue_type: CueType,
    pub position: u32,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

/// A note entry in the right column
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: Uuid,
    pub content: Content,
    #[serde(rename = "type")]
    pub note_type: NoteType,
    pub position: u32,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cueLinks")]
    pub cue_links: Option<Vec<Uuid>>,
}

/// A section containing cues and notes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    pub id: Uuid,
    pub cues: Vec<Cue>,
    pub notes: Vec<Note>,
    pub position: u32,
}

/// Summary section at the bottom
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Summary {
    pub content: Content,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

/// Complete Cornell Note document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CornellNote {
    pub version: String,
    pub metadata: Metadata,
    pub sections: Vec<Section>,
    pub summary: Summary,
}

impl Metadata {
    /// Validate metadata fields
    pub fn validate(&self) -> Result<()> {
        if self.title.is_empty() {
            return Err(CornellNoteError::ValidationError(
                "Title cannot be empty".to_string(),
            ));
        }

        if self.modified < self.created {
            return Err(CornellNoteError::ValidationError(
                "Modified date cannot be before created date".to_string(),
            ));
        }

        Ok(())
    }
}

impl CornellNote {
    /// Create a new Cornell Note with default values
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        CornellNote {
            version: "1.0".to_string(),
            metadata: Metadata {
                id: Uuid::new_v4(),
                title,
                created: now,
                modified: now,
                subject: None,
                topic: None,
                author: None,
                tags: None,
            },
            sections: vec![],
            summary: Summary {
                content: Content::Simple(String::new()),
                created: now,
                modified: now,
            },
        }
    }

    /// Validate the entire Cornell Note document
    pub fn validate(&self) -> Result<()> {
        // Validate version format
        let version_regex = regex::Regex::new(r"^[0-9]+\.[0-9]+$").unwrap();
        if !version_regex.is_match(&self.version) {
            return Err(CornellNoteError::InvalidVersion(format!(
                "Version '{}' does not match required format (e.g., '1.0')",
                self.version
            )));
        }

        // Validate metadata
        self.metadata.validate()?;

        // Validate sections
        if self.sections.is_empty() {
            return Err(CornellNoteError::ValidationError(
                "Document must contain at least one section".to_string(),
            ));
        }

        // Track all section IDs to check for duplicates
        let mut section_ids = HashSet::new();
        let mut section_positions = HashSet::new();

        for section in &self.sections {
            // Check for duplicate section IDs
            if !section_ids.insert(section.id) {
                return Err(CornellNoteError::ValidationError(format!(
                    "Duplicate section ID: {}",
                    section.id
                )));
            }

            // Check for duplicate positions
            if !section_positions.insert(section.position) {
                return Err(CornellNoteError::ValidationError(format!(
                    "Duplicate section position: {}",
                    section.position
                )));
            }

            section.validate()?;
        }

        // Validate summary
        self.summary.validate()?;

        Ok(())
    }

    /// Add a new section to the note
    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
        self.metadata.modified = Utc::now();
    }

    /// Update the summary
    pub fn update_summary(&mut self, content: Content) {
        self.summary.content = content;
        self.summary.modified = Utc::now();
        self.metadata.modified = Utc::now();
    }
}

impl Section {
    /// Create a new empty section
    pub fn new(position: u32) -> Self {
        Section {
            id: Uuid::new_v4(),
            cues: vec![],
            notes: vec![],
            position,
        }
    }

    /// Validate a section
    pub fn validate(&self) -> Result<()> {
        // Collect all cue IDs for later validation of cue links
        let mut cue_ids = HashSet::new();
        let mut cue_positions = HashSet::new();

        for cue in &self.cues {
            if !cue_ids.insert(cue.id) {
                return Err(CornellNoteError::ValidationError(format!(
                    "Duplicate cue ID in section: {}",
                    cue.id
                )));
            }

            if !cue_positions.insert(cue.position) {
                return Err(CornellNoteError::ValidationError(format!(
                    "Duplicate cue position in section: {}",
                    cue.position
                )));
            }

            cue.validate()?;
        }

        let mut note_ids = HashSet::new();
        let mut note_positions = HashSet::new();

        for note in &self.notes {
            if !note_ids.insert(note.id) {
                return Err(CornellNoteError::ValidationError(format!(
                    "Duplicate note ID in section: {}",
                    note.id
                )));
            }

            if !note_positions.insert(note.position) {
                return Err(CornellNoteError::ValidationError(format!(
                    "Duplicate note position in section: {}",
                    note.position
                )));
            }

            note.validate()?;

            // Validate cue links reference actual cues
            if let Some(ref links) = note.cue_links {
                for link in links {
                    if !cue_ids.contains(link) {
                        return Err(CornellNoteError::ValidationError(format!(
                            "Note {} references non-existent cue: {}",
                            note.id, link
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    /// Add a cue to this section
    pub fn add_cue(&mut self, cue: Cue) {
        self.cues.push(cue);
    }

    /// Add a note to this section
    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }
}

impl Cue {
    /// Create a new cue
    pub fn new(content: Content, cue_type: CueType, position: u32) -> Self {
        let now = Utc::now();
        Cue {
            id: Uuid::new_v4(),
            content,
            cue_type,
            position,
            created: now,
            modified: now,
        }
    }

    /// Validate a cue
    pub fn validate(&self) -> Result<()> {
        if self.modified < self.created {
            return Err(CornellNoteError::ValidationError(format!(
                "Cue {} has modified date before created date",
                self.id
            )));
        }
        Ok(())
    }
}

impl Note {
    /// Create a new note
    pub fn new(content: Content, note_type: NoteType, position: u32) -> Self {
        let now = Utc::now();
        Note {
            id: Uuid::new_v4(),
            content,
            note_type,
            position,
            created: now,
            modified: now,
            cue_links: None,
        }
    }

    /// Create a new note with cue links
    pub fn new_with_links(
        content: Content,
        note_type: NoteType,
        position: u32,
        cue_links: Vec<Uuid>,
    ) -> Self {
        let now = Utc::now();
        Note {
            id: Uuid::new_v4(),
            content,
            note_type,
            position,
            created: now,
            modified: now,
            cue_links: Some(cue_links),
        }
    }

    /// Validate a note
    pub fn validate(&self) -> Result<()> {
        if self.modified < self.created {
            return Err(CornellNoteError::ValidationError(format!(
                "Note {} has modified date before created date",
                self.id
            )));
        }
        Ok(())
    }
}

impl Summary {
    /// Create a new summary
    pub fn new(content: Content) -> Self {
        let now = Utc::now();
        Summary {
            content,
            created: now,
            modified: now,
        }
    }

    /// Validate a summary
    pub fn validate(&self) -> Result<()> {
        if self.modified < self.created {
            return Err(CornellNoteError::ValidationError(
                "Summary has modified date before created date".to_string(),
            ));
        }
        Ok(())
    }
}

/// Parse a Cornell Note from JSON string
pub fn from_json(json: &str) -> Result<CornellNote> {
    let note: CornellNote = serde_json::from_str(json)?;
    note.validate()?;
    Ok(note)
}

/// Parse a Cornell Note from JSON string without validation
pub fn from_json_unchecked(json: &str) -> Result<CornellNote> {
    Ok(serde_json::from_str(json)?)
}

/// Serialize a Cornell Note to JSON string
pub fn to_json(note: &CornellNote) -> Result<String> {
    Ok(serde_json::to_string(note)?)
}

/// Serialize a Cornell Note to pretty-printed JSON string
pub fn to_json_pretty(note: &CornellNote) -> Result<String> {
    Ok(serde_json::to_string_pretty(note)?)
}

/// Read a Cornell Note from a file
pub fn read_from_file(path: &str) -> Result<CornellNote> {
    let contents = std::fs::read_to_string(path)?;
    from_json(&contents)
}

/// Write a Cornell Note to a file
pub fn write_to_file(note: &CornellNote, path: &str) -> Result<()> {
    let json = to_json_pretty(note)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Helper function to extract content as a string
fn content_to_string(content: &Content) -> String {
    match content {
        Content::Simple(s) => s.clone(),
        Content::Structured(structured) => structured.data.clone(),
    }
}

/// Export a Cornell Note to Markdown format
pub fn to_markdown(note: &CornellNote) -> String {
    let mut output = String::new();

    // Title
    output.push_str(&format!("# {}\n\n", note.metadata.title));

    // Optional metadata
    if let Some(ref subject) = note.metadata.subject {
        output.push_str(&format!("**Subject:** {}\n\n", subject));
    }
    if let Some(ref topic) = note.metadata.topic {
        output.push_str(&format!("**Topic:** {}\n\n", topic));
    }
    if let Some(ref author) = note.metadata.author {
        output.push_str(&format!("**Author:** {}\n\n", author));
    }
    if let Some(ref tags) = note.metadata.tags {
        if !tags.is_empty() {
            output.push_str(&format!("**Tags:** {}\n\n", tags.join(", ")));
        }
    }

    // Sections
    let mut sorted_sections = note.sections.clone();
    sorted_sections.sort_by_key(|s| s.position);

    for section in sorted_sections {
        output.push_str(&format!("## Section {}\n\n", section.position + 1));

        // Cues
        if !section.cues.is_empty() {
            output.push_str("### Cues\n\n");
            let mut sorted_cues = section.cues.clone();
            sorted_cues.sort_by_key(|c| c.position);

            for cue in sorted_cues {
                let content = content_to_string(&cue.content);
                output.push_str(&format!("- {}\n", content));
            }
            output.push('\n');
        }

        // Notes
        if !section.notes.is_empty() {
            output.push_str("### Notes\n\n");
            let mut sorted_notes = section.notes.clone();
            sorted_notes.sort_by_key(|n| n.position);

            for note_entry in sorted_notes {
                let content = content_to_string(&note_entry.content);
                output.push_str(&content);
                output.push_str("\n\n");
            }
        }
    }

    // Summary
    output.push_str("## Summary\n\n");
    let summary_content = content_to_string(&note.summary.content);
    output.push_str(&summary_content);
    output.push('\n');

    output
}

/// Write a Cornell Note to a Markdown file
pub fn write_to_markdown_file(note: &CornellNote, path: &str) -> Result<()> {
    let markdown = to_markdown(note);
    std::fs::write(path, markdown)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_note() {
        let note = CornellNote::new("Test Note".to_string());
        assert_eq!(note.version, "1.0");
        assert_eq!(note.metadata.title, "Test Note");
        assert!(note.sections.is_empty());
    }

    #[test]
    fn test_validation_requires_sections() {
        let note = CornellNote::new("Test".to_string());
        assert!(note.validate().is_err());
    }

    #[test]
    fn test_version_validation() {
        let mut note = CornellNote::new("Test".to_string());
        note.version = "invalid".to_string();
        let result = note.validate();
        assert!(result.is_err());
        match result {
            Err(CornellNoteError::InvalidVersion(_)) => (),
            _ => panic!("Expected InvalidVersion error"),
        }
    }

    #[test]
    fn test_empty_title_validation() {
        let mut note = CornellNote::new("".to_string());
        let mut section = Section::new(0);
        section.add_cue(Cue::new(
            Content::Simple("Test cue".to_string()),
            CueType::Text,
            0,
        ));
        note.add_section(section);
        assert!(note.validate().is_err());
    }

    #[test]
    fn test_valid_note_with_section() {
        let mut note = CornellNote::new("Valid Note".to_string());
        let mut section = Section::new(0);

        let cue = Cue::new(
            Content::Simple("What is Rust?".to_string()),
            CueType::Question,
            0,
        );
        section.add_cue(cue);

        let note_entry = Note::new(
            Content::Simple("A systems programming language".to_string()),
            NoteType::Text,
            0,
        );
        section.add_note(note_entry);

        note.add_section(section);
        assert!(note.validate().is_ok());
    }

    #[test]
    fn test_cue_links_validation() {
        let mut note = CornellNote::new("Test".to_string());
        let mut section = Section::new(0);

        let cue = Cue::new(Content::Simple("Cue".to_string()), CueType::Text, 0);
        let cue_id = cue.id;
        section.add_cue(cue);

        // Valid cue link
        let note_entry = Note::new_with_links(
            Content::Simple("Note".to_string()),
            NoteType::Text,
            0,
            vec![cue_id],
        );
        section.add_note(note_entry);

        note.add_section(section);
        assert!(note.validate().is_ok());
    }

    #[test]
    fn test_invalid_cue_links() {
        let mut note = CornellNote::new("Test".to_string());
        let mut section = Section::new(0);

        // Add note with link to non-existent cue
        let note_entry = Note::new_with_links(
            Content::Simple("Note".to_string()),
            NoteType::Text,
            0,
            vec![Uuid::new_v4()],
        );
        section.add_note(note_entry);

        note.add_section(section);
        assert!(note.validate().is_err());
    }

    #[test]
    fn test_structured_content() {
        let structured = StructuredContent {
            format: ContentFormat::Markdown,
            data: "# Header\n\nContent".to_string(),
            attachments: None,
        };

        let content = Content::Structured(structured);
        let cue = Cue::new(content, CueType::Text, 0);
        assert!(cue.validate().is_ok());
    }

    #[test]
    fn test_json_roundtrip() {
        let mut note = CornellNote::new("Roundtrip Test".to_string());
        let mut section = Section::new(0);

        section.add_cue(Cue::new(
            Content::Simple("Question?".to_string()),
            CueType::Question,
            0,
        ));
        section.add_note(Note::new(
            Content::Simple("Answer".to_string()),
            NoteType::Text,
            0,
        ));

        note.add_section(section);

        let json = to_json_pretty(&note).unwrap();
        let parsed = from_json(&json).unwrap();

        assert_eq!(note, parsed);
    }

    #[test]
    fn test_duplicate_section_ids() {
        let mut note = CornellNote::new("Test".to_string());
        let section1 = Section::new(0);
        let mut section2 = Section::new(1);
        section2.id = section1.id; // Duplicate ID

        note.add_section(section1);
        note.add_section(section2);

        assert!(note.validate().is_err());
    }

    #[test]
    fn test_duplicate_cue_positions() {
        let mut note = CornellNote::new("Test".to_string());
        let mut section = Section::new(0);

        section.add_cue(Cue::new(
            Content::Simple("Cue 1".to_string()),
            CueType::Text,
            0,
        ));
        section.add_cue(Cue::new(
            Content::Simple("Cue 2".to_string()),
            CueType::Text,
            0, // Duplicate position
        ));

        note.add_section(section);
        assert!(note.validate().is_err());
    }

    #[test]
    fn test_markdown_export() {
        let mut note = CornellNote::new("Introduction to Rust".to_string());
        note.metadata.subject = Some("Computer Science".to_string());
        note.metadata.topic = Some("Programming Languages".to_string());
        note.metadata.author = Some("Student".to_string());
        note.metadata.tags = Some(vec!["rust".to_string(), "programming".to_string()]);

        let mut section = Section::new(0);

        section.add_cue(Cue::new(
            Content::Simple("What is Rust?".to_string()),
            CueType::Question,
            0,
        ));
        section.add_cue(Cue::new(
            Content::Simple("Memory Safety".to_string()),
            CueType::Keyword,
            1,
        ));

        section.add_note(Note::new(
            Content::Simple("A systems programming language".to_string()),
            NoteType::Text,
            0,
        ));
        section.add_note(Note::new(
            Content::Structured(StructuredContent {
                format: ContentFormat::Markdown,
                data: "Rust provides memory safety without GC".to_string(),
                attachments: None,
            }),
            NoteType::Text,
            1,
        ));

        note.add_section(section);
        note.update_summary(Content::Simple(
            "Rust is a safe systems language".to_string(),
        ));

        let markdown = to_markdown(&note);

        assert!(markdown.contains("# Introduction to Rust"));
        assert!(markdown.contains("**Subject:** Computer Science"));
        assert!(markdown.contains("**Topic:** Programming Languages"));
        assert!(markdown.contains("**Author:** Student"));
        assert!(markdown.contains("**Tags:** rust, programming"));
        assert!(markdown.contains("## Section 1"));
        assert!(markdown.contains("### Cues"));
        assert!(markdown.contains("- What is Rust?"));
        assert!(markdown.contains("- Memory Safety"));
        assert!(markdown.contains("### Notes"));
        assert!(markdown.contains("A systems programming language"));
        assert!(markdown.contains("Rust provides memory safety without GC"));
        assert!(markdown.contains("## Summary"));
        assert!(markdown.contains("Rust is a safe systems language"));
    }

    #[test]
    fn test_markdown_file_write() {
        let mut note = CornellNote::new("Test Note".to_string());
        let mut section = Section::new(0);

        section.add_cue(Cue::new(
            Content::Simple("Cue".to_string()),
            CueType::Text,
            0,
        ));
        section.add_note(Note::new(
            Content::Simple("Note".to_string()),
            NoteType::Text,
            0,
        ));

        note.add_section(section);
        note.update_summary(Content::Simple("Summary".to_string()));

        // Use cross-platform temp directory
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_note.md");
        write_to_markdown_file(&note, temp_file.to_str().unwrap()).unwrap();

        let contents = std::fs::read_to_string(&temp_file).unwrap();
        assert!(contents.contains("# Test Note"));
        assert!(contents.contains("## Section 1"));
        assert!(contents.contains("### Cues"));
        assert!(contents.contains("- Cue"));
        assert!(contents.contains("### Notes"));
        assert!(contents.contains("Note"));
        assert!(contents.contains("## Summary"));
        assert!(contents.contains("Summary"));

        std::fs::remove_file(temp_file).ok();
    }
}
