use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: <program> <output_format>");
        std::process::exit(1);
    }

    let input_file: &String = &args[1];
    let output_format: &String = &args[2];

    match convert_file(input_file, output_format) {
        Ok(()) => println!("Conversion successful!"),
        Err(e) => eprintln!("Error during conversion: {}", e),
    }
}

fn convert_file(input_file: &str, output_format: &str) -> Result<(), String> {
    match output_format {
        ".txt" => {
            println!("txt {}", input_file);
            return Ok(());
        }
        ".csv" => {
            println!("csv {}", input_file);
            return Ok(());
        }
        ".xml" => {
            println!("xml {}", input_file);
            return Ok(());
        }
        ".yaml" => {
            println!("yaml {}", input_file);
            return Ok(());
        }
        _ => return Err(format!("No such files allowed: {}", output_format)),
    }
}
