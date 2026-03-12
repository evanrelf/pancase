use clap::Parser as _;
use heck::{ToKebabCase as _, ToLowerCamelCase as _, ToPascalCase as _, ToSnakeCase as _};
use regex::bytes::{Captures, Regex};
use std::io::{self, BufRead as _, Write as _};

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
    sigpipe::reset();

    let args = Args::parse();

    let regex = Regex::new(r"\b_*[a-zA-Z0-9]+(?:[-]?[a-zA-Z0-9_]+)*\b")?;

    fastrand::seed(0);

    let case = match args.case {
        Case::Snake => |s: &str| s.to_snake_case(),
        Case::Kebab => |s: &str| s.to_kebab_case(),
        Case::Camel => |s: &str| s.to_lower_camel_case(),
        Case::Pascal => |s: &str| s.to_pascal_case(),
        Case::Spongebob => |s: &str| spongebob(s),
    };

    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut input = Vec::new();

    loop {
        input.clear();

        if stdin.read_until(b'\n', &mut input)? == 0 {
            break;
        }

        let output = regex.replace_all(&input, |captures: &Captures| {
            case(str::from_utf8(&captures[0]).unwrap())
        });

        stdout.write_all(&output)?;
        stdout.flush()?;
    }

    Ok(())
}

fn spongebob(text: &str) -> String {
    let mut string = String::with_capacity(text.len());
    for char in text.chars() {
        if !char.is_alphabetic() {
            string.push(char);
            continue;
        }
        if fastrand::bool() {
            string.extend(char.to_uppercase());
        } else {
            string.extend(char.to_lowercase());
        }
    }
    string
}
