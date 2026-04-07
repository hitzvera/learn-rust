//! # Rust Modules & Packages Challenges
//!
//! A comprehensive set of exercises to master Rust's module system.
//!
//! ## 📚 Learning Path
//!
//! 1. **Level 1**: Packages & Crates Basics
//! 2. **Level 2**: Modules & Privacy
//! 3. **Level 3**: Paths & Use Keyword
//! 4. **Level 4**: File Separation & Re-exporting
//!
//! ## 🚀 Getting Started
//!
//! ```bash
//! # Run the challenge binary
//! cargo run
//!
//! # Run tests
//! cargo test
//! ```
//!
//! ## 📖 Based On
//!
//! [Rust Book Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level1_answers() {
        assert_eq!(level1::CRATE_TYPE_ANSWER, "B", "Crate type should be Binary");
        assert_eq!(level1::LIB_CRATE_ROOT_ANSWER, "B", "Lib root should be src/lib.rs");
        assert_eq!(level1::PACKAGE_CONTENTS_ANSWER, "B", "Package can have multiple binaries + 1 lib");
    }

    #[test]
    fn test_level2_privacy() {
        assert_eq!(level2::PUB_MODULE_CHILDREN, "B", "Children are NOT automatically public");
        assert_eq!(level2::CHILD_ACCESS_PARENT, "A", "Children CAN access parent items");
        assert_eq!(level2::SIBLING_ACCESS, "B", "Siblings CANNOT access each other's private items");
    }

    #[test]
    fn test_level3_paths() {
        assert_eq!(level3::SELF_REFERS_TO, "B", "self = current module");
        assert_eq!(level3::SUPER_REFERS_TO, "A", "super = parent module");
        assert_eq!(level3::CRATE_REFERS_TO, "C", "crate = crate root");
    }

    #[test]
    fn test_level4_files() {
        assert_eq!(level4::MODULE_FILE_LOCATION, "C", "Module can be .rs or /mod.rs");
        assert_eq!(level4::SUBMODULE_FILE_LOCATION, "B", "Submodule in parent dir");
    }
}
