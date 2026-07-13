//! Generic select branching utilities to resolve multiple futures into self-documenting inline handlers.

/// Selects over either two or three futures and executes the matching inline branch handler.
///
/// Supports either 1-argument closures (e.g. `|val| ...`) or 0-argument closures (e.g. `|| ...`)
/// for each branch.
#[macro_export]
macro_rules! select_branch {
    // === 2-BRANCH SELECT ===

    // 1. || and ||
    (
        $f1:expr => || $r1:expr,
        $f2:expr => || $r2:expr $(,)?
    ) => {
        match $crate::select::select($f1, $f2).await {
            $crate::select::Either::First(_) => $r1,
            $crate::select::Either::Second(_) => $r2,
        }
    };

    // 2. || and |val|
    (
        $f1:expr => || $r1:expr,
        $f2:expr => |$v2:ident| $r2:expr $(,)?
    ) => {
        match $crate::select::select($f1, $f2).await {
            $crate::select::Either::First(_) => $r1,
            $crate::select::Either::Second(val) => {
                let $v2 = val;
                $r2
            }
        }
    };

    // 3. |val| and ||
    (
        $f1:expr => |$v1:ident| $r1:expr,
        $f2:expr => || $r2:expr $(,)?
    ) => {
        match $crate::select::select($f1, $f2).await {
            $crate::select::Either::First(val) => {
                let $v1 = val;
                $r1
            }
            $crate::select::Either::Second(_) => $r2,
        }
    };

    // 4. |val| and |val|
    (
        $f1:expr => |$v1:ident| $r1:expr,
        $f2:expr => |$v2:ident| $r2:expr $(,)?
    ) => {
        match $crate::select::select($f1, $f2).await {
            $crate::select::Either::First(val) => {
                let $v1 = val;
                $r1
            }
            $crate::select::Either::Second(val) => {
                let $v2 = val;
                $r2
            }
        }
    };

    // === 3-BRANCH SELECT ===

    // 1. || and || and ||
    (
        $f1:expr => || $r1:expr,
        $f2:expr => || $r2:expr,
        $f3:expr => || $r3:expr $(,)?
    ) => {
        match $crate::select::select3($f1, $f2, $f3).await {
            $crate::select::Either3::First(_) => $r1,
            $crate::select::Either3::Second(_) => $r2,
            $crate::select::Either3::Third(_) => $r3,
        }
    };

    // 2. || and || and |val|
    (
        $f1:expr => || $r1:expr,
        $f2:expr => || $r2:expr,
        $f3:expr => |$v3:ident| $r3:expr $(,)?
    ) => {
        match $crate::select::select3($f1, $f2, $f3).await {
            $crate::select::Either3::First(_) => $r1,
            $crate::select::Either3::Second(_) => $r2,
            $crate::select::Either3::Third(val) => {
                let $v3 = val;
                $r3
            }
        }
    };

    // 3. || and |val| and ||
    (
        $f1:expr => || $r1:expr,
        $f2:expr => |$v2:ident| $r2:expr,
        $f3:expr => || $r3:expr $(,)?
    ) => {
        match $crate::select::select3($f1, $f2, $f3).await {
            $crate::select::Either3::First(_) => $r1,
            $crate::select::Either3::Second(val) => {
                let $v2 = val;
                $r2
            }
            $crate::select::Either3::Third(_) => $r3,
        }
    };

    // 4. || and |val| and |val|
    (
        $f1:expr => || $r1:expr,
        $f2:expr => |$v2:ident| $r2:expr,
        $f3:expr => |$v3:ident| $r3:expr $(,)?
    ) => {
        match $crate::select::select3($f1, $f2, $f3).await {
            $crate::select::Either3::First(_) => $r1,
            $crate::select::Either3::Second(val) => {
                let $v2 = val;
                $r2
            }
            $crate::select::Either3::Third(val) => {
                let $v3 = val;
                $r3
            }
        }
    };

    // 5. |val| and || and ||
    (
        $f1:expr => |$v1:ident| $r1:expr,
        $f2:expr => || $r2:expr,
        $f3:expr => || $r3:expr $(,)?
    ) => {
        match $crate::select::select3($f1, $f2, $f3).await {
            $crate::select::Either3::First(val) => {
                let $v1 = val;
                $r1
            }
            $crate::select::Either3::Second(_) => $r2,
            $crate::select::Either3::Third(_) => $r3,
        }
    };

    // 6. |val| and || and |val|
    (
        $f1:expr => |$v1:ident| $r1:expr,
        $f2:expr => || $r2:expr,
        $f3:expr => |$v3:ident| $r3:expr $(,)?
    ) => {
        match $crate::select::select3($f1, $f2, $f3).await {
            $crate::select::Either3::First(val) => {
                let $v1 = val;
                $r1
            }
            $crate::select::Either3::Second(_) => $r2,
            $crate::select::Either3::Third(val) => {
                let $v3 = val;
                $r3
            }
        }
    };

    // 7. |val| and |val| and ||
    (
        $f1:expr => |$v1:ident| $r1:expr,
        $f2:expr => |$v2:ident| $r2:expr,
        $f3:expr => || $r3:expr $(,)?
    ) => {
        match $crate::select::select3($f1, $f2, $f3).await {
            $crate::select::Either3::First(val) => {
                let $v1 = val;
                $r1
            }
            $crate::select::Either3::Second(val) => {
                let $v2 = val;
                $r2
            }
            $crate::select::Either3::Third(_) => $r3,
        }
    };

    // 8. |val| and |val| and |val|
    (
        $f1:expr => |$v1:ident| $r1:expr,
        $f2:expr => |$v2:ident| $r2:expr,
        $f3:expr => |$v3:ident| $r3:expr $(,)?
    ) => {
        match $crate::select::select3($f1, $f2, $f3).await {
            $crate::select::Either3::First(val) => {
                let $v1 = val;
                $r1
            }
            $crate::select::Either3::Second(val) => {
                let $v2 = val;
                $r2
            }
            $crate::select::Either3::Third(val) => {
                let $v3 = val;
                $r3
            }
        }
    };
}

/// Wraps a future with a timeout.
///
/// Returns a new future that resolves to `Some(val)` if the future completes,
/// or `None` if the timeout occurs first.
#[macro_export]
macro_rules! with_timeout {
    ($fut:expr, $dur:expr) => {
        async {
            match $crate::select::select($fut, $crate::select::Timer::after($dur)).await {
                $crate::select::Either::First(val) => Some(val),
                $crate::select::Either::Second(_) => None,
            }
        }
    };
}

/// Selects over two futures with a timeout, executing the matching inline branch handler.
///
/// Wraps the 2-branch `select_branch!` inside `with_timeout!`. Returns `Some(result)` if
/// one of the branches completed, or `None` if the timeout elapsed.
#[macro_export]
macro_rules! select_branch_with_timeout {
    (
        $dur:expr,
        $($tokens:tt)*
    ) => {
        $crate::with_timeout!(
            async {
                $crate::select_branch!($($tokens)*)
            },
            $dur
        ).await.flatten()
    };
}

// Re-export select, select3, Either, Either3, and Timer so they are accessible to the macros
#[doc(hidden)]
pub use embassy_futures::select::{select, select3, Either, Either3};
#[doc(hidden)]
pub use embassy_time::Timer;
