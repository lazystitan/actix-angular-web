use log::{info, trace, warn};
use log4rs;

pub struct Logger<'a> {
    name: &'a str,
}

impl<'a> Logger<'a> {
    pub fn init_config() {
        log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap_or_else(|e| {
            panic!("init logger file failed: {:?}", e);
        });
    }
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
    pub fn trace(&self, from: &str, message: &str) {
        trace!("{} from {} with {}", self.name, from, message)
    }

    pub fn info(&self, from: &str, message: &str) {
        info!("{} from {} with {}", self.name, from, message)
    }

    pub fn warn(&self, from: &str, message: &str) {
        warn!("{} from {} with {}", self.name, from, message)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn logger_test() {
        trace!("This is trace");
        info!("This is info");
        warn!("This is warn");
    }

    #[test]
    fn logger_log() {
        Logger::init_config();
        let user_logger = Logger::new("user");
        user_logger.trace("user", &format!("{} log in", "riton"));
        user_logger.info("user", "cannot find user");
        user_logger.warn("user", &format!("cannot find user {} password", "riton"));
    }
}
