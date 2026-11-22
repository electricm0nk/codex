//! Signed update manifest: the app installs nothing from a manifest it cannot verify.
//!
//! CI signs the exact bytes of `update-manifest.json` with Ed25519 (`openssl pkeyutl -sign -rawin`,
//! a 64-byte raw signature) and publishes `update-manifest.json.sig` beside it, both on the release
//! and on the `update-index` mirror. The webview fetches and parses the manifest for display only;
//! its parsed copy cannot be byte-verified, so the install path fetches the mirror bytes and the
//! signature itself and verifies them here before any hash or URL in the manifest is read. The only
//! way to obtain a [`VerifiedManifest`] outside tests is [`fetch_verified_manifest`].

use base64::Engine;
use ed25519_dalek::{Signature, VerifyingKey};
use serde_json::Value;

/// Raw 32-byte Ed25519 public keys (base64): the active key, then the spare kept for rotation.
/// A manifest signed by either is accepted. Rotation runbook: `docs/release/update-signing-key-runbook.md`.
pub const TRUSTED_PUBLIC_KEYS_B64: [&str; 2] = [
    "VCG7pyELsjtD58KkWmmxmaKseuw290Fe2Ley6UDT+uY=",
    "X5vaJgOFUXoYvfTo60JoQfEhKkfrBxaDSQKou2FXCM4=",
];

/// Where the publish workflow mirrors each release's manifest: `<prefix><tag>/update-manifest.json`,
/// with the signature at the same URL plus `.sig`.
pub const MANIFEST_MIRROR_PREFIX: &str = "https://raw.githubusercontent.com/electricm0nk/codex/update-index/manifests/";

/// A manifest whose exact published bytes carried a valid signature from a trusted key.
#[derive(Debug, Clone, PartialEq)]
pub struct VerifiedManifest {
    value: Value,
}

impl VerifiedManifest {
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Tests of the installers downstream of verification build their input here.
    #[cfg(test)]
    pub(crate) fn assume_verified_for_test(value: Value) -> Self {
        VerifiedManifest { value }
    }
}

/// The embedded public keys, decoded.
pub fn trusted_keys() -> Result<Vec<VerifyingKey>, String> {
    TRUSTED_PUBLIC_KEYS_B64
        .iter()
        .map(|b64| {
            let raw: [u8; 32] = base64::engine::general_purpose::STANDARD
                .decode(b64)
                .ok()
                .and_then(|bytes| bytes.try_into().ok())
                .ok_or(format!("embedded update public key {b64} is not 32 base64 bytes"))?;
            VerifyingKey::from_bytes(&raw).map_err(|e| format!("embedded update public key {b64} is invalid: {e}"))
        })
        .collect()
}

fn text_at<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str> {
    path.iter().try_fold(value, |v, key| v.get(key))?.as_str().filter(|s| !s.is_empty())
}

/// Verify `signature` over the exact `bytes` with any of `keys`, then parse the signed bytes. The
/// fields that stop an old manifest replaying as a new one (`channel`, `version`, `tag`,
/// `promotion_lineage.promoted_at`) must be present in what was signed.
pub fn verify_manifest_bytes(bytes: &[u8], signature: &[u8], keys: &[VerifyingKey]) -> Result<VerifiedManifest, String> {
    let raw: [u8; 64] = signature.try_into().map_err(|_| {
        format!("refusing update: the manifest signature is {} bytes, an Ed25519 signature is 64 bytes", signature.len())
    })?;
    let signature = Signature::from_bytes(&raw);
    if !keys.iter().any(|key| key.verify_strict(bytes, &signature).is_ok()) {
        return Err("refusing update: the manifest signature does not verify with any trusted key (no trusted key signed these bytes)".to_string());
    }
    let value: Value =
        serde_json::from_slice(bytes).map_err(|e| format!("refusing update: the signed manifest is not JSON: {e}"))?;
    for path in [&["channel"][..], &["version"], &["tag"], &["promotion_lineage", "promoted_at"]] {
        if text_at(&value, path).is_none() {
            return Err(format!("refusing update: the signed manifest has no {}", path.join(".")));
        }
    }
    Ok(VerifiedManifest { value })
}

/// The mirror URL of the manifest published under `tag` (`<channel>/v<version>-<sha>`).
pub fn manifest_url_for_tag(tag: &str) -> Result<String, String> {
    let well_formed = match tag.split_once('/') {
        Some((channel, rest)) => {
            ["alpha", "beta", "stable"].contains(&channel)
                && !rest.is_empty()
                && !rest.contains("..")
                && rest.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        }
        None => false,
    };
    if !well_formed {
        return Err(format!("refusing update: release tag {tag:?} is not <channel>/v<version>-<sha>"));
    }
    Ok(format!("{MANIFEST_MIRROR_PREFIX}{tag}/update-manifest.json"))
}

