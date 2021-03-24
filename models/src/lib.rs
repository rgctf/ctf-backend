mod model_prelude {
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};
}

mod team;

pub use team::*;
