mod cfg;
mod entries;
mod shared;

pub use cfg::{get_config, CocapConfig};
pub use entries::{add_entry, find_entry, CocapEntry};
