use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use colored::*;

use chrono::Utc;
use isahc::{config::Configurable, Request, RequestExt};

use log::{set_logger, Metadata, Record};

use crate::{VioletLog, VioletLogSeverity};
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GResult<T> = Result<T, GenericError>;

lazy_static::lazy_static! {
    static ref CLIENT: HttpVioletData = HttpVioletData::new();
}

#[derive(Clone)]
#[non_exhaustive]
pub struct HttpVioletData {
    config: Arc<RwLock<Option<VioletBuilder>>>,
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

    pub fn init(self) -> GResult<()> {
        if CLIENT
            .config
            .read()
            .map_err(|err| format!("Poisoded mutex here: {:?}", &err))?
            .is_some()
        {
            return Ok(());
        }

        CLIENT.set_config(self)?;
        set_logger(&*CLIENT).unwrap();

        Ok(())
    }
}

impl HttpVioletData {
    fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(None)),
        }
    }

    pub fn get_http() -> &'static Self {
        &CLIENT
    }

    fn set_config(&self, config: VioletBuilder) -> GResult<()> {
        *self
            .config
            .write()
            .map_err(|err| format!("Mutex is poisoned: {:?}", err))? = Some(config);
        Ok(())
    }

    pub async fn send_data(
        &self,
        title: String,
        severity: VioletLogSeverity,
        message: String,
    ) -> GResult<()> {
        let log_vio = VioletLog::new(severity, title, message);
        let log_vio_json = serde_json::to_string(&log_vio)?;
        let config = self
            .config
            .clone()
            .read()
            .map_err(|err| format!("Poisoned mutex: {:?}", err))?
            .as_ref()
            .ok_or("Violet não foi inicializada")?
            .clone();
        println!("{:?}", &log_vio_json);
        Request::post(format!(
            "https://violet.zuraaa.com/api/apps/{}/events",
            config.indentifier
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", config.token)
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
        if self.config.read().unwrap().is_none() {
            panic!("Violet não foi inicializado");
        }

        let config = self
            .config
            .read()
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone();

        let mut pointer_data = (record.level(), record.args().to_string());

        {
            let level = crate::convert_level_to_string(&pointer_data.0);
            let data = Utc::now();
            let data_formated = data.format("%d/%m/%Y %H:%M:%S").to_string();
            println!("[({}) ({})]: {}", data_formated, level, &pointer_data.1)
        }

        {
            let level_u8 = u8::from(config.send_level.clone());
            let level_event_u8 = crate::convert_level_to_u8(&pointer_data.0);
            if level_event_u8 > level_u8 {
                return;
            }
        }

        pointer_data.1 = pointer_data.1.normal().clear().to_string();

        if config.send_err_async {
            let cloned_self = self.clone();
            thread::spawn(move || {
                futures::executor::block_on(async {
                    cloned_self
                        .send_data(config.default_title, pointer_data.0.into(), pointer_data.1)
                        .await
                        .ok();
                });
            });
        } else {
            futures::executor::block_on(async {
                self.send_data(config.default_title, pointer_data.0.into(), pointer_data.1)
                    .await
                    .ok();
            })
        }
    }
}
