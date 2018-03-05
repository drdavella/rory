/*
 * DEBUG FUNCTIONS
 */
#[cfg(feature = "debug")]
pub type Debug = String;

#[cfg(feature = "debug")]
macro_rules! debug_format {
    ( $( $args:tt )+ ) => {
        format!($($args)*);
    }
}

/*
 * NORMAL FUNCTIONS
 */
#[cfg(not(feature = "debug"))]
pub type Debug = ();

#[cfg(not(feature = "debug"))]
macro_rules! debug_format {
    () => ()
}
