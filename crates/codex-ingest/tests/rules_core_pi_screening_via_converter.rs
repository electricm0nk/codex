// -- split from `tests` in src/rules_core/pi_screening.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::pi_screening::*;
    use codex::rules_core::shape_b_v1::{License, REDACTED_PI_MARKER};

    #[test]
    fn reconcile_is_a_no_op_when_the_stamp_already_lists_description_among_others() {
        let fixed = reconcile_description_pi_stamp(
            Some(REDACTED_PI_MARKER),
            License::PiRedacted,
            Some(&format!("description,name,{}", codex_ingest::pcgen_import::ingest_payload::INGEST_TOKENS_FIELD)),
        );
        assert_eq!(fixed, None);
    }


}
