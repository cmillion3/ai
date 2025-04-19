use clap::Parser;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use std::io::{self, Write};
use std::process::Command;

/// CLI tool to turn text into terminal commands using Ollama
#[derive(Parser, Debug)]
#[command(name = "shell-gen")]
#[command(about = "Translate natural language into shell commands")]
struct Args {
    /// Natural language input (e.g. "change directory")
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Ollama::default();
	let formatted_prompt = format!("Give me only the command to {}, do not add explainations or markdowns", args.prompt);
    let model = "llama3.2";
    let request = GenerationRequest::new(model.to_string(), formatted_prompt);
    let response = client.generate(request).await?;

    let command = response.response.trim();
    println!("Generated command: {}\n", command);

    // Prompt for confirmation
    print!("Do you want to execute this command? [y/N]: ");
    io::stdout().flush()?; // make sure prompt is shown before input

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
		if command.starts_with("cd ") {
		    println!(
		        "⚠️ Note: 'cd' changes the directory only within a temporary shell and won't affect your terminal session.\n"
		    );
		}
        println!("\n⚠️ Executing generated command...\n");

        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

		let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
		let stderr  =  String::from_utf8_lossy(&output.stderr).trim().to_string();
		
		
        println!("Output:\n{}", if stdout.is_empty() { "None" } else { &stdout });
        eprintln!("Errors:\n{}", if stdout.is_empty() { "None" } else { &stderr });
    } else {
        println!("Command execution skipped.");
    }

    println!("You are running on {}", std::env::consts::OS);
    Ok(())
}
