#![feature(bool_to_option)]
#![feature(string_remove_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::wrong_self_convention)]

pub use assets_manager::{loader::RonLoader, Asset, AssetCache};
pub use itertools::Itertools;
pub use serde::Deserialize;
pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

pub mod days;
pub mod util;

pub use util::get_input;
