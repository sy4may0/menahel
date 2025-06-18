pub mod constants;
pub mod enums;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod repository;

pub fn init_logger() {
    simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        std::fs::File::create("log.txt").unwrap(),
    )])
    .unwrap();
}
