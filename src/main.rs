mod buffer;
mod terminal;
mod errors;
mod command;

use std::env;
use buffer::buffer::Buffer;
use terminal::Terminal;


// #[tokio::main]
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match errors::check_args(args.clone()) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return Ok(())
        }
    }
    let buf : Buffer = Buffer::new();
    match args.len() {
        1 => {
            println!("1 arg");
        }
        2 => {
            println!("2 args");
            let arg = &args[1];
            println!("{}", arg);
        }
        _ => {
            println!("{} args", args.len());
        }
    }
    let terminal : Terminal = Terminal;
    if let Err(e) = terminal.setup(buf) {
        println!("Error: {}", e);
    }
    Ok(())
}
