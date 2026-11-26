use std::collections::HashMap;

pub type CommandFN = fn(&[&str]) -> bool;
pub type CommandRegistry = HashMap<&'static str, CommandFN>;
