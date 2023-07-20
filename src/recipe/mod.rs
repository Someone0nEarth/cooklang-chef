use std::io::Read;

use anyhow::{anyhow, Context as _, Result};
use camino::Utf8PathBuf;
use clap::{Args, Subcommand};
use cooklang_fs::{resolve_recipe, FsIndex};

use crate::{util::Input, Context};

mod ast;
mod check;
mod read;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct RecipeArgs {
    #[command(subcommand)]
    command: Option<RecipeCommand>,

    #[command(flatten)]
    read_args: read::ReadArgs,
}

#[derive(Debug, Subcommand)]
enum RecipeCommand {
    /// Reads a recipe file
    #[command(alias = "r")]
    Read(read::ReadArgs),
    /// Checks a recipe file for errors or warnings
    #[command(alias = "c")]
    Check(check::CheckArgs),
    /// Get the recipe abstract syntax tree
    Ast(ast::AstArgs),
}

pub fn run(ctx: &Context, args: RecipeArgs) -> Result<()> {
    let command = args.command.unwrap_or(RecipeCommand::Read(args.read_args));

    match command {
        RecipeCommand::Read(args) => read::run(ctx, args),
        RecipeCommand::Check(args) => check::run(ctx, args),
        RecipeCommand::Ast(args) => ast::run(ctx, args),
    }
}

#[derive(Debug, Args)]
struct RecipeInputArgs {
    /// Input recipe, none for stdin
    ///
    /// This can be a full path, a partial path, or just the name.
    #[arg(value_hint = clap::ValueHint::FilePath)]
    recipe: Option<Utf8PathBuf>,

    /// Give or override a name for the recipe
    ///
    /// If not given will be obtained from input path.
    #[arg(short, long, required_unless_present = "recipe")]
    name: Option<String>,
}

impl RecipeInputArgs {
    pub fn read(&self, index: &FsIndex) -> Result<Input> {
        let input = if let Some(query) = &self.recipe {
            // RecipeInputArgs::recipe is a pathbuf even if inmediatly converted
            // to a string to enforce validation.
            let entry = resolve_recipe(query.as_str(), index, None)?;

            Input::File {
                content: entry.read()?,
                override_name: self.name.clone(),
            }
        } else {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("Failed to read stdin")?;
            Input::Stdin {
                text: buf,
                name: self.name.clone().ok_or(anyhow!("No name for recipe"))?,
            }
        };
        Ok(input)
    }
}
