mod backend;
mod export;
mod extension;
mod formula_inspect;
mod powerquery_extract;
mod powerquery_translate;
mod preview;
mod tool;
mod vba_extract;
mod vba_onlyoffice_analyze;
mod vba_onlyoffice_translate;
mod vba_onlyoffice_workbook_review;
mod vba_translate;

pub use extension::install;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
