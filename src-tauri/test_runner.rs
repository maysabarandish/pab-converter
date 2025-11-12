use std::fs;

fn main() {
    // Read the sample file
    let content = match fs::read_to_string("hands-pglCX2WsUJbPBjsNSE1siiDJy.ohh.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("Loaded {} bytes from sample file", content.len());
    println!("Converting hands...\n");
    
    // Note: We can't directly call the converter since it's in a different crate
    // This would need to be integrated properly
    println!("To test properly, we need to build and run the Tauri app or create a proper test");
}
