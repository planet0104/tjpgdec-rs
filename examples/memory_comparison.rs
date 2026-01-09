//! C版本 vs Rust版本 内存使用对比
//! 
//! 运行: cargo run --example memory_comparison

use std::mem::size_of;
use tjpgdec_rs::{JpegDecoder, MemoryPool, calculate_pool_size};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        C版本 vs Rust版本 内存使用对比                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    println!("\n=== C版本 (JD_FASTDECODE=2) ===");
    println!("┌─────────────────────────────────┬────────────────┐");
    println!("│ 分配位置                        │ 大小 (bytes)   │");
    println!("├─────────────────────────────────┼────────────────┤");
    println!("│ JDEC结构体 (栈)                 │ ~120           │");
    println!("│ inbuf (池内)                    │ 512            │");
    println!("│ huffbits[4] (池内)              │ 64             │");
    println!("│ huffcode[4] (池内,动态)         │ ~1000          │");
    println!("│ huffdata[4] (池内,动态)         │ ~500           │");
    println!("│ hufflut_ac[2] (池内)            │ 4096           │");
    println!("│ hufflut_dc[2] (池内)            │ 2048           │");
    println!("│ qttbl[4] (池内)                 │ 1024           │");
    println!("│ workbuf (池内)                  │ ~320           │");
    println!("│ mcubuf (池内)                   │ ~768           │");
    println!("├─────────────────────────────────┼────────────────┤");
    println!("│ 池总计                          │ ~9644          │");
    println!("│ 栈总计                          │ ~120           │");
    println!("│ 总计                            │ ~9764          │");
    println!("└─────────────────────────────────┴────────────────┘");

    println!("\n=== Rust版本 ===");
    let decoder_size = size_of::<JpegDecoder>();
    
    println!("┌─────────────────────────────────┬────────────────┐");
    println!("│ 分配位置                        │ 大小 (bytes)   │");
    println!("├─────────────────────────────────┼────────────────┤");
    println!("│ JpegDecoder结构体 (栈)          │ {:>14} │", decoder_size);
    println!("│ 内部数据 (池内分配)             │ 动态           │");
    println!("│ mcu_buffer (外部)               │ 用户提供       │");
    println!("│ work_buffer (外部)              │ 用户提供       │");
    println!("└─────────────────────────────────┴────────────────┘");

    println!("\n=== 内存池使用 ===");
    let pool_capacity = calculate_pool_size(0, 0, cfg!(feature = "fast-decode-2"));
    
    println!("推荐池大小: {} bytes", pool_capacity);
    println!("解码器栈使用: {} bytes", decoder_size);
    println!("MemoryPool大小: {} bytes", size_of::<MemoryPool>());

    println!("\n=== 对比总结 ===");
    println!("┌────────────────────┬──────────┬──────────┬──────────┐");
    println!("│ 版本               │ 栈使用   │ 堆/池使用│ 总计     │");
    println!("├────────────────────┼──────────┼──────────┼──────────┤");
    println!("│ C (FASTDECODE=2)   │ ~120     │ ~9644    │ ~9764    │");
    println!("│ Rust (池)          │ {:>8} │ {:>8} │ {:>8} │", decoder_size, pool_capacity, decoder_size + pool_capacity);
    println!("└────────────────────┴──────────┴──────────┴──────────┘");

    println!("\n=== 关键特性 ===");
    println!("1. Rust版本直接使用输入切片，无需固定inbuf");
    println!("2. 使用MemoryPool进行内存分配，与C版本一致");
    println!("3. fast-decode feature控制优化级别");
    println!("4. 结构体栈使用: {} bytes", decoder_size);
}
