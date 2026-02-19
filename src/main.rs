//! This is the main driver code for the starter.
//! Run with `cargo run` or `<project_name>` to see the auto-generated help text.

use clap::Parser as _;

pub(crate) mod api;
mod commands;

use commands::*;

type Result<T> = color_eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
#[clap(arg_required_else_help = true)]
enum Commands {
    /// Basic command that does things and stuff
    Basic,
    /// Example command, useful to copy and scaffold new commands
    Example(example::Arguments),
}

fn main() -> crate::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    if let Some(cmds) = &cli.command {
        match cmds {
            Commands::Basic => basic_command(),
            Commands::Example(args) => example::run(args),
        }?;
    };

    Ok(())
}

fn basic_command() -> crate::Result<()> {
    println!("Running the basic command from the top level");

    api::ExampleApi::foo()?;

    Ok(())
}
