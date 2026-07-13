//! Shared CLI utilities and helper macros.

/// Helper macro to generate CLI subcommand enums and implement TryFrom<&str>.
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
        #[derive(Clone, Copy, PartialEq, Eq)]
        $vis enum $name {
            $($(#[$vmeta])* $variant,)*
        }

        impl TryFrom<&str> for $name {
            type Error = &'static str;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                $(
                    if $crate::subcommand_enum!(@match_variant value, $variant $(, $str_val)?) {
                        return Ok(Self::$variant);
                    }
                )*
                Err($err_msg)
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
