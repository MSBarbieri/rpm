mod sub_commands;
use anyhow::{Result};
pub use self::sub_commands::{list_subcommand, play_subcommand, playback_subcommand, search_subcommand};
use clap::{App, Arg};
use manager::BANNER;

pub fn create_cli<'a,'b>() -> Result<App<'a, 'b>> {
    let title = String::from(r#"rpm - repository package manager"#);
    let clap_app = App::new(title)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .usage("Press `?` while running the app to see keybindings")
        .before_help(BANNER)
        .after_help("All configs are stored in /etc/rpm")
        .arg(
            Arg::with_name("completions")
                .long("completions")
                .help("Generates completions for your preferred shell")
                .takes_value(true)
                .possible_values(&["bash", "zsh", "fish", "power-shell", "elvish"])
                .value_name("SHELL"),
        )
        // Control spotify from the command line
        .subcommand(playback_subcommand())
        .subcommand(play_subcommand())
        .subcommand(list_subcommand())
        .subcommand(search_subcommand());

    Ok(clap_app)
}