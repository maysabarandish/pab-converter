use std::fs;

fn main() {
    let content = fs::read_to_string("test_sample.txt").unwrap();

    // Include the converter module
    mod converter;
    use converter::convert_ohh_file;

    match convert_ohh_file(&content) {
        Ok(output) => {
            println!("{}", output);
            fs::write("test_output.txt", output).unwrap();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
