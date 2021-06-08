use std::{error::Error, thread::{self, spawn}};

use isahc::{AsyncBody, HttpClient, Response};
use log::{Level, Log, Metadata, Record, info, set_logger, set_max_level};
use serde_json::to_string;

use crate::log::{VioletLog, VioletLogSeverity};

pub struct VioletMail {
    base_url: String,
    client: HttpClient
}

static mut VIOLET_INITIALIZED: bool = false;

pub type VioletError = Box<dyn Error + Send + Sync + 'static>;

impl VioletMail {
    pub fn init(identifier: u32, token: &str) -> Result<&'static Self, VioletError> {
        unsafe {
            if !VIOLET_INITIALIZED {
                let client = HttpClient::builder()
                    .default_header("Authorization", token)
                    .default_header("Content-Type", "application/json")
                    .build()?;
        
                let base_url = format!("https://violet.zuraaa.com/api/apps/{}/events", identifier);
        
                let violet_mail = Box::new(Self {
                    client,
                    base_url
                });
        
                let violet_leak: &'static VioletMail = Box::leak(violet_mail); 
        
                set_logger(violet_leak).map_err(|err|
                    format!("{:?}", err)
                )?;
                set_max_level(log::LevelFilter::Trace);
        
                Ok(violet_leak)
            } else {
                Err("Violet mail ja iniciado".into())
            }
        }
    }

    pub fn send_log(&self, violet_log: &VioletLog) {
        let body = to_string(violet_log);

        let client = &self.client;
        let base_url = &self.base_url;

        let handle = spawn(move || {
            let future = async {
                client.post_async(base_url, body.unwrap()).await
            };

            futures::executor::block_on(future)
        });

    }
}

impl Log for VioletMail {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("{} - {}", record.level(), record.args());

        let violet_log = VioletLog::new(
            VioletLogSeverity::from(record.level()), 
            "Violet mail".to_string(), 
            record.args().to_string()
        );

        self.send_log(&violet_log);
    }

    fn flush(&self) {
        todo!()
    }
}

// pub trait SendToVioletMail {
//     fn send_to_violet();
// }

// impl SendToVioletMail for dyn Error {
//     fn send_to_violet() {
//         todo!()
//     }
// }
