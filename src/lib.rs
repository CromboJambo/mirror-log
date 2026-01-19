// Re-export modules for library use
pub mod chunk;
pub mod db;
pub mod log;
pub mod view;

// Re-export the main functionality
pub use chunk::{create_chunks, search_chunks};
pub use db::{db_info, init_db};
pub use log::{append, append_stdin};
pub use view::{by_source, get_by_id, recent, search};
