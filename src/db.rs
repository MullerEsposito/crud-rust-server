use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use crate::types::Database;

pub static DB: Lazy<Database> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});
