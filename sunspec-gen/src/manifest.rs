use std::{fmt::Write as _, fs, path::Path};

use thiserror::Error;

const BEGIN_MARKER: &str = "# BEGIN generated model features";
const END_MARKER: &str = "# END generated model features";

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("Missing marker `{marker}` in Cargo.toml")]
    MissingMarker { marker: &'static str },
    #[error("Invalid generated model feature marker order in Cargo.toml")]
    InvalidMarkerOrder,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn model_feature_name(model_id: u16) -> String {
    format!("model{model_id}")
}

pub fn write_model_features(
    manifest_path: impl AsRef<Path>,
    model_ids: &[u16],
) -> Result<(), ManifestError> {
    let manifest_path = manifest_path.as_ref();
    let manifest = fs::read_to_string(manifest_path)?;
    let updated = update_model_features(&manifest, model_ids)?;
    if manifest != updated {
        fs::write(manifest_path, updated)?;
    }
    Ok(())
}

fn update_model_features(manifest: &str, model_ids: &[u16]) -> Result<String, ManifestError> {
    let Some(begin) = manifest.find(BEGIN_MARKER) else {
        return Err(ManifestError::MissingMarker {
            marker: BEGIN_MARKER,
        });
    };
    let Some(end) = manifest.find(END_MARKER) else {
        return Err(ManifestError::MissingMarker { marker: END_MARKER });
    };
    if begin >= end {
        return Err(ManifestError::InvalidMarkerOrder);
    }

    let generated = render_model_features(model_ids);
    let end = manifest[end..]
        .find('\n')
        .map(|offset| end + offset + 1)
        .unwrap_or(manifest.len());

    let mut updated = String::with_capacity(manifest.len() + generated.len());
    updated.push_str(&manifest[..begin]);
    updated.push_str(&generated);
    if end < manifest.len() {
        updated.push('\n');
        updated.push_str(&manifest[end..]);
    }
    Ok(updated)
}

fn render_model_features(model_ids: &[u16]) -> String {
    let mut output = String::new();
    output.push_str(BEGIN_MARKER);
    output.push('\n');
    output.push_str("all-models = [\n");
    for &model_id in model_ids {
        let feature = model_feature_name(model_id);
        writeln!(output, "    \"{feature}\",").expect("writing to string failed");
    }
    output.push(']');
    output.push('\n');
    for &model_id in model_ids {
        let feature = model_feature_name(model_id);
        writeln!(output, "{feature} = []").expect("writing to string failed");
    }
    output.push_str(END_MARKER);
    output
}

#[cfg(test)]
mod tests {
    use super::update_model_features;

    #[test]
    fn rewrites_generated_model_feature_block() {
        let manifest = r#"[features]
default = ["all-models"]
# BEGIN generated model features
all-models = []
# END generated model features

[dependencies]
"#;

        let updated = update_model_features(manifest, &[1, 103]).unwrap();

        assert!(updated.contains("all-models = [\n    \"model1\",\n    \"model103\",\n]"));
        assert!(updated.contains("model1 = []"));
        assert!(updated.contains("model103 = []"));
        assert!(updated.contains("[dependencies]"));
    }
}
