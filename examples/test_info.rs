use tjpgdec_rs::{JpegDecoder, MemoryPool, RECOMMENDED_POOL_SIZE, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let jpeg_path = if args.len() > 1 { &args[1] } else { "test_images/test1.jpg" };
    
    let jpeg_data = std::fs::read(jpeg_path).unwrap_or_else(|e| {
        eprintln!("Error reading '{}': {}", jpeg_path, e);
        std::process::exit(1);
    });
    
    let mut pool_buffer = vec![0u8; RECOMMENDED_POOL_SIZE];
    let mut pool = MemoryPool::new(&mut pool_buffer);
    let mut decoder = JpegDecoder::new();
    decoder.prepare(&jpeg_data, &mut pool)?;
    
    println!("Width: {}", decoder.width());
    println!("Height: {}", decoder.height());
    println!("Components: {}", decoder.components());
    
    Ok(())
}
