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

/// Parses a boolean parameter from a string argument.
///
/// Supports "on", "off", "true", "false", "1", "0" (case-insensitive).
///
/// # Errors
/// Returns an error message string if the input string cannot be parsed as a boolean.
pub fn parse_bool_arg(arg: &str) -> Result<bool, &'static str> {
    if arg.eq_ignore_ascii_case("on") || arg.eq_ignore_ascii_case("true") || arg == "1" {
        Ok(true)
    } else if arg.eq_ignore_ascii_case("off") || arg.eq_ignore_ascii_case("false") || arg == "0" {
        Ok(false)
    } else {
        Err("Expected 'on', 'off', 'true', 'false', '1', or '0'")
    }
}
