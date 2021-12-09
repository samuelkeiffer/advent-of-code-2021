#![feature(bool_to_option)]

pub use assets_manager::{loader::RonLoader, Asset, AssetCache};
pub use itertools::Itertools;
pub use serde::Deserialize;
pub use std::collections::{BTreeMap, HashMap, HashSet};

pub mod days;
pub mod util;

pub use util::get_input;
