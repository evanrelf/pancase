use clap::Parser as _;
use heck::{ToKebabCase as _, ToLowerCamelCase as _, ToPascalCase as _, ToSnakeCase as _};
use std::io;

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

fn main() {
    let args = Args::parse();
    let case = match args.case {
        Case::Snake => |s: &str| s.to_snake_case(),
        Case::Kebab => |s: &str| s.to_kebab_case(),
        Case::Camel => |s: &str| s.to_lower_camel_case(),
        Case::Pascal => |s: &str| s.to_pascal_case(),
    };
    for line in io::stdin().lines() {
        println!("{}", case(&line.unwrap()));
    }
}
