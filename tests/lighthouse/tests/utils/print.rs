fn format_hex(data: &[u8]) -> String {
    let mut result = String::new();
    for (i, chunk) in data.chunks(32).enumerate() {
        // Write the offset
        result.push_str(&format!("{:08x} ({:08}): ", i * 32, i * 32));

        // Write each byte in the chunk
        for byte in chunk {
            result.push_str(&format!("{:02x} ", byte));
        }

        // Add a new line
        result.push('\r');
        result.push('\n');
    }
    result
}
