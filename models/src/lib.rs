mod model_prelude {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};
}

mod challenge;
mod event;
mod team;

pub use challenge::*;
pub use event::*;
pub use team::*;
