//! Example usage of tjpgd decoder (Memory-efficient version)

use tjpgdec_rs::{JpegDecoder, MemoryPool, RECOMMENDED_POOL_SIZE, Result};
use std::env;

fn main() -> Result<()> {
    // Get JPEG file from command line argument or use default
    let args: Vec<String> = env::args().collect();
    let jpeg_path = if args.len() > 1 {
        &args[1]
    } else {
        "test_images/test1.jpg"
    };

    // Read JPEG data from file
    let jpeg_data = match std::fs::read(jpeg_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", jpeg_path, e);
            eprintln!("Usage: cargo run --example basic [jpeg_file]");
            std::process::exit(1);
        }
    };

    // Create memory pool
    let mut pool_buffer = vec![0u8; RECOMMENDED_POOL_SIZE];
    let mut pool = MemoryPool::new(&mut pool_buffer);

    // Create decoder
    let mut decoder = JpegDecoder::new();

    // Prepare (parse headers)
    decoder.prepare(&jpeg_data, &mut pool)?;

    println!("Image size: {}x{}", decoder.width(), decoder.height());
    println!("Components: {}", decoder.components());

    // Get required buffer sizes
    let mcu_size = decoder.mcu_buffer_size();
    let work_size = decoder.work_buffer_size();
    
    println!("MCU buffer size: {} (i16 elements)", mcu_size);
    println!("Work buffer size: {} bytes", work_size);

    // Allocate external buffers (memory-efficient approach)
    let mut mcu_buffer = vec![0i16; mcu_size];
    let mut work_buffer = vec![0u8; work_size];
    let mut output_buffer = Vec::new();

    // Decompress with external buffers
    decoder.decompress(
        &jpeg_data, 
        0,  // scale = 0 (no scaling)
        &mut mcu_buffer,
        &mut work_buffer,
        &mut |_decoder, bitmap, rect| {
            println!(
                "Received block: ({}, {}) to ({}, {})",
                rect.left, rect.top, rect.right, rect.bottom
            );

            // In a real application, you would write bitmap data to display or file
            output_buffer.extend_from_slice(bitmap);

            Ok(true) // Continue processing
        }
    )?;

    println!("Decompression complete!");
    println!("Total output size: {} bytes", output_buffer.len());
    Ok(())
}
