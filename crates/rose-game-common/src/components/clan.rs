use std::num::{NonZeroU16, NonZeroU32};

use serde::{Deserialize, Serialize};

macro_rules! deref_newtype {
    ($name:ident, $inner:ty) => {
        impl std::ops::Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &$inner {
                &self.0
            }
        }
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }
    };
}
deref_newtype!(ClanUniqueId, NonZeroU32);
deref_newtype!(ClanLevel, NonZeroU32);
deref_newtype!(ClanPoints, u64);

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ClanUniqueId(pub NonZeroU32);

impl ClanUniqueId {
    pub fn new(n: u32) -> Option<ClanUniqueId> {
        NonZeroU32::new(n).map(ClanUniqueId)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ClanLevel(pub NonZeroU32);

impl ClanLevel {
    pub fn new(n: u32) -> Option<ClanLevel> {
        NonZeroU32::new(n).map(ClanLevel)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ClanPoints(pub u64);

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ClanMark {
    Premade {
        background: NonZeroU16,
        foreground: NonZeroU16,
    },
    Custom {
        crc16: u16,
    },
}
