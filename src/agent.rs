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
use crate::ai_cli;

pub async fn run_agent(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Read rules
    // let rules = fs::read_to_string(agent_rules.md)
    //                     .map_err(|_| "Không thể đọc file agent_rules.md!")?;

    // Context Integration
    let full_prompt = format!("User question {}",prompt);

    // Delegate execution to the Client
    let ai_resp = ai_cli::send_to_gemini(&full_prompt).await?;
    Ok(ai_resp)
}