pub mod bus;
pub mod commands;
pub mod db;
pub mod engine;
pub mod modules;
pub mod simverse;

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

    #[test]
    fn generated_ipc_types_package_exists() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../packages/ipc-types/src/index.ts");
        let contents = std::fs::read_to_string(path).expect("ipc types should be generated");

        assert!(contents.contains("export type ModuleSummary"));
        assert!(contents.contains("export type SimStatus"));
        assert!(contents.contains("export type LogLine"));
    }
}
