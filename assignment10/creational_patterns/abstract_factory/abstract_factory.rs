pub trait Button {
    fn render(&self);
}

pub struct WindowsButton;

impl Button for WindowsButton {
    fn render(&self) {
        println!("Rendering Windows Button");
    }
}

pub struct MacOSButton;

impl Button for MacOSButton {
    fn render(&self) {
        println!("Rendering MacOS Button");
    }
}

pub trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
}

pub struct WindowsFactory;

impl GUIFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}

pub struct MacOSFactory;

impl GUIFactory for MacOSFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacOSButton)
    }
}
