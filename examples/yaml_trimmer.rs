use anyhow::Context;
use eye::OptionToResult;
use serde_yaml::Value;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

fn main() -> anyhow::Result<()> {
    // 1. Read and parse input YAML
    let paths = vec!["/chat/completions", "/embeddings"];
    let file = File::open("docs/provider/openai.yaml")?;
    let reader = BufReader::new(file);
    let mut input_yaml: Value = serde_yaml::from_reader(reader)?;

    // 3. Process specified paths
    let root = input_yaml
        .as_mapping_mut()
        .to_ok()
        .context("root must be a mapping")?;
    let paths_root = root.get_mut("paths").to_ok().context("paths not found")?;
    {
        let paths_root_map = paths_root
            .as_mapping_mut()
            .to_ok()
            .context("paths must be a mapping")?;
        paths_root_map.retain(|key, value| {
            let retain = paths.contains(&key.as_str().unwrap_or_default());
            if retain {
                let value = value.as_mapping_mut().unwrap();
                let value = value.get_mut("post").unwrap().as_mapping_mut().unwrap();
                let value = value
                    .get_mut("responses")
                    .unwrap()
                    .as_mapping_mut()
                    .unwrap();
                value.retain(|key, _| key.as_str().unwrap_or_default() == "200");
            }
            retain
        });
    }
    let mut keys = HashSet::new();
    find_refs(paths_root, &mut keys);

    let schemas = root
        .get_mut("components")
        .to_ok()
        .context("components not found")?
        .get_mut("schemas")
        .to_ok()
        .context("schemas not found")?;
    let schemas_map = schemas
        .as_mapping()
        .to_ok()
        .context("schemas must be a mapping")?;
    let mut schemas_keys = keys.clone();
    while let Some(key) = schemas_keys.iter().next().cloned() {
        let value = schemas_map
            .get(&key)
            .to_ok()
            .context(format!("schema {} not found", key))?;
        let mut new_keys = HashSet::new();
        find_refs(value, &mut new_keys);
        schemas_keys.remove(&key);
        schemas_keys.extend(new_keys.clone());
        keys.extend(new_keys);
    }

    schemas
        .as_mapping_mut()
        .to_ok()?
        .retain(|key, _| keys.contains(key.as_str().unwrap_or_default()));

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("docs/provider/openai.trimmed.yaml")?;
    let writer = BufWriter::new(file);
    serde_yaml::to_writer(writer, &input_yaml)?;
    Ok(())
}

fn find_refs(value: &Value, refs: &mut HashSet<String>) {
    match value {
        Value::Mapping(map) => {
            for (k, v) in map {
                // Check if key is "$ref"
                if let Value::String(key_str) = k {
                    if key_str == "$ref" {
                        if let Value::String(ref_str) = v {
                            if let Some(schema_name) = parse_schema_name(ref_str) {
                                refs.insert(schema_name);
                            }
                        }
                    }
                }
                // Recursively search in values
                find_refs(v, refs);
            }
        }
        Value::Sequence(seq) => {
            for v in seq {
                find_refs(v, refs);
            }
        }
        _ => {}
    }
}

fn parse_schema_name(ref_str: &str) -> Option<String> {
    if ref_str.starts_with("#/components/schemas/") {
        Some(
            ref_str
                .trim_start_matches("#/components/schemas/")
                .to_string(),
        )
    } else {
        None
    }
}
