mod jwt;
mod window;

pub fn start() -> std::result::Result<(), iced::Error> {
    iced::application(
        window::Window::default,
        window::Window::update,
        window::Window::view,
    )
    .run()
}
