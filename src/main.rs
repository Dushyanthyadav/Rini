// 1. Declare the module (tells rustc to look for src/parser.rs)
mod parser;

// 2. Import the public items you need into the current scope
use parser::{Event, Parser, Error};

fn main() {
    let my_config = "
;System Identity Configuration
[user_info]
user name = Dushyanth yadav v
environment = arch linux

[custom_shell]
# Core environment variables
PROMPT_STYLE=minimal
display graphics =
";

    println!("Starting Parser Engine...\n");
    
    // Instantiate your parser using the public constructor
    let parser = Parser::new(my_config);

    // Consume the iterator
    for event in parser {
        match event {
            Ok(Event::Section(name)) => {
                println!("📦 Section: [{}]", name);
            }
            Ok(Event::Property((key, value))) => {
                println!("   🔑 Key: '{}' | 📄 Value: '{}'", key, value);
            }
            Ok(Event::Comment(text)) => {
                println!("   💬 Comment: {}", text);
            }
            Err(error) => {
                match error {
                    Error::UnexpectedCharacter(pos) => {
                        println!("❌ Syntax Error: Unexpected character found at byte index {}.", pos);
                    }
                    Error::MissingBracket(pos) => {
                        println!("❌ Syntax Error: Missing closing bracket ']' detected at byte index {}.", pos);
                    }
                }
                break;
            }
        }
    }
    
    println!("\nParsing Complete.");
}