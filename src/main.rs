mod modules;
mod ai_cli;
mod agent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- INTIALIZE AI AGENT (RULE-BASED) ---");

    let question = "Please briefly explain the Ownership mechanism in Rust in exactly two sentences.";
    println!("User asks: {}\n", question);

    // match agent::run_agent(question).await {
    //     Ok(answer) => {
    //         println!("Agent phản hồi:");
    //         println!("{}", answer);
    //     },
    //     Err(e) => eprintln!("Errors Occur: {}", e),
    // }

    Ok(())
}

