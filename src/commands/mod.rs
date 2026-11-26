pub mod command_registry;
pub mod help;

use self::command_registry::CommandRegistry;

pub fn register_all(registry: &mut CommandRegistry) {
    help::register(registry);
}
