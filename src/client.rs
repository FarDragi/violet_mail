use std::{thread, time::Duration};

use isahc::{config::Configurable, Request, RequestExt};

use log::{Metadata, Record};

use crate::prelude::{VioletLog, VioletLogSeverity};
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GResult<T> = Result<T, GenericError>;

#[derive(Clone)]
struct HttpVioletData {
    indetifier: u64,
    token: String,
}

impl HttpVioletData {
    fn new(indetifier: u64, token: String) -> Self {
        Self { indetifier, token }
    }

    async fn send_data(
        &self,
        title: String,
        severity: VioletLogSeverity,
        message: String,
    ) -> GResult<()> {
        let log_vio = VioletLog::new(severity, title, message);
        let log_vio_json = serde_json::to_string(&log_vio)?;

        let retorno = Request::post(format!(
            "https://violet.zuraaa.com/api/apps/{}/events",
            self.indetifier
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", &self.token)
        .timeout(Duration::from_secs(20))
        .body(log_vio_json)?
        .send_async()
        .await?;
        let body = retorno.status();
        println!("{:?}", body);
        Ok(())
    }
}

pub fn init(indentifier: u64, token: String) {
    static mut HAS_INIT: bool = false;

    unsafe {
        if HAS_INIT {
            return;
        }
    }

    let leak_content: &'static mut HttpVioletData =
        Box::leak(Box::new(HttpVioletData::new(indentifier, token)));

    log::set_logger(leak_content).unwrap();

    unsafe {
        HAS_INIT = true;
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
        let cloned_self = self.clone();
        let pointer_data = (record.level(), record.args().to_string());

        thread::spawn(move || {
            futures::executor::block_on(async {
                cloned_self
                    .send_data("vulcan gay".into(), pointer_data.0.into(), pointer_data.1)
                    .await
                    .unwrap();
            });
        });
    }
}
