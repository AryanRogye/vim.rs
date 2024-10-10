mod buffer;
mod terminal;
mod errors;
mod command;
mod list;

use std::{fs, env, process::exit, path::Path};
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
    let mut contents : String = "".to_string();
    match args.len() {
        1 => {
            println!("1 arg");
            // For now
            exit(1);
        }
        2 => {
            println!("2 args");
            let file = &args[1];
            if file.len() == 1 {exit(1)};

            // Right now it has to just be a filepath so has to end with a .(something)
            // The specified path will always be (currentdir)/(filename)
            // or (currentdir)/../../../
            if !Path::new(file).exists() {
                fs::File::create_new(file)?;
            }
            contents = fs::read_to_string(file).expect("Could Not Read File");
            println!("{contents}");
        }
        _ => {
            println!("{} args", args.len());
        }
    }
    let terminal : Terminal = Terminal;
    let buf : Buffer = Buffer::new();
    buf.configure_lines(contents).expect("Count Not Parse File");
    if let Err(e) = terminal.setup(buf) {
        println!("Error: {}", e);
    }
    Ok(())
}
