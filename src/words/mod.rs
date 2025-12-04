pub mod comparison;
pub mod io;
pub mod logic;
pub mod math;
pub mod stack;
pub mod stringwords;

// Re-export the register functions for convenience
pub use comparison::register_comparison_words;
pub use io::register_io_words;
pub use logic::register_logic_words;
pub use math::register_math_words;
pub use stack::register_stack_words;
pub use stringwords::register_string_words;
