mod extension;
mod tool;

pub use extension::install;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
