mod core;

extern crate android_logger;
extern crate log;

use android_logger::Config;

#[ctor::ctor]
#[cfg_attr(target_os = "android", link(name = "c++_shared"))]
pub fn init() {
    std::thread::spawn(|| {
        setup();
        main();
    });
}

fn setup() {
    android_logger::init_once(Config::default().with_max_level(log::LevelFilter::Trace));
}

fn main() {
    log::debug!("{} Hi from ARM!", core::constants::LOG_TAG);

    let symbol_name = std::ffi::CString::new("printf").unwrap();

    unsafe {
        let symbol = libc::dlsym(libc::RTLD_DEFAULT, symbol_name.as_ptr());
        if symbol.is_null() {
            log::debug!(
                "{} dlsym failed to find symbol 'printf'",
                core::constants::LOG_TAG
            );
        } else {
            log::info!(
                "{} dlsym found symbol 'printf' at address {:?}",
                core::constants::LOG_TAG,
                symbol
            );
        }
    }
}
