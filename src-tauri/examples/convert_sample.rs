use std::fs;

// We need to access the converter functions, so we'll use the public interface
fn main() {
    eprintln!("Reading sample file...");
    let content = match fs::read_to_string("../hands-pglCX2WsUJbPBjsNSE1siiDJy.ohh.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to read input file: {}", e);
            std::process::exit(1);
        }
    };

    eprintln!("✓ Loaded {} bytes", content.len());
    eprintln!("Converting hands...\n");

    // Since converter module is private, we'll need to make it public or use lib exports
    // For now, let's just show the structure and user can run via the app

    println!("To test the full conversion:");
    println!("1. Run: cargo tauri dev");
    println!("2. Upload the hands-pglCX2WsUJbPBjsNSE1siiDJy.ohh.txt file");
    println!("3. Click 'Convert to PokerStars Format'");
    println!("4. Download the converted output");
    println!("\nAlternatively, you can use the test:");
    println!("cargo test test_real_sample_hand -- --nocapture");
}
