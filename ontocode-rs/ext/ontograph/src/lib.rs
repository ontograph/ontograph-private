mod extension;
mod indexer;
mod manifest;
mod query;
mod tool;

pub use extension::install;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
