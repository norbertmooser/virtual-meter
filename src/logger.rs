use env_logger::Builder;
use chrono::Local;
use std::io::Write;

pub fn init_logger() {
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info"); 

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} {} {}",
                Local::now().format("%y%m%d::%H%M%S"),
                record.level(),
                record.args()
            )
        })
        .parse_env(env)
        .init();
}
