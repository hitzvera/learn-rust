mod level1;
mod level2;
mod level3;
mod level4;

fn main() {
    println!("🦀 Rust Modules & Packages Challenges\n");
    println!("=====================================\n");
    
    println!("Select a level to run:");
    println!("  1. Level 1: Packages & Crates Basics");
    println!("  2. Level 2: Modules & Privacy");
    println!("  3. Level 3: Paths & Use Keyword");
    println!("  4. Level 4: File Separation & Re-exporting");
    println!("  5. Run all levels\n");
    
    // For now, run all levels
    // You can comment out levels you're not working on
    
    level1::run_challenges();
    println!("-------------------------------------\n");
    
    level2::run_challenges();
    println!("-------------------------------------\n");
    
    level3::run_challenges();
    println!("-------------------------------------\n");
    
    level4::run_challenges();
    println!("-------------------------------------\n");
    
    println!("🎉 All challenges complete!");
}
