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

#[cfg(feature = "debug")]
macro_rules! debug_println {
    ( $( $args:tt )+ ) => {
        println!($($args)*);
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

#[cfg(not(feature = "debug"))]
macro_rules! debug_println {
    ( $( $args:tt )+ ) => {
        () => ()
    }
}
