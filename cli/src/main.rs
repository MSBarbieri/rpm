mod cli;

use manager::App;
use clap::Shell;
use rpm_tui::{panic_hook,start_tui};
use anyhow::{Result,anyhow};
use std:: {
    panic::{self}
};
use cli::create_cli;

#[tokio::main]
async fn main() -> Result<()> {
    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));
    println!("Hello, world!");

    let mut clap_app = create_cli()?;
    
    let matches = clap_app.clone().get_matches();

    // Shell completions don't need any spotify work
    if let Some(s) = matches.value_of("completions") {
        let shell = match s {
            "fish" => Shell::Fish,
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "power-shell" => Shell::PowerShell,
            "elvish" => Shell::Elvish,
            _ => return Err(anyhow!("no completions avaible for '{}'", s)),
        };
        clap_app.gen_completions_to("rpm", shell, &mut std::io::stdout());
        
    }

    match matches.subcommand_name().unwrap_or_default() {
        "new" => {}
        _ => {
            let app = &mut (App::new(String::default()));
            start_tui(app).await?;
        }
    };

    Ok(())
}
