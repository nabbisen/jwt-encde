mod util;
mod window;

pub fn start() -> std::result::Result<(), iced::Error> {
    iced::application(
        window::Window::new,
        window::Window::update,
        window::Window::view,
    )
    .run()
}
