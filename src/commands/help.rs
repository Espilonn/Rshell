use super::command_registry::CommandRegistry;

pub fn execute(_args: &[&str]) {
    println!();
    println!("commands:");
    println!("help: affiche cette page");
    println!("exit: ferme le shell");
}

pub fn register(registry: &mut CommandRegistry) {
    registry.insert("help", execute);
}
