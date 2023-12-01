fn main() {
    heartless::Game::with_env_args( std::env::args_os() ).main_loop();
}
