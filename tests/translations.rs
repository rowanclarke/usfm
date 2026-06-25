extern crate usfm;

use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use zip::ZipArchive;

static MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Deserialize)]
struct Translation {
    url: String,
    script: String,
    code: String,
}

struct TestResult {
    script: String,
    code: String,
    file: String,
    passed: bool,
    error: Option<String>,
}

#[test]
fn parse_sample_translations() {
    let mut rng: StdRng = match std::env::var("USFM_SEED") {
        Ok(seed) => {
            let seed: u64 = seed.parse().expect("USFM_SEED must be a u64");
            eprintln!("Using seed: {seed}");
            StdRng::seed_from_u64(seed)
        }
        Err(_) => {
            let rng = StdRng::from_entropy();
            eprintln!("Using random seed (set USFM_SEED for reproducibility)");
            rng
        }
    };

    let path = PathBuf::from(MANIFEST_DIR).join("usfm/translations.json");
    let json = std::fs::read_to_string(&path).expect("Failed to read translations.json");
    let translations: Vec<Translation> =
        serde_json::from_str(&json).expect("Failed to parse translations.json");

    // Group by script and pick one per script
    let mut by_script: HashMap<String, Vec<&Translation>> = HashMap::new();
    for t in &translations {
        by_script.entry(t.script.clone()).or_default().push(t);
    }

    let mut sampled: Vec<&Translation> = by_script
        .values()
        .filter_map(|group| group.choose(&mut rng).copied())
        .collect();
    sampled.sort_by(|a, b| a.script.cmp(&b.script));

    eprintln!(
        "Sampled {} translations across {} scripts",
        sampled.len(),
        sampled.len()
    );

    let mut results: Vec<TestResult> = Vec::new();

    for translation in &sampled {
        eprintln!(
            "  [{:<20}] Downloading {}...",
            translation.script, translation.code
        );

        let result = process_translation(translation);
        results.push(result);
    }

    // Print report
    eprintln!();
    eprintln!(
        "{:<20} {:<12} {:<40} {}",
        "Script", "Code", "File", "Result"
    );
    eprintln!("{}", "-".repeat(90));

    let mut pass_count = 0;
    let mut fail_count = 0;

    for r in &results {
        let status = if r.passed { "PASS" } else { "FAIL" };
        if r.passed {
            pass_count += 1;
        } else {
            fail_count += 1;
        }
        eprintln!(
            "{:<20} {:<12} {:<40} {}",
            r.script,
            r.code,
            r.file,
            status
        );
        if let Some(err) = &r.error {
            eprintln!("    Error: {}", err);
        }
    }

    eprintln!();
    eprintln!(
        "Summary: {pass_count} passed, {fail_count} failed out of {} total",
        results.len()
    );

    if fail_count > 0 {
        panic!(
            "{fail_count} of {} translations failed to parse",
            results.len()
        );
    }
}

fn process_translation(translation: &Translation) -> TestResult {
    let make_error = |file: &str, error: String| TestResult {
        script: translation.script.clone(),
        code: translation.code.clone(),
        file: file.to_string(),
        passed: false,
        error: Some(error),
    };

    // Download the zip
    let response = match ureq::get(&translation.url).call() {
        Ok(r) => r,
        Err(e) => return make_error("(download)", format!("Download failed: {e}")),
    };

    let bytes = match response.into_body().read_to_vec() {
        Ok(b) => b,
        Err(e) => return make_error("(download)", format!("Read body failed: {e}")),
    };

    // Open zip archive
    let cursor = Cursor::new(bytes);
    let mut archive = match ZipArchive::new(cursor) {
        Ok(a) => a,
        Err(e) => return make_error("(zip)", format!("Invalid zip: {e}")),
    };

    // Find first USFM file
    let usfm_index = (0..archive.len()).find(|&i| {
        if let Ok(file) = archive.by_index_raw(i) {
            let name = file.name().to_lowercase();
            name.ends_with(".usfm") || name.ends_with(".sfm")
        } else {
            false
        }
    });

    let usfm_index = match usfm_index {
        Some(i) => i,
        None => return make_error("(zip)", "No .usfm or .sfm file found in archive".into()),
    };

    let (file_name, content) = {
        let mut file = match archive.by_index(usfm_index) {
            Ok(f) => f,
            Err(e) => return make_error("(zip)", format!("Failed to read zip entry: {e}")),
        };

        let name = file.name().to_string();
        let mut raw = Vec::new();
        if let Err(e) = file.read_to_end(&mut raw) {
            return make_error(&name, format!("Failed to read file contents: {e}"));
        }

        let content = match String::from_utf8(raw.clone()) {
            Ok(s) => s,
            Err(_) => return make_error("(read)", format!("Not valid UTF-8.")),
        };

        (name, content)
    };

    match usfm::parse(&content) {
        Ok(_book) => TestResult {
            script: translation.script.clone(),
            code: translation.code.clone(),
            file: file_name,
            passed: true,
            error: None,
        },
        Err(msg) => make_error(&file_name, msg),
    }
}

