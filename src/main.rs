mod commands;
use std::io;
use std::io::Write;

fn parsing(prompt: &str) -> Vec<&str> {
    let args: Vec<&str> = prompt.split_whitespace().collect();
    return args;
}

fn shell_loop(registry: &mut commands::command_registry::CommandRegistry) {
    loop {
        println!("\n Rshell");
        print!("Rshell>");
        io::stdout().flush().unwrap();

        let mut prompt = String::new();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to readline");

        let args = parsing(&prompt);
        if let Some(command) = registry.get(args[0]) {
            let running = command(&args[1..]);
            if !running {
                break;
            }
        } else {
            println!("commande inconnue");
        }
    }
}

fn main() {
    let mut registry = commands::command_registry::CommandRegistry::new();
    commands::register_all(&mut registry);
    shell_loop(&mut registry)
}
