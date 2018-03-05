#[cfg(feature = "debug")]
pub type Debug = String;

#[cfg(not(feature = "debug"))]
pub type Debug = ();
