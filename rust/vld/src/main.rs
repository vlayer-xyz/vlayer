fn main() {
    let vlayer_path = get_vlayer_path();
    println!("vlayer_path: {}", vlayer_path);
}

fn get_vlayer_path() -> String {
    let home_dir = std::env::var("HOME").expect("Failed to get home directory");
    let vld_path = std::path::Path::new(&home_dir).join(".vld");
    let content = std::fs::read_to_string(vld_path)
        .expect("Failed to read ~/.vld");
    
    let vlayer_path = content
        .lines()
        .find(|line| line.starts_with("VLAYER_PATH="))
        .map(|line| line.trim_start_matches("VLAYER_PATH=").trim().to_string())
        .expect("Could not find VLAYER_PATH in ~/.vld");

    if vlayer_path.is_empty() {
        panic!("VLAYER_PATH value in ~/.vld is empty");
    }
    vlayer_path
}

