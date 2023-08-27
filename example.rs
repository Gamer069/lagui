use lagui::Window;
use lagui::WindowHandle;
use lagui::WindowSize;

fn main() {
    let mut window: Window = Window::new("hello world".to_string(), WindowSize::default());

    window.set_handle(WindowHandle::Resizeable, true);

    window.start();
}
