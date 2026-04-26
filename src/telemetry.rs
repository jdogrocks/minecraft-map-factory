/// Installs a panic hook that logs panics to stderr (no remote reporting).
pub fn install_panic_hook() {
    use log::error;
    std::panic::set_hook(Box::new(|panic_info| {
        error!("Application panicked: {:?}", panic_info);
    }));
}
