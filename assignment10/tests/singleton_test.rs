#[path = "../creational_patterns/singleton/singleton.rs"]
mod singleton;

use singleton::INSTANCE;

#[test]
fn test_singleton_instance_exists() {
    assert_eq!(
        INSTANCE.connection_string,
        "LibraryDBConnection"
    );
}

#[test]
fn test_singleton_same_instance() {
    let first = &INSTANCE.connection_string;
    let second = &INSTANCE.connection_string;

    assert_eq!(first, second);
}
