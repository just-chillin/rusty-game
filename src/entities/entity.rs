use std::collections::{HashMap, HashSet};
use std::borrow::BorrowMut;
use std::ptr::null;
use std::error::Error;
use std::hash::{Hash, Hasher};

pub trait Entity {
    fn tick(&self);
}

