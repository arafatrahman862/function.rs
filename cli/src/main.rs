use std::env;

const HELP: &str = r#"

"#;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut args = env::args();
    let cmd = args.nth(1).unwrap_or("-h".into());

    match &cmd[..] {
        "new" => match args.next() {
            Some(name) => {
                
                // println!("{:?}", name);
            }
            None => {}
        },
        "init" => {}
        "--help" | "-h" | "" => println!("{HELP}"),
        "--version" | "-v" => println!("{NAME} v{VERSION}"),
        cmd => println!("No such command: `{cmd}`"),
    }
}
