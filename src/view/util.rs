use std::io::Write;

pub fn input_line(message: &str, line: &mut String) {
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(line).unwrap();
    line.pop();
    // trim the line
    *line = line.trim().to_string();
} 