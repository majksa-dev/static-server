use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate(modified: &SystemTime, len: u64) -> String {
    format!(
        "{:x}-{:x}",
        modified.duration_since(UNIX_EPOCH).unwrap().as_secs(),
        len,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_create_etag() {
        let updated_at = SystemTime::UNIX_EPOCH
            .checked_add(std::time::Duration::new(1_000_000_000, 0))
            .unwrap();
        let etag = generate(&updated_at, 5);
        assert_eq!(etag, "3b9aca00-5");
    }
}
