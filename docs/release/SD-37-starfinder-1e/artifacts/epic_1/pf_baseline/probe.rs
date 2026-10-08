
#[cfg(test)]
mod e1_pf_render_probe {
    use super::*;
    /// Temporary pre/post render probe (SD-37 E1.1-E1.3 baseline; not committed).
    #[test]
    fn e1_pf_render_probe_dump() {
        let out = PathBuf::from(std::env::var("E1_PROBE_OUT").expect("E1_PROBE_OUT"));
        std::fs::create_dir_all(&out).unwrap();
        let app = std::env::temp_dir().join(format!("codex-e1-probe-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&app);
        std::fs::create_dir_all(&app).unwrap();
        seed_default_characters_at(&app, "0.0.0-test").expect("seeding succeeds");
        for (name, id) in [("aldric", DEFAULT_CHARACTER_ID), ("elowen", SECOND_SEED_CHARACTER_ID)] {
            let root = characters_root_from_app_data_dir(&app).join(id);
            let loaded = load_saved_character_at_root(&root).expect("load");
            let json = serde_json::to_string_pretty(&loaded).unwrap();
            std::fs::write(out.join(format!("{name}.json")), json).unwrap();
        }
        std::fs::remove_dir_all(&app).ok();
    }
}
