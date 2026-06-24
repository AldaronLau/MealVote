mod logger;

use async_main::{LocalSpawner, async_main};

use self::logger::Logger;

fn init() {
    web_sys::console::info_1(&"MealVote 0.0.1".to_string().into());
    std::panic::set_hook(Box::new(web_panic_hook::hook));
    log::set_logger(&Logger).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}

#[async_main]
async fn main(_spawner: LocalSpawner) {
    init();
    log::info!("Hello, world!");
}
