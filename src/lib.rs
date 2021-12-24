#![feature(bool_to_option)]
#![feature(string_remove_matches)]
#![feature(array_from_fn)]
#![feature(map_first_last)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::comparison_chain)]

pub use assets_manager::{loader::RonLoader, Asset, AssetCache};
pub use bitvec::prelude::*;
pub use itertools::Itertools;
pub use pathfinding::prelude::*;
pub use serde::Deserialize;
pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
pub use vek::*;

pub mod days;
pub mod util;

pub use util::get_input;
