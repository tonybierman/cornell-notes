use cornell_notes::{
    Content, ContentFormat, CornellNote, Cue, CueType, Note, NoteType, Section, StructuredContent,
};

fn main() -> cornell_notes::Result<()> {
    // Create a new Cornell Note
    let mut note = CornellNote::new("Introduction to Rust Programming".to_string());

    // Add metadata
    note.metadata.subject = Some("Computer Science".to_string());
    note.metadata.topic = Some("Rust Programming Language".to_string());
    note.metadata.author = Some("Tony Bierman".to_string());
    note.metadata.tags = Some(vec![
        "programming".to_string(),
        "rust".to_string(),
        "systems".to_string(),
    ]);

    // Create first section with basic cues and notes
    let mut section1 = Section::new(0);

    // Add a question cue
    let cue1 = Cue::new(
        Content::Simple("What is Rust?".to_string()),
        CueType::Question,
        0,
    );
    let cue1_id = cue1.id;
    section1.add_cue(cue1);

    // Add a note linked to the cue
    let note1 = Note::new_with_links(
        Content::Simple(
            "Rust is a systems programming language focused on safety, speed, and concurrency."
                .to_string(),
        ),
        NoteType::Text,
        0,
        vec![cue1_id],
    );
    section1.add_note(note1);

    // Add a keyword cue
    let cue2 = Cue::new(
        Content::Simple("Memory Safety".to_string()),
        CueType::Keyword,
        1,
    );
    let cue2_id = cue2.id;
    section1.add_cue(cue2);

    // Add a note with markdown content
    let note2 = Note::new_with_links(
        Content::Structured(StructuredContent {
            format: ContentFormat::Markdown,
            data: "Rust achieves memory safety through:\n- **Ownership**: Each value has a single owner\n- **Borrowing**: References with compile-time checks\n- **Lifetimes**: Ensures references are valid".to_string(),
            attachments: None,
        }),
        NoteType::List,
        1,
        vec![cue2_id],
    );
    section1.add_note(note2);

    note.add_section(section1);

    // Create second section with code example
    let mut section2 = Section::new(1);

    let cue3 = Cue::new(
        Content::Simple("Basic Syntax Example".to_string()),
        CueType::Text,
        0,
    );
    let cue3_id = cue3.id;
    section2.add_cue(cue3);

    let code_example = r#"fn main() {
    let message = "Hello, Rust!";
    println!("{}", message);
}
"#;

    let note3 = Note::new_with_links(
        Content::Structured(StructuredContent {
            format: ContentFormat::Plain,
            data: code_example.to_string(),
            attachments: None,
        }),
        NoteType::Code,
        0,
        vec![cue3_id],
    );
    section2.add_note(note3);

    note.add_section(section2);

    // Add summary
    note.update_summary(Content::Simple(
        "Rust is a modern systems programming language that provides memory safety without \
         garbage collection through its ownership system. It's ideal for performance-critical \
         applications where safety and concurrency are important."
            .to_string(),
    ));

    // Validate the note
    println!("Validating Cornell Note...");
    note.validate()?;
    println!("Validation successful!");

    // Convert to JSON
    let json = cornell_notes::to_json_pretty(&note)?;
    println!("\nGenerated JSON:\n{}", json);

    // Save to file
    let output_file = "example_note.json";
    cornell_notes::write_to_file(&note, output_file)?;
    println!("\nNote saved to: {}", output_file);

    // Export to Markdown
    let markdown_file = "example_note.md";
    cornell_notes::write_to_markdown_file(&note, markdown_file)?;
    println!("Markdown exported to: {}", markdown_file);

    // Show markdown preview
    let markdown = cornell_notes::to_markdown(&note);
    println!("\nMarkdown preview:\n{}", "=".repeat(60));
    println!("{}", markdown);
    println!("{}", "=".repeat(60));

    // Read back from file
    println!("\nReading note from JSON file...");
    let loaded_note = cornell_notes::read_from_file(output_file)?;
    println!("Successfully loaded note: {}", loaded_note.metadata.title);
    println!("Version: {}", loaded_note.version);
    println!("Sections: {}", loaded_note.sections.len());
    println!(
        "Total cues: {}",
        loaded_note
            .sections
            .iter()
            .map(|s| s.cues.len())
            .sum::<usize>()
    );
    println!(
        "Total notes: {}",
        loaded_note
            .sections
            .iter()
            .map(|s| s.notes.len())
            .sum::<usize>()
    );

    Ok(())
}
