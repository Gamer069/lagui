use std::cmp::Ordering;
use std::collections::BTreeMap;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowSize {
    width: i32,
    height: i32,
}

impl Default for WindowSize {
    fn default() -> Self {
        WindowSize {
            width: 900,
            height: 900,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum WindowHandle {
    Resizeable,
}

impl Ord for WindowHandle {
    fn cmp(&self, other: &Self) -> Ordering {
        // Define your custom ordering logic here
        // For demonstration purposes, let's consider Resizeable as less than other variants
        match (self, other) {
            (WindowHandle::Resizeable, WindowHandle::Resizeable) => Ordering::Equal,
            (WindowHandle::Resizeable, _) => Ordering::Less,
            (_, WindowHandle::Resizeable) => Ordering::Greater,
        }
    }
}

impl PartialOrd for WindowHandle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Window {
    title: String,
    size: WindowSize,
    handles: BTreeMap<WindowHandle, bool>,
}

impl Window {
    pub fn new(title: String, size: WindowSize) -> Window {
        println!("{}", std::thread::current().name().unwrap());
        Self {
            title,
            size,
            handles: BTreeMap::new(),
        }
    }

    pub fn set_handle(&mut self, handle: WindowHandle, value: bool) {
        self.handles.insert(handle, value);
    }

    pub fn start(&mut self) {
        let event_loop = EventLoop::new().expect("failed to unwrap the event loop");
        let mut builder: WindowBuilder = WindowBuilder::new();
        let mut resizeable: bool = true;

        for entry in &self.handles {
            if *entry.1 {
                if entry.0.cmp(&WindowHandle::Resizeable) == Ordering::Equal {
                    resizeable = true;
                }
            }
        }

        builder = builder.with_title(self.title.clone());
        builder = builder.with_resizable(resizeable);
        builder = builder.with_inner_size(LogicalSize::new(self.size.width, self.size.height));
        builder = builder.with_active(true);
        builder = builder.with_visible(true);

        builder.build(&event_loop).expect("Failed to create window");

        let mut done: bool = false;

        event_loop
            .run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        done = true;
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            })
            .expect("failed to execute event loop");
    }
}
