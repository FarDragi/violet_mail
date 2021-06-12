use std::{thread, time::Duration};

use chrono::Utc;
use isahc::{config::Configurable, Request, RequestExt};

use log::{Metadata, Record};

use crate::{VioletLog, VioletLogSeverity};
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GResult<T> = Result<T, GenericError>;

#[derive(Clone)]
struct HttpVioletData {
    config: VioletBuilder,
}

#[derive(Debug, Clone)]
pub struct VioletBuilder {
    indentifier: u64,
    token: String,
    send_err_async: bool,
    default_title: String,
    send_level: VioletLogSeverity,
}

impl VioletBuilder {
    pub fn new(token: impl AsRef<str>, indentifier: u64) -> Self {
        Self {
            token: token.as_ref().to_string(),
            indentifier,
            default_title: env!("CARGO_PKG_NAME").into(),
            send_err_async: false,
            send_level: VioletLogSeverity::Error,
        }
    }

    pub fn enable_async(mut self) -> Self {
        self.send_err_async = true;
        self
    }

    pub fn set_title(mut self, title: impl AsRef<str>) -> Self {
        self.default_title = title.as_ref().to_string();
        self
    }

    pub fn set_send_min_level(mut self, level: VioletLogSeverity) -> Self {
        self.send_level = level;
        self
    }

    pub fn init(self) {
        static mut HAS_INIT: bool = false;

        unsafe {
            if HAS_INIT {
                return;
            }
        }

        let leak_content: &'static mut HttpVioletData =
            Box::leak(Box::new(HttpVioletData::new(self)));

        log::set_logger(leak_content).unwrap();

        unsafe {
            HAS_INIT = true;
        }
    }
}

impl HttpVioletData {
    fn new(config: VioletBuilder) -> Self {
        Self { config }
    }

    async fn send_data(
        &self,
        title: String,
        severity: VioletLogSeverity,
        message: String,
    ) -> GResult<()> {
        let log_vio = VioletLog::new(severity, title, message);
        let log_vio_json = serde_json::to_string(&log_vio)?;

        Request::post(format!(
            "https://violet.zuraaa.com/api/apps/{}/events",
            self.config.indentifier
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", &self.config.token)
        .timeout(Duration::from_secs(20))
        .body(log_vio_json)?
        .send_async()
        .await?;
        Ok(())
    }
}

impl log::Log for HttpVioletData {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn flush(&self) {
        todo!()
    }

    fn log(&self, record: &Record) {
        let pointer_data = (record.level(), record.args().to_string());

        {
            let level = crate::convert_level_to_string(&pointer_data.0);
            let data = Utc::now();
            let data_formated = data.format("%d/%m/%Y %H:%M:%S").to_string();
            println!("[({}) ({})]: {}", data_formated, level, &pointer_data.1)
        }

        {
            let level_u8 = u8::from(&self.config.send_level);
            let level_event_u8 = crate::convert_level_to_u8(&pointer_data.0);
            if level_event_u8 > level_u8 {
                return;
            }
        }

        if self.config.send_err_async {
            let cloned_self = self.clone();
            thread::spawn(move || {
                futures::executor::block_on(async {
                    cloned_self
                        .send_data(
                            cloned_self.config.default_title.clone(),
                            pointer_data.0.into(),
                            pointer_data.1,
                        )
                        .await
                        .ok();
                });
            });
        } else {
            futures::executor::block_on(async {
                self.send_data(
                    self.config.default_title.clone(),
                    pointer_data.0.into(),
                    pointer_data.1,
                )
                .await
                .ok();
            })
        }
    }
}
