use crate::*;

pub fn get_input<T: assets_manager::Asset>(path: &str) -> T
where
    T: Clone,
{
    let cache = AssetCache::new("assets").unwrap();
    cache.load::<T>(path).unwrap().cloned()
}
