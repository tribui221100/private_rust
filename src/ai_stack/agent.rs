// ============================================================================
// MODULE: agent.rs
// PURPOSE: Acts as the orchestrator that manages system rules and context.
// ============================================================================

// STEP 1: Import internal modules and file-system tools
// - Import `std::fs` to read flat files from disk.
// - Import `crate::ai_client` to delegate the networking operations.

// STEP 2: Define the asynchronous Agent runner signature
// - Define `pub async fn run_agent(user_question: &str) -> Result<String, Box<dyn std::error::Error>>`
// - It must be `async` because it will await the internal `ai_client` call.

    // STEP 3: Ingest the System Rules configuration file
    // - Read the contents of "agent_rules.md" located at the project root into a String using `fs::read_to_string`.
    // - Chain `.map_err()` to catch file IO errors (e.g., file not found) and provide 
    //   a descriptive error message. Propagate up using the `?` operator.

    // STEP 4: Context Integration (Prompt Engineering)
    // - Since this is a raw API communication layer, combine the System Rules and 
    //   the User's Question into a single coherent prompt string using `format!`.
    // - Explicitly separate sections so the AI differentiates between its rules and the query.

    // STEP 5: Delegate execution to the Client
    // - Pass the consolidated prompt into `ai_client::send_to_gemini(&full_prompt)`.
    // - Append `.await` to yield execution control until the response arrives.
    // - Leave the expression open (no trailing semicolon) to implicitly return the final `Result` directly to `main.rs`.

    use std::fs;
    use crate::ai_stack::ai_cli;
    use crate::ai_stack::code_creator;
    use std::io::{self, Write};
    use colored::*;
    
    pub async fn run_agent() -> Result<String, Box<dyn std::error::Error>> {
        // Read rules
        let rules = include_str!("../../agent_rules.md");
    
        loop {
            print!("You: ");
            io::stdout().flush()?;
    
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input)?;
            let user_input = user_input.trim();
    
            if user_input.eq_ignore_ascii_case("exit") {
                println!("Goodbye!");
                break Ok(Default::default());
            }
    
            if user_input.is_empty() {
                continue;
            }
    
            // Context Integration
            let full_prompt = format!(
                "--- SYSTEM RULES (ALWAYS OBEY) ---\n\
                {}\n\
                ----------------------------------\n\
                User question: {}", 
                rules, user_input
            );
    
            // Delegate execution to the Client
            match ai_cli::send_to_gemini(&full_prompt).await {
                Ok(reply) => {
                    match code_creator::code_generate(user_input, &reply) {
                        Ok(filename) => {
                            println!("\n{} File automatically created: {}", "⚙️ [Embedded System]:".green().bold(), filename.yellow());
                        },
                        Err(e) => {
                            eprintln!("\n⚠️ [Embedded Warning]: Cannot create file due to  {:?}", e);
                        }
                    };
                    use syntect::easy::HighlightLines;
                    use syntect::parsing::SyntaxSet;
                    use syntect::highlighting::{ThemeSet, Style};
                    use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
    
                    // Load default syntax definitions and themes
                    let ps = SyntaxSet::load_defaults_newlines();
                    let ts = ThemeSet::load_defaults();
                    
                    // Pick a dark-mode friendly theme: "base16-ocean.dark" or "Solarized (dark)"
                    let theme = &ts.themes["base16-ocean.dark"];
                    
                    // Track state for the code blocks
                    let mut in_code_block = false;
                    let mut highligher: Option<HighlightLines> = None;
    
                    print!("\n{} ", "Agent:".red().bold());
    
                    // Use LinesWithEndings to keep syntect's parser structurally happy
                    for line in LinesWithEndings::from(&reply) {
                        let trimmed = line.trim();
    
                        // Check for the entry or exit of a code block fence
                        if trimmed.starts_with("```") {
                            if !in_code_block {
                                in_code_block = true;
                                // Extract language flag (e.g., "rust" from "```rust")
                                let lang = trimmed.strip_prefix("
                                    ```").unwrap_or("txt").trim();
                                let syntax = ps.find_syntax_by_token(lang)
                                    .unwrap_or_else(|| ps.find_syntax_by_token("txt").unwrap());
                                
                                highligher = Some(HighlightLines::new(syntax, theme));
                                println!(); // Line break before code starts
                            } else {
                                in_code_block = false;
                                highligher = None;
                                println!(); // Line break when code ends
                            }
                            continue;
                        }
    
                        if in_code_block {
                            if let Some(ref mut hl) = highligher {
                                // Syntax highlight the code line
                                let regions = hl.highlight_line(line, &ps).unwrap();
                                // Convert colors to TrueColor ANSI escape blocks
                                let escaped = as_24_bit_terminal_escaped(&regions[..], false);
                                print!("{}", escaped);
                            } else {
                                print!("{}", line);
                            }
                        } else {
                            // Regular Markdown formatting outside code blocks
                            if trimmed.starts_with("### ") || trimmed.starts_with("## ") {
                                print!("{}", line.cyan().bold());
                            } else {
                                print!("{}", line);
                            }
                        }
                    }
                    // Reset terminal color just in case
                    print!("\x1b[0m");
                    println!("{}", "\n----------------------------------------".bright_black());
                }
                Err(e) => {
                    println!("\n❌ Error: {}\n", e);
                }
            }
        }
    }