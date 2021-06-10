use log::Record;
use log4rs;
use log4rs::filter::{Filter, Response};
use log4rs::config;
use serde::Deserialize;

pub struct Logger<'a> {
    name: &'a str,
}

impl<'a> Logger<'a> {
    pub fn init_config() {
        let mut custom_filter_deserializer = log4rs::config::Deserializers::new();
        custom_filter_deserializer.insert(
            "request_filter",
            CustomFilterDeserializer
        );
        log4rs::init_file("config/log4rs.yaml", custom_filter_deserializer).unwrap_or_else(|e| {
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

#[derive(Deserialize)]
pub struct RequestFilterConfig {
    request_field: String
}

#[derive(Debug)]
pub struct RequestFilter {
    request_field: String
}

impl  RequestFilter {
    /// Creates a new `CustomFilter` with the specified interesting field.
    pub fn new(request_field: String) -> Self {
        Self { request_field }
    }
}


impl Filter for RequestFilter {
    fn filter(&self, record: &Record) -> Response {
        println!("RequestFilter {:?}", record);
        let path = record.module_path().unwrap();
        println!("RequestFilter {} - {}", path, &self.request_field);
        if path.contains(&self.request_field)  {
            Response::Accept
        } else {
            Response::Reject
        }
    }
}

pub struct CustomFilterDeserializer;

impl config::Deserialize for CustomFilterDeserializer {
    type Trait = dyn Filter;
    type Config = RequestFilterConfig;

    fn deserialize(&self, config: RequestFilterConfig, _: &config::Deserializers) ->  anyhow::Result<Box<dyn Filter>> {
        let filter = RequestFilter::new(config.request_field);
        println!("CustomFilterDeserializer {:?}", &filter);
        Ok(Box::new(filter))
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
