// Marked as private and child branches re-exported so they
// can re-export common structures defined in their parent,
// and also be able to define their own.
mod valve;

pub use valve::{orange_box, source};

macro_rules! define_lump_definitions {
    ($($variant:ident = $val:expr),*,) => {
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
        pub enum LumpDefinition {
            $($variant = $val),*
        }

        impl From<LumpDefinition> for usize {
            fn from(def: LumpDefinition) -> usize {
                match def {
                    $(LumpDefinition::$variant => $val),*
                }
            }
        }

        impl std::fmt::Display for LumpDefinition {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, stringify!($variant))),*
                }
            }
        }
    };
}

pub(crate) use define_lump_definitions;
