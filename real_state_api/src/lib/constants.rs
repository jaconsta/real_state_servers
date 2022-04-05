// https://doc.rust-lang.org/book/title-page.html
// https://doc.rust-lang.org/rust-by-example/error.html
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
// https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/constants.html

// https://highassurance.rs/

// Default values constants
pub static SERVER_URL: &'static str = "127.0.0.1:8080";
pub const DB_URL: &'static str = "mongodb://game_api:game_api@192.168.0.23:27017/real_state_game?w=majority";
pub const DB_NAME: &'static str = "real_state_game";
pub const HASH_SALT: &'static str = "EBjcJltLztMfObY7fnNpJ2SkkamUdW";
