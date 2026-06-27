// sortcrab — property-based tests

#![allow(unused_imports)] // imports are used via proptest! macro expansion

use std::collections::HashSet;
use std::time::SystemTime;

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveTime, TimeZone};
use proptest::prelude::*;

use sortcrab::config::rules::RulesConfig;
use sortcrab::core::classify::classify_extension;
use sortcrab::core::mover::{Classification, MoveOptions, move_file};
use sortcrab::core::semester::semester_from_time;

// ── Date strategy: covers general dates + semester boundaries ────────────────

fn date_strategy() -> BoxedStrategy<NaiveDate> {
    // General: all months, days 1–28 (safe for every month)
    let general = (1970i32..2100, 1u32..13, 1u32..29u32)
        .prop_map(|(y, m, d)| NaiveDate::from_ymd_opt(y, m, d).unwrap())
        .boxed();

    // June boundary: days 28–30
    let june = (1970i32..2100, 6u32..7, 28u32..31u32)
        .prop_map(|(y, m, d)| NaiveDate::from_ymd_opt(y, m, d).unwrap())
        .boxed();

    // July boundary: days 1–3
    let july = (1970i32..2100, 7u32..8, 1u32..4u32)
        .prop_map(|(y, m, d)| NaiveDate::from_ymd_opt(y, m, d).unwrap())
        .boxed();

    prop_oneof![general, june, july].boxed()
}

// ── Extension strategy: case / dot / whitespace variants of known extensions ─

static EXT_VARIANTS: &[&str] = &[
    "pdf", "PDF", "Pdf", "pDF", "pDf", "PdF", "mp3", "MP3", "Mp3", "mP3", "rs", "RS", "Rs", "rS",
    "jpg", "JPG", "Jpg", "jPG", "jPg", "jpG", "txt", "TXT", "Txt", "tXT", ".pdf", ".PDF", ".Pdf",
    " pdf ", " PDF ", " pdf", ". Mp3", " .mp3", "  txt  ", ".Rs", " .Rs ",
];

fn ext_case_strategy() -> impl Strategy<Value = String> {
    prop::sample::subsequence(EXT_VARIANTS.to_vec(), 1..2).prop_map(|v| v[0].to_string())
}

// ── Collision resolution ─────────────────────────────────────────────────────
//
// Generate arbitrary filename stems and verify the collision resolver
// never returns an existing path.

proptest! {
    #[test]
    fn proptest_collision_resolution(
        stem in prop::collection::vec(proptest::char::range('a', 'z'), 1..8)
            .prop_map(|v| v.into_iter().collect::<String>()),
        ext in prop_oneof![
            Just("pdf"), Just("mp3"), Just("rs"), Just("jpg"), Just("txt")
        ],
    ) {
        let src = tempfile::tempdir().unwrap();
        let tgt = tempfile::tempdir().unwrap();

        let filename = format!("{stem}.{ext}");
        let class = Classification {
            category: "Documents".into(),
            subcategory: "PDF".into(),
        };
        let semester = "2025-I";
        let mut dests: HashSet<std::path::PathBuf> = HashSet::new();

        for i in 0..5 {
            let src_file = src.path().join(format!("src-{i}-{filename}"));
            std::fs::write(&src_file, format!("content-{i}")).unwrap();

            let opts = MoveOptions {
                source: &src_file,
                target: tgt.path(),
                classification: &class,
                semester,
                filename: &filename,
            };

            let dest = move_file(&opts).unwrap();
            prop_assert!(
                dests.insert(dest),
                "duplicate destination for iteration {i} with stem {stem:?} ext {ext:?}",
            );
        }
    }
}

// ── Semester boundaries ──────────────────────────────────────────────────────
//
// Test that dates at month boundaries produce correct semester string.
// January–June → "{year}-I", July–December → "{year}-II".

proptest! {
    #[test]
    fn proptest_semester_boundaries(date in date_strategy()) {
        let naive_dt = date.and_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap());
        let local_dt: DateTime<Local> = Local
            .from_local_datetime(&naive_dt)
            .earliest()
            .unwrap();
        let time = SystemTime::from(local_dt);

        let sem = semester_from_time(&time);
        let year = date.year();
        let month = date.month();

        let expected = if month <= 6 {
            format!("{year}-I")
        } else {
            format!("{year}-II")
        };

        prop_assert_eq!(&sem, &expected, "failed for date {}", date);
    }
}

// ── Extension normalisation ─────────────────────────────────────────────────
//
// Test that classify_extension handles any casing / whitespace / dots for
// known extensions and maps them to the correct rule.

proptest! {
    #[test]
    fn proptest_extension_normalization(ext in ext_case_strategy()) {
        let rules = RulesConfig::default();
        let result = classify_extension(&rules, &ext);
        prop_assert!(
            result.is_some(),
            "extension {ext:?} should be recognised (case/dot/whitespace variant)",
        );
    }
}
