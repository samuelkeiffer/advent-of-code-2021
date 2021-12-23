#![feature(bool_to_option)]
#![feature(string_remove_matches)]
#![feature(array_from_fn)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::wrong_self_convention)]

pub use assets_manager::{loader::RonLoader, Asset, AssetCache};
pub use bitvec::prelude::*;
pub use itertools::Itertools;
pub use serde::Deserialize;
pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
pub use vek::*;

pub mod days;
pub mod util;

pub use util::get_input;
