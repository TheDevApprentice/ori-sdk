//! Cohérence du catalogue public : chaque entrée de `catalog/skills.json`
//! pointe un `SKILL.md` qui respecte les bornes du parseur cœur (ADR-052) —
//! nom validé, `name`/`description` concordants, fichier borné. Une skill qui
//! échouerait ici serait refusée par `parse_document` dans `ori-core`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // fichier de test uniquement

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Mêmes bornes que `ori-core/src/skill.rs` — dupliquées volontairement : le
/// SDK public ne dépend pas du dépôt privé, le test est le garde-fou.
const MAX_SKILL_FILE: u64 = 20 * 1024;
const MAX_SKILL_NAME: usize = 64;
const MAX_SKILL_DESCRIPTION: usize = 1024;
const MAX_SKILL_BODY: usize = 16 * 1024;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn name_ok(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= MAX_SKILL_NAME
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && name
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_alphanumeric())
}

/// Re-joue le strict minimum du frontmatter du parseur cœur.
fn frontmatter(text: &str) -> (&str, &str) {
    let mut lines = text.lines();
    assert_eq!(lines.next(), Some("---"), "première ligne");
    let mut name = None;
    let mut description = None;
    for line in lines.by_ref() {
        if line == "---" {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            let value = value.trim();
            match key.trim() {
                "name" => name = Some(value),
                "description" => description = Some(value),
                _ => {}
            }
        }
    }
    (name.unwrap(), description.unwrap())
}

#[test]
fn catalog_entries_resolve_to_bounded_valid_skill_files() {
    let root = root();
    let catalog_path = root.join("catalog/skills.json");
    let catalog: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&catalog_path).unwrap()).unwrap();
    let entries = catalog["skills"].as_array().expect("skills[] requis");
    assert!(!entries.is_empty(), "le catalogue ne doit pas être vide");

    let mut seen = BTreeSet::new();
    for entry in entries {
        let name = entry["name"].as_str().expect("name");
        let path = entry["path"].as_str().expect("path");
        let description = entry["description"].as_str().expect("description");
        assert!(name_ok(name), "nom de catalogue invalide : {name}");
        assert!(seen.insert(name.to_owned()), "doublon : {name}");
        assert!(
            !path.contains("..") && path.starts_with("skills/"),
            "path : {path}"
        );

        let file_path = root.join(path);
        let meta = std::fs::metadata(&file_path).expect("fichier du catalogue absent");
        assert!(meta.len() <= MAX_SKILL_FILE, "{name} au-delà de la borne");
        let text = std::fs::read_to_string(&file_path).unwrap();
        let (front_name, front_description) = frontmatter(&text);
        assert_eq!(front_name, name, "{name} : frontmatter ≠ catalogue");
        assert_eq!(front_description, description, "{name} : description ≠");
        assert!(front_description.len() <= MAX_SKILL_DESCRIPTION);
        // Le corps (après le second délimiteur) respecte la borne du parseur.
        let body_start = text
            .match_indices("---")
            .nth(1)
            .map(|(i, _)| i + 3)
            .unwrap();
        assert!(
            text.len() - body_start <= MAX_SKILL_BODY,
            "{name} : corps au-delà de la borne"
        );
    }
}

#[test]
fn every_skill_directory_is_in_the_catalog() {
    let root = root();
    let catalog: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("catalog/skills.json")).unwrap())
            .unwrap();
    let listed: BTreeSet<String> = catalog["skills"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|entry| entry["name"].as_str().map(str::to_owned))
        .collect();
    for dir in std::fs::read_dir(root.join("skills")).unwrap().flatten() {
        if !dir.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        let name = dir.file_name().into_string().unwrap();
        assert!(listed.contains(&name), "{name} absent du catalogue");
        assert!(
            dir.path().join("SKILL.md").is_file(),
            "{name} : SKILL.md absent"
        );
    }
}
