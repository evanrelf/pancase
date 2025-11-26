use clap::Parser as _;
use heck::{ToKebabCase as _, ToLowerCamelCase as _, ToPascalCase as _, ToSnakeCase as _};
use regex::{Captures, Regex};
use std::io::{self, Write as _};

#[derive(Clone, clap::ValueEnum)]
enum Case {
    Snake,  // snake_case
    Kebab,  // kebab-case
    Camel,  // camelCase
    Pascal, // PascalCase
}

#[derive(clap::Parser)]
struct Args {
    case: Case,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let regex = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_-]*")?;

    let case = match args.case {
        Case::Snake => |s: &str| s.to_snake_case(),
        Case::Kebab => |s: &str| s.to_kebab_case(),
        Case::Camel => |s: &str| s.to_lower_camel_case(),
        Case::Pascal => |s: &str| s.to_pascal_case(),
    };

    let input = io::read_to_string(io::stdin())?;

    let output = regex.replace_all(&input, |captures: &Captures| case(&captures[0]));

    _ = write!(io::stdout(), "{output}");

    Ok(())
}
