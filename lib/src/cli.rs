//! Shared CLI utilities and helper macros.

/// Helper macro to generate CLI subcommand enums and implement FromArgument.
#[macro_export]
macro_rules! subcommand_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident $(= $str_val:literal)?
            ),* $(,)?
        }
        $err_msg:literal
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        $vis enum $name {
            $($(#[$vmeta])* $variant,)*
        }

        impl<'a> $crate::embedded_cli::arguments::FromArgument<'a> for $name {
            fn from_arg(arg: &'a str) -> Result<Self, $crate::embedded_cli::arguments::FromArgumentError<'a>> {
                $(
                    if $crate::subcommand_enum!(@match_variant arg, $variant $(, $str_val)?) {
                        return Ok(Self::$variant);
                    }
                )*
                Err($crate::embedded_cli::arguments::FromArgumentError {
                    value: arg,
                    expected: $err_msg,
                })
            }
        }
    };

    (@match_variant $val:ident, $var:ident, $str_val:literal) => {
        $val.eq_ignore_ascii_case($str_val)
    };
    (@match_variant $val:ident, $var:ident) => {
        $val.eq_ignore_ascii_case(stringify!($var))
    };
}
