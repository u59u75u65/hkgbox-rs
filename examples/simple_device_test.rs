//! Simple test of device ID generation

fn generate_device_id() -> String {
    use std::time::SystemTime;
    use std::process;

    // Alphanumeric character set (0-9, a-z)
    const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

    // Create unique seeds from time and process
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let process_id = process::id() as u128;

    // Combine multiple entropy sources
    let mut state = timestamp as u128;
    state = state.wrapping_mul(31).wrapping_add(process_id);
    state = state.wrapping_mul(37).wrapping_add(timestamp % 999999999);

    // Generate 40-character alphanumeric string
    let mut result = String::with_capacity(40);
    for i in 0..40 {
        let index = (state.wrapping_mul(i as u128 + 1) % CHARS.len() as u128) as usize;
        result.push(CHARS[index] as char);
    }

    result
}

fn main() {
    let device_id = generate_device_id();
    println!("Generated: {}", device_id);
    println!("Length: {}", device_id.len());
    println!("Is alphanumeric: {}", device_id.chars().all(|c| c.is_alphanumeric()));
    println!("Is lowercase: {}", device_id.chars().all(|c| c.is_ascii_lowercase()));

    // Print each character
    for (i, c) in device_id.chars().enumerate() {
        println!("Char {}: '{}' ({}), is_lowercase: {}",
            i, c, c as u32, c.is_ascii_lowercase());
    }
}