use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use serde::{Deserialize, Deserializer};

pub(super) type ID = ValueWrapper<i32>;
pub(super) type Rarity = ValueWrapper<i32>;
pub(super) type Level = ValueWrapper<i32>;
pub(super) type Amount = ValueWrapper<i32>;
pub(super) type Position = ValueWrapper<i32>;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub(super) struct ValueWrapper<T>(#[serde(deserialize_with = "from_str")] T)
where
    T: FromStr + Copy,
    T::Err: Display;

impl<T> Deref for ValueWrapper<T>
where
    T: FromStr + Copy,
    T::Err: Display,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Debug)]
pub(super) struct Collection<T>(HashMap<ID, T>);

impl<T> Deref for Collection<T> {
    type Target = HashMap<ID, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
