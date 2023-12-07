use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct Preferences {
    pub public_editors: bool,
    pub public_operating_systems: bool,
    pub public_languages: bool,
    pub public_devices: bool,
}
