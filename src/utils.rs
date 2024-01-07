use std::io::Write;

pub fn ask_confirmation(text: &str, buffer: &mut String) -> Result<bool, std::io::Error> {
    buffer.clear();

    print!("{text} [y/N]: ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(buffer)?;
    Ok(buffer.trim().to_lowercase().starts_with('y'))
}
