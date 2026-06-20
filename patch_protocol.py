import re

with open("ontocode-rs/app-server-protocol/src/protocol/v2/item.rs", "r") as f:
    code = f.read()

# Add ignored field `generated_file` to ApplyPatch matching
code = code.replace(
    "CoreGuardianAssessmentAction::ApplyPatch { cwd, files } => {",
    "CoreGuardianAssessmentAction::ApplyPatch { cwd, files, generated_file } => {"
)

# Pass along `generated_file`
code = code.replace(
    "Self::ApplyPatch { cwd, files }",
    "Self::ApplyPatch { cwd, files, generated_file }"
)

with open("ontocode-rs/app-server-protocol/src/protocol/v2/item.rs", "w") as f:
    f.write(code)
