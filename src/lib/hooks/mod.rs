mod cfg;
mod entries;
mod shared;

pub use cfg::{get_config, reset_config, CocapConfig};
pub use entries::{add_entry, clear_entries, find_entry, load_entries, CocapEntry};
