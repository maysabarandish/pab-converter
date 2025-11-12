#!/bin/bash

echo "Creating conversion test..."

# Create a test that reads and converts the full file
cat > ../test_full_conversion.rs << 'TESTCODE'
#[cfg(test)]
mod full_conversion_test {
    use std::fs;
    
    #[test]
    #[ignore] // Ignore by default, run with: cargo test --ignored
    fn convert_full_sample_file() {
        // Read the full sample file
        let content = fs::read_to_string("../hands-pglCX2WsUJbPBjsNSE1siiDJy.ohh.txt")
            .expect("Failed to read sample file");
        
        println!("Loaded {} bytes from sample file", content.len());
        
        // Convert using the converter module
        let result = pab_converter_lib::converter::convert_ohh_file(&content);
        
        assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
        
        let output = result.unwrap();
        let hand_count = output.matches("PokerStars Hand #").count();
        
        println!("✓ Successfully converted {} hands", hand_count);
        println!("✓ Output size: {} bytes", output.len());
        
        // Write to file
        fs::write("../converted_full_sample.txt", &output)
            .expect("Failed to write output file");
        
        println!("✓ Wrote converted hands to ../converted_full_sample.txt");
        
        // Show first 50 lines as preview
        let preview: Vec<&str> = output.lines().take(50).collect();
        println!("\n=== PREVIEW (first 50 lines) ===");
        for line in preview {
            println!("{}", line);
        }
        println!("=== END PREVIEW ===\n");
    }
}
TESTCODE

echo "Test file created. Since converter module is private, we need to make it public."
echo "Let's add it to the lib.rs exports..."

