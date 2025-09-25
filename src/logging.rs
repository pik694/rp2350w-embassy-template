pub fn init_logger() {
    env_logger::builder().is_test(true).init();
}
