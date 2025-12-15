use clap::Parser as _;
use heck::{ToKebabCase as _, ToLowerCamelCase as _, ToPascalCase as _, ToSnakeCase as _};
use regex::bytes::{Captures, Regex};
use std::io::{self, Read as _, Write as _};

#[expect(clippy::doc_markdown)]
#[derive(Clone, clap::ValueEnum)]
enum Case {
    /// snake_case
    Snake,
    /// kebab-case
    Kebab,
    /// camelCase
    Camel,
    /// PascalCase
    Pascal,
    /// sPoNgEbOb
    Spongebob,
}

#[derive(clap::Parser)]
struct Args {
    case: Case,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let regex = Regex::new(r"\b_*[a-zA-Z0-9]+(?:[-]?[a-zA-Z0-9_]+)*\b")?;

    let case = |s: &str| match args.case {
        Case::Snake => s.to_snake_case(),
        Case::Kebab => s.to_kebab_case(),
        Case::Camel => s.to_lower_camel_case(),
        Case::Pascal => s.to_pascal_case(),
        Case::Spongebob => spongebob(s),
    };

    let mut input = Vec::new();

    io::stdin().read_to_end(&mut input)?;

    let output = regex.replace_all(&input, |captures: &Captures| {
        case(str::from_utf8(&captures[0]).unwrap())
    });

    let _ = io::stdout().write_all(&output);

    Ok(())
}

fn spongebob(text: &str) -> String {
    let mut string = String::with_capacity(text.len());
    let mut upper = false;
    for char in text.chars() {
        if !char.is_alphabetic() {
            string.push(char);
            continue;
        }
        if upper {
            string.extend(char.to_uppercase());
        } else {
            string.extend(char.to_lowercase());
        }
        upper = fastrand::bool();
    }
    string
}
