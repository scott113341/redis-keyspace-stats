use crate::data::Keys;

pub fn example_keys(keys: &Keys) -> Keys {
    keys.clone().into_iter().take(5).collect()
}