/// Fetch the signed bytes of the release `requested` names (the webview's parsed copy, used only to
/// locate it), verify them, and confirm they describe the same release the user chose.
pub fn fetch_verified_manifest(
    requested: &Value,
    keys: &[VerifyingKey],
    fetch: &dyn Fn(&str) -> Result<Vec<u8>, String>,
) -> Result<VerifiedManifest, String> {
    let tag = text_at(requested, &["tag"]).ok_or("refusing update: the chosen release names no tag")?;
    let url = manifest_url_for_tag(tag)?;
    let bytes = fetch(&url).map_err(|e| format!("refusing update: cannot fetch the release manifest: {e}"))?;
    let sig_url = format!("{url}.sig");
    let signature = fetch(&sig_url).map_err(|e| format!("refusing update: the release has no signature ({e})"))?;
    let verified = verify_manifest_bytes(&bytes, &signature, keys)?;
    for field in ["channel", "version", "tag"] {
        let (signed, chosen) = (text_at(verified.value(), &[field]), text_at(requested, &[field]));
        if signed != chosen {
            return Err(format!(
                "refusing update: the signed manifest's {field} is {signed:?}, the release chosen has {chosen:?}; check for updates again"
            ));
        }
    }
    Ok(verified)
}

/// HTTPS GET of `url` into memory, through the installers' shared download edge.
pub fn fetch_url(url: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    super::download::download_to_writer(url, &mut bytes).map_err(|e| e.to_string())?;
    Ok(bytes)
}

#[cfg(test)]
pub(crate) mod test_support {
    use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
    use serde_json::{json, Value};

    /// A throwaway key from a fixed test seed. Never a real key.
    pub fn throwaway(seed: u8) -> SigningKey {
        SigningKey::from_bytes(&[seed; 32])
    }

    pub fn public(key: &SigningKey) -> VerifyingKey {
        key.verifying_key()
    }

    pub fn sign(key: &SigningKey, bytes: &[u8]) -> Vec<u8> {
        key.sign(bytes).to_bytes().to_vec()
    }

    pub const TAG: &str = "alpha/v0.22.159-0123abcd";

    /// A manifest carrying the replay-relevant fields plus `extra` top-level entries.
    pub fn manifest_bytes(extra: Value) -> Vec<u8> {
        let mut m = json!({
            "schema_version": "1.3.0",
            "channel": "alpha",
            "version": "0.22.159",
            "tag": TAG,
            "tranche_id": "STC-CODEX-SD-16",
            "source_commit": "a".repeat(40),
            "promotion_lineage": {
                "source_branch": "develop",
                "source_commit": "a".repeat(40),
                "promoted_at": "2026-11-21T00:00:00Z"
            }
        });
        if let Value::Object(extra) = extra {
            m.as_object_mut().unwrap().extend(extra);
        }
        // Pretty-printed with a trailing newline, like the published file: verification is over
        // these bytes, never a re-serialization.
        let mut out = serde_json::to_vec_pretty(&m).unwrap();
        out.push(b'\n');
        out
    }

