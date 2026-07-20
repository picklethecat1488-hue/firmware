//! Generic select branching utilities to resolve multiple futures into self-documenting inline handlers.

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
    // === 2-BRANCH MATCH PATTERNS ===

    // 1. |val| and ||
    (
        $dur:expr,
        $f1:expr => |$v1:ident| $r1:expr,
        $f2:expr => || $r2:expr $(,)?
    ) => {
        match $crate::with_timeout!(
            async {
                match $crate::select::select($f1, $f2).await {
                    $crate::select::Either::First(val) => $crate::select::Either::First(val),
                    $crate::select::Either::Second(_) => $crate::select::Either::Second(()),
                }
            },
            $dur
        )
        .await
        {
            Some($crate::select::Either::First(val)) => {
                let $v1 = val;
                $r1
            }
            Some($crate::select::Either::Second(_)) => $r2,
            None => None,
        }
    };

    // 2. || and ||
    (
        $dur:expr,
        $f1:expr => || $r1:expr,
        $f2:expr => || $r2:expr $(,)?
    ) => {
        match $crate::with_timeout!(
            async {
                match $crate::select::select($f1, $f2).await {
                    $crate::select::Either::First(_) => $crate::select::Either::First(()),
                    $crate::select::Either::Second(_) => $crate::select::Either::Second(()),
                }
            },
            $dur
        )
        .await
        {
            Some($crate::select::Either::First(_)) => $r1,
            Some($crate::select::Either::Second(_)) => $r2,
            None => None,
        }
    };

    // 3. || and |val|
    (
        $dur:expr,
        $f1:expr => || $r1:expr,
        $f2:expr => |$v2:ident| $r2:expr $(,)?
    ) => {
        match $crate::with_timeout!(
            async {
                match $crate::select::select($f1, $f2).await {
                    $crate::select::Either::First(_) => $crate::select::Either::First(()),
                    $crate::select::Either::Second(val) => $crate::select::Either::Second(val),
                }
            },
            $dur
        )
        .await
        {
            Some($crate::select::Either::First(_)) => $r1,
            Some($crate::select::Either::Second(val)) => {
                let $v2 = val;
                $r2
            }
            None => None,
        }
    };

    // 4. |val| and |val|
    (
        $dur:expr,
        $f1:expr => |$v1:ident| $r1:expr,
        $f2:expr => |$v2:ident| $r2:expr $(,)?
    ) => {
        match $crate::with_timeout!(
            async {
                match $crate::select::select($f1, $f2).await {
                    $crate::select::Either::First(val) => $crate::select::Either::First(val),
                    $crate::select::Either::Second(val) => $crate::select::Either::Second(val),
                }
            },
            $dur
        )
        .await
        {
            Some($crate::select::Either::First(val)) => {
                let $v1 = val;
                $r1
            }
            Some($crate::select::Either::Second(val)) => {
                let $v2 = val;
                $r2
            }
            None => None,
        }
    };

    // === 3-BRANCH MATCH PATTERNS ===

    // 5. |val|, |val|, and |val|
    (
        $dur:expr,
        $f1:expr => |$v1:ident| $r1:expr,
        $f2:expr => |$v2:ident| $r2:expr,
        $f3:expr => |$v3:ident| $r3:expr $(,)?
    ) => {
        match $crate::with_timeout!(
            async {
                match $crate::select::select3($f1, $f2, $f3).await {
                    $crate::select::Either3::First(val) => $crate::select::Either3::First(val),
                    $crate::select::Either3::Second(val) => $crate::select::Either3::Second(val),
                    $crate::select::Either3::Third(val) => $crate::select::Either3::Third(val),
                }
            },
            $dur
        )
        .await
        {
            Some($crate::select::Either3::First(val)) => {
                let $v1 = val;
                $r1
            }
            Some($crate::select::Either3::Second(val)) => {
                let $v2 = val;
                $r2
            }
            Some($crate::select::Either3::Third(val)) => {
                let $v3 = val;
                $r3
            }
            None => None,
        }
    };
}

// Re-export select, select3, Either, Either3, and Timer so they are accessible to the macros
#[doc(hidden)]
pub use embassy_futures::select::{select, select3, Either, Either3};
#[doc(hidden)]
pub use embassy_time::Timer;
