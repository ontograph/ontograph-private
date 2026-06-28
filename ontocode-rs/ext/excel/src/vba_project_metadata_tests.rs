use std::path::Path;

use pretty_assertions::assert_eq;
use tempfile::tempdir;

use crate::tests::VbaFixtureModuleType;
use crate::tests::build_minimal_vba_project_bin;
use crate::tests::build_vba_dir_stream;
use crate::tests::build_vba_dir_stream_with_modules;
use crate::tests::compress_ovba_literal_only;
use crate::tests::write_zip_fixture_bytes;
use crate::vba_project_metadata::InspectVbaProjectMetadataResult;
use crate::vba_project_metadata::VbaProjectReferenceCounts;
use crate::vba_project_metadata::inspect_vba_project_metadata_from_workbook;

#[test]
fn inspect_vba_project_metadata_summarizes_module_types() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("sample.xlsm");
    write_zip_fixture_bytes(
        &workbook_path,
        &[
            (
                "[Content_Types].xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                  <Override PartName="/xl/vbaProject.bin" ContentType="application/vnd.ms-office.vbaProject"/>
                </Types>"#
                .to_vec(),
            ),
            (
                "xl/vbaProject.bin",
                build_minimal_vba_project_bin(&[
                    (
                        "dir".to_string(),
                        compress_ovba_literal_only(&build_vba_dir_stream("MyModule", 0x04E4)),
                    ),
                    (
                        "MyModule".to_string(),
                        compress_ovba_literal_only(
                            b"Attribute VB_Name = \"MyModule\"\r\nFunction Square(x As Double) As Double\r\n  Square = x * x\r\nEnd Function\r\n",
                        ),
                    ),
                ]),
            ),
        ],
    );

    let result =
        inspect_vba_project_metadata_from_workbook(&workbook_path, Path::new("sample.xlsm"));

    assert_eq!(
        result,
        InspectVbaProjectMetadataResult {
            mode: "read_only_inspection".to_string(),
            path: "sample.xlsm".to_string(),
            has_vba_project: true,
            code_page: Some(1252),
            module_count: 1,
            procedural_module_count: 1,
            doc_cls_designer_module_count: 0,
            module_names: vec!["MyModule".to_string()],
            module_names_truncated: false,
            doc_cls_designer_module_names: Vec::new(),
            doc_cls_designer_module_names_truncated: false,
            reference_counts: VbaProjectReferenceCounts {
                control: 0,
                original: 0,
                registered: 0,
                project: 0,
            },
            warnings: Vec::new(),
        }
    );
}

#[test]
fn inspect_vba_project_metadata_reports_bounded_module_samples() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("sample.xlsm");
    let module_specs = (0..34)
        .map(|index| {
            let name = format!("Module{index}");
            let module_type = if index < 18 {
                VbaFixtureModuleType::DocClsDesigner
            } else {
                VbaFixtureModuleType::Procedural
            };
            (name, module_type)
        })
        .collect::<Vec<_>>();
    let dir_modules = module_specs
        .iter()
        .map(|(name, module_type)| (name.as_str(), *module_type))
        .collect::<Vec<_>>();
    let mut streams = vec![(
        "dir".to_string(),
        compress_ovba_literal_only(&build_vba_dir_stream_with_modules(&dir_modules, 0x04E4)),
    )];
    for (name, _) in &module_specs {
        streams.push((
            name.clone(),
            compress_ovba_literal_only(format!("Attribute VB_Name = \"{name}\"\r\n").as_bytes()),
        ));
    }

    write_zip_fixture_bytes(
        &workbook_path,
        &[
            (
                "[Content_Types].xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                  <Override PartName="/xl/vbaProject.bin" ContentType="application/vnd.ms-office.vbaProject"/>
                </Types>"#
                .to_vec(),
            ),
            ("xl/vbaProject.bin", build_minimal_vba_project_bin(&streams)),
        ],
    );

    let result =
        inspect_vba_project_metadata_from_workbook(&workbook_path, Path::new("sample.xlsm"));

    assert_eq!(result.module_count, 34);
    assert_eq!(result.procedural_module_count, 16);
    assert_eq!(result.doc_cls_designer_module_count, 18);
    assert_eq!(result.module_names.len(), 32);
    assert_eq!(result.doc_cls_designer_module_names.len(), 16);
    assert!(result.module_names_truncated);
    assert!(result.doc_cls_designer_module_names_truncated);
    assert_eq!(
        result.warnings,
        vec![
            "doc_cls_designer modules may represent document, class, or designer/forms metadata; current parser does not distinguish them further".to_string(),
            "module_names is a bounded sample capped at 32 entries".to_string(),
            "doc_cls_designer_module_names is a bounded sample capped at 16 entries".to_string(),
        ]
    );
}
