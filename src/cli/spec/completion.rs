use clap::{Arg, Command};

pub(super) fn command() -> Command {
    Command::new("completion")
        .visible_alias("completions")
        .about("Generate shell completion scripts")
        .arg(
            Arg::new("shell")
                .value_name("SHELL")
                .required(true)
                .value_parser(super::super::completion::SUPPORTED_SHELLS)
                .help(crate::cli::localize("Shell to generate completions for")),
        )
}
