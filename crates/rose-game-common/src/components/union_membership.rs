use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct UnionMembership {
    pub current_union: Option<NonZeroUsize>,
    pub points: [u32; 10],
}

impl UnionMembership {
    pub fn new() -> Self {
        Default::default()
    }
}