    /// A fetcher serving `bytes` at the mirror URL for `TAG` and `sig` (if any) at `.sig`.
    pub fn serving(bytes: Vec<u8>, sig: Option<Vec<u8>>) -> impl Fn(&str) -> Result<Vec<u8>, String> {
        let url = super::manifest_url_for_tag(TAG).unwrap();
        move |asked: &str| {
            if asked == url {
                Ok(bytes.clone())
            } else if asked == format!("{url}.sig") {
                sig.clone().ok_or_else(|| format!("download of {asked} failed: status code 404"))
            } else {
                Err(format!("unexpected fetch of {asked}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;
    use serde_json::json;

    #[test]
    fn a_valid_signature_from_the_active_key_passes() {
        let key = throwaway(1);
        let bytes = manifest_bytes(json!({}));
        let verified = verify_manifest_bytes(&bytes, &sign(&key, &bytes), &[public(&key)]).expect("verifies");
        assert_eq!(verified.value()["version"], "0.22.159");
    }

    #[test]
    fn a_signature_from_the_second_spare_key_passes() {
        let (active, spare) = (throwaway(1), throwaway(2));
        let bytes = manifest_bytes(json!({}));
        let sig = sign(&spare, &bytes);
        assert!(verify_manifest_bytes(&bytes, &sig, &[public(&active), public(&spare)]).is_ok());
        assert!(verify_manifest_bytes(&bytes, &sig, &[public(&active)]).is_err(), "only because the spare is trusted");
    }

    #[test]
    fn one_flipped_manifest_byte_fails() {
        let key = throwaway(1);
        let bytes = manifest_bytes(json!({}));
        let sig = sign(&key, &bytes);
        for i in [0, bytes.len() / 2, bytes.len() - 1] {
            let mut tampered = bytes.clone();
            tampered[i] ^= 0x01;
            let err = verify_manifest_bytes(&tampered, &sig, &[public(&key)]).unwrap_err();
            assert!(err.contains("signature"), "byte {i}: {err}");
        }
    }

    #[test]
    fn a_signature_from_an_untrusted_key_fails() {
        let bytes = manifest_bytes(json!({}));
        let sig = sign(&throwaway(9), &bytes);
        let err = verify_manifest_bytes(&bytes, &sig, &[public(&throwaway(1)), public(&throwaway(2))]).unwrap_err();
        assert!(err.contains("no trusted key"), "{err}");
    }

    #[test]
    fn an_empty_or_truncated_signature_fails() {
        let key = throwaway(1);
        let bytes = manifest_bytes(json!({}));
        let sig = sign(&key, &bytes);
        for cut in [0, 1, 63] {
            let err = verify_manifest_bytes(&bytes, &sig[..cut], &[public(&key)]).unwrap_err();
            assert!(err.contains("64 bytes"), "cut {cut}: {err}");
        }
        let mut long = sig.clone();
        long.push(0);
        assert!(verify_manifest_bytes(&bytes, &long, &[public(&key)]).is_err(), "an over-long signature is not accepted either");
    }

    #[test]
    fn channel_version_and_publish_time_are_inside_the_signed_bytes() {
        let key = throwaway(1);
        let bytes = manifest_bytes(json!({}));
        let sig = sign(&key, &bytes);
        let text = String::from_utf8(bytes.clone()).unwrap();
        // An old alpha cannot be relabelled as a new stable, or re-dated, without breaking the signature.
        for (from, to) in [
            ("\"alpha\"", "\"stable\""),
            ("0.22.159", "0.22.999"),
            ("2026-11-21T00:00:00Z", "2027-11-21T00:00:00Z"),
        ] {
            assert!(text.contains(from), "{from} is in the signed bytes");
            let relabelled = text.replacen(from, to, 1).into_bytes();
            assert!(verify_manifest_bytes(&relabelled, &sig, &[public(&key)]).is_err(), "{from} -> {to}");
        }
    }

    #[test]
    fn a_signed_manifest_without_the_replay_fields_is_refused() {
        let key = throwaway(1);
        for field in ["channel", "version", "tag"] {
            let mut m: Value = serde_json::from_slice(&manifest_bytes(json!({}))).unwrap();
            m.as_object_mut().unwrap().remove(field);
            let bytes = serde_json::to_vec(&m).unwrap();
            let err = verify_manifest_bytes(&bytes, &sign(&key, &bytes), &[public(&key)]).unwrap_err();
            assert!(err.contains(field), "{field}: {err}");
        }
        let mut m: Value = serde_json::from_slice(&manifest_bytes(json!({}))).unwrap();
        m["promotion_lineage"].as_object_mut().unwrap().remove("promoted_at");
        let bytes = serde_json::to_vec(&m).unwrap();
        let err = verify_manifest_bytes(&bytes, &sign(&key, &bytes), &[public(&key)]).unwrap_err();
        assert!(err.contains("promoted_at"), "{err}");
    }

    /// Cross-implementation vector: a throwaway key generated with OpenSSL 3.0.13 and the exact CI
    /// command `openssl pkeyutl -sign -rawin -inkey k.pem -in m.json -out m.json.sig`. Proves the
    /// signature format CI emits is the one this crate verifies.
    #[test]
    fn a_signature_made_by_the_ci_openssl_command_verifies() {
        let b64 = |s: &str| base64::engine::general_purpose::STANDARD.decode(s).unwrap();
        let public: [u8; 32] = b64("gG7zA3DZAzJV9rrxEprArF+ZYwp23lNkAKR7cZ06fZo=").try_into().unwrap();
        let key = VerifyingKey::from_bytes(&public).unwrap();
        let sig = b64("xZ5/rpEWcYhIKHmdO89L5FGgPjRIdilo3pdNYLkV8ABH4cWuuP25p2XxLqvDL0Ah9N5P0tsrMen6ZiNDeQOVDg==");
        let message = b"{\"channel\":\"alpha\",\"version\":\"0.22.1\"}\n";
        // This message lacks `tag`, so the signature passes and the field check is what refuses.
        let err = verify_manifest_bytes(message, &sig, &[key]).unwrap_err();
        assert!(err.contains("tag") && !err.contains("signature"), "{err}");
        let mut flipped = message.to_vec();
        flipped[2] ^= 1;
        assert!(verify_manifest_bytes(&flipped, &sig, &[key]).unwrap_err().contains("signature"));
    }

    #[test]
    fn the_embedded_keys_are_the_two_published_public_keys() {
        let keys = trusted_keys().expect("both embedded keys decode to valid Ed25519 points");
        assert_eq!(keys.len(), 2);
        let b64 = |k: &VerifyingKey| base64::engine::general_purpose::STANDARD.encode(k.as_bytes());
        assert_eq!(b64(&keys[0]), "VCG7pyELsjtD58KkWmmxmaKseuw290Fe2Ley6UDT+uY=", "active");
        assert_eq!(b64(&keys[1]), "X5vaJgOFUXoYvfTo60JoQfEhKkfrBxaDSQKou2FXCM4=", "spare");
        // No test key is trusted by the shipped list.
        let bytes = manifest_bytes(json!({}));
        assert!(verify_manifest_bytes(&bytes, &sign(&throwaway(1), &bytes), &keys).is_err());
    }

    #[test]
    fn the_mirror_url_is_built_only_from_a_well_formed_tag() {
        assert_eq!(
            manifest_url_for_tag("alpha/v0.22.158-6016ea83").unwrap(),
            "https://raw.githubusercontent.com/electricm0nk/codex/update-index/manifests/alpha/v0.22.158-6016ea83/update-manifest.json"
        );
        assert!(manifest_url_for_tag("beta/v0.22.1-abcdef12").is_ok());
        for bad in ["", "alpha", "nightly/v1-abc", "alpha/", "alpha/../../main/x", "alpha/..", "alpha/v1/x", "alpha/v1?x=1", "alpha/v1#x", "alpha/v 1"] {
            assert!(manifest_url_for_tag(bad).is_err(), "{bad:?} must be refused");
        }
    }

    #[test]
    fn the_fetched_signed_bytes_are_the_bytes_verified() {
        let key = throwaway(1);
        let bytes = manifest_bytes(json!({ "linux_appimage": { "sha256": "b".repeat(64) } }));
        let requested: Value = serde_json::from_slice(&bytes).unwrap();
        let fetch = serving(bytes.clone(), Some(sign(&key, &bytes)));
        let verified = fetch_verified_manifest(&requested, &[public(&key)], &fetch).expect("verifies");
        assert_eq!(verified.value(), &requested);
    }

    #[test]
    fn a_missing_signature_file_is_refused() {
        let key = throwaway(1);
        let bytes = manifest_bytes(json!({}));
        let requested: Value = serde_json::from_slice(&bytes).unwrap();
        let err = fetch_verified_manifest(&requested, &[public(&key)], &serving(bytes, None)).unwrap_err();
        assert!(err.contains("no signature"), "{err}");
    }

    #[test]
    fn a_mirror_that_serves_tampered_bytes_is_refused() {
        let key = throwaway(1);
        let bytes = manifest_bytes(json!({}));
        let sig = sign(&key, &bytes);
        let requested: Value = serde_json::from_slice(&bytes).unwrap();
        let mut tampered = bytes.clone();
        let at = tampered.len() - 3;
        tampered[at] ^= 0x01;
        let err = fetch_verified_manifest(&requested, &[public(&key)], &serving(tampered, Some(sig))).unwrap_err();
        assert!(err.contains("signature"), "{err}");
    }

    #[test]
    fn a_signed_manifest_for_a_different_release_than_the_one_chosen_is_refused() {
        let key = throwaway(1);
        let bytes = manifest_bytes(json!({}));
        let fetch = serving(bytes.clone(), Some(sign(&key, &bytes)));
        let mut requested: Value = serde_json::from_slice(&bytes).unwrap();
        requested["version"] = json!("0.22.160");
        let err = fetch_verified_manifest(&requested, &[public(&key)], &fetch).unwrap_err();
        assert!(err.contains("version"), "{err}");
    }

    #[test]
    fn a_request_without_a_usable_tag_fetches_nothing() {
        let fetch = |url: &str| -> Result<Vec<u8>, String> { panic!("must not fetch {url}") };
        let err = fetch_verified_manifest(&json!({ "version": "1" }), &[], &fetch).unwrap_err();
        assert!(err.contains("tag"), "{err}");
        let err = fetch_verified_manifest(&json!({ "tag": "alpha/../x" }), &[], &fetch).unwrap_err();
        assert!(err.contains("tag"), "{err}");
    }
}
