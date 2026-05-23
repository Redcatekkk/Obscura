pub fn app_name() -> &'static str {
    "obscura.deck"
}

#[cfg(test)]
mod tests {
    use super::app_name;

    #[test]
    fn app_name_matches_product() {
        assert_eq!(app_name(), "obscura.deck");
    }
}
