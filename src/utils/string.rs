pub trait StringExt {
    fn starts_with_ci(&self, lower_prefix: &str) -> bool;
}

impl StringExt for String {
    fn starts_with_ci(&self, lower_prefix: &str) -> bool {
        let mut self_iter = self.chars();
        let mut other_iter = lower_prefix.chars();
        loop {
            match (self_iter.next(), other_iter.next()) {
                (Some(a), Some(b)) if a.to_ascii_lowercase() == b => {}
                (_, None) => return true,
                _ => return false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StringExt;

    #[test]
    fn test_starts_with_ci() {
        let mut time = std::time::Instant::now();
        assert!("hello, World!".to_string().starts_with_ci("hello"));
        println!("{:?}", time.elapsed());
        assert!("HELLO, World!".to_string().starts_with_ci("hello"));
        println!("{:?}", time.elapsed());
        assert!("hElLo, World!".to_string().starts_with_ci("hello"));
        println!("{:?}", time.elapsed());
        assert!(!"Hello, World!".to_string().starts_with_ci("world"));
        println!("{:?}", time.elapsed());
        println!();

        time = std::time::Instant::now();
        assert!(!"hello, World!".to_string().starts_with_ci("world"));
        println!("{:?}", time.elapsed());
        assert!(!"HELLO, World!".to_string().starts_with_ci("world"));
        println!("{:?}", time.elapsed());
        assert!(!"hElLo, World!".to_string().starts_with_ci("world"));
        println!("{:?}", time.elapsed());
        assert!("Hello, World!".to_string().starts_with_ci("hello"));
        println!("{:?}", time.elapsed());
        println!();
    }
}
