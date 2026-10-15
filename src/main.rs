use std::env;

fn main() {
    println!("🚀 Rust Mining Initialized via API Automation!");
    
    let args: Vec<String> = env::args().collect();
    let mut start_offset: u64 = 0;
    let mut steps: u64 = 1000;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--start" && i + 1 < args.len() {
            start_offset = args[i + 1].parse().unwrap_or(0);
        } else if args[i] == "--steps" && i + 1 < args.len() {
            steps = args[i + 1].parse().unwrap_or(1000);
        }
        i += 2;
    }

    println!("🎯 Start Offset: {}, Steps: {}", start_offset, steps);
}
