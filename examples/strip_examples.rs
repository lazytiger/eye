use std::fs;
use std::path::Path;
use yaml_rust2::{Yaml, YamlLoader, YamlEmitter};

fn main() -> anyhow::Result<()> {
    let dir = Path::new("docs/provider");
    if dir.exists() && dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "yaml" || ext == "yml") {
                println!("Processing {:?}", path);
                let content = fs::read_to_string(&path)?;
                
                // Load YAML
                let docs = YamlLoader::load_from_str(&content);
                match docs {
                    Ok(docs) => {
                         let mut new_content = String::new();
                        for (i, doc) in docs.into_iter().enumerate() {
                            if i > 0 {
                                new_content.push_str("---\n");
                            }
                            let mut doc = doc;
                            strip_examples(&mut doc);
                            
                            let mut out_str = String::new();
                            {
                                let mut emitter = YamlEmitter::new(&mut out_str);
                                emitter.dump(&doc).unwrap();
                            }
                            new_content.push_str(&out_str);
                            new_content.push('\n');
                        }
                        fs::write(&path, new_content)?;
                    }
                    Err(e) => {
                        eprintln!("Failed to parse {:?}: {}", path, e);
                    }
                }
            }
        }
    }
    Ok(())
}

fn strip_examples(doc: &mut Yaml) {
    match doc {
        Yaml::Hash(hash) => {
            // Remove "example" key
            // Note: yaml-rust2 uses Yaml as key.
            hash.remove(&Yaml::String("example".to_string()));
            hash.remove(&Yaml::String("examples".to_string()));
            
            // Recurse
            for (_, v) in hash.iter_mut() {
                strip_examples(v);
            }
        }
        Yaml::Array(arr) => {
            for v in arr.iter_mut() {
                strip_examples(v);
            }
        }
        _ => {}
    }
}
