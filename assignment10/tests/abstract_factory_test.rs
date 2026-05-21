#[path = "../creational_patterns/abstract_factory/abstract_factory.rs"]
mod abstract_factory;

use abstract_factory::{
    GUIFactory,
    WindowsFactory,
    MacOSFactory,
};

#[test]
fn test_windows_button_creation() {
    let factory = WindowsFactory;
    let _button = factory.create_button();
}

#[test]
fn test_macos_button_creation() {
    let factory = MacOSFactory;
    let _button = factory.create_button();
}
