use std::fs;
use std::path::Path;
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<&str> = vec!["", "docs/provider/openrouter.yaml"];

    let input_path = &args[1];
    let default_output_dir = "docs/protocol";
    let output_path = if args.len() > 2 {
        &args[2]
    } else {
        default_output_dir
    };

    let output_dir = Path::new(output_path);

    println!("Reading file: {}", input_path);

    let content = fs::read_to_string(input_path)?;
    let docs = YamlLoader::load_from_str(&content)?;

    if docs.is_empty() {
        eprintln!("No YAML documents found in file.");
        return Ok(());
    }

    let doc = &docs[0];

    // Navigate to components -> schemas
    let components = &doc["components"];
    if components.is_badvalue() {
        eprintln!("'components' key not found.");
        return Ok(());
    }

    let schemas = &components["schemas"];
    if schemas.is_badvalue() {
        eprintln!("'schemas' key not found under 'components'.");
        return Ok(());
    }

    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
        println!("Created directory: {}", output_dir.display());
    }

    if let Yaml::Hash(schema_map) = schemas {
        let mut count = 0;
        for (key, value) in schema_map {
            if let Yaml::String(name) = key {
                let mut out_str = String::new();
                {
                    let mut emitter = YamlEmitter::new(&mut out_str);
                    emitter.dump(value)?;
                }

                // Sanitize filename
                let safe_name =
                    name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "_");
                let file_name = format!("{}.yaml", safe_name);
                let file_path = output_dir.join(&file_name);

                fs::write(&file_path, &out_str)?;
                count += 1;
            }
        }
        println!(
            "Successfully extracted {} schemas to {}",
            count,
            output_dir.display()
        );
    } else {
        eprintln!("'schemas' is not a hash map.");
    }

    Ok(())
}
