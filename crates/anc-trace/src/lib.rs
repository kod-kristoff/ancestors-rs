// use tracing::span;
/// The level at which the tracing item should be created.
///
/// It's used to filter items early.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Level {
    /// A coarse-grained trace level, one that should span entire operations with low frequency.
    Coarse = 1,
    /// Finer grained trace level that further subdivides coarse-level traces.
    ///
    /// Note that these should only be created for areas of the code which have significant cost.
    Detail = 2,
}

/// The maximum allowed level for tracing items, as compiled in.
pub const MAX_LEVEL: Level = Level::Detail;

mod enabled;

pub use enabled::{field, Span};

impl Span {
    /// Execute `f` in with this span active, consuming it.
    pub fn into_scope<T>(self, f: impl FnOnce() -> T) -> T {
        f()
    }
}

#[doc(hidden)]
pub use enabled::{metadata, Event, MetaOnlyCallsite, Metadata};

#[macro_export]
macro_rules! coarse {
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::Coarse,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::coarse!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::Coarse,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::coarse!($name,)};
}

/// Create a new [detail][Level::Detail] span.
#[macro_export]
macro_rules! detail {
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::Detail,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::detail!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::Detail,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::coarse!($name,)};
}
