mod parser;

fn main() {
    let lines = vec![
        "(unique,0x5380,1) = LOAD (const,0x55e4a78f0330,8) (register,0x0,8)",
    ];

    for line in lines {
        // Use `parse` method to convert the string into `Inst`
        match line.parse::<parser::Inst>() {
            Ok(inst) => {
                // Successfully parsed the line, now you can work with the `inst` object
                println!("Parsed instruction: {:?}", inst);
            }
            Err(_) => {
                // Handle parsing error
                println!("Error parsing the line.");
            }
        }
    }
}
