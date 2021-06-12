use log::Level;
use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Serialize)]
pub(crate) struct VioletLog {
    severity: VioletLogSeverity,
    title: String,
    message: String,
    stacktrace: Option<String>,
}

#[derive(Debug, Clone, Serialize_repr)]
#[repr(u8)]
pub enum VioletLogSeverity {
    NoDefined = 0,
    Severe = 1,
    Error = 2,
    Warning = 3,
    Info = 4,
    Verbose = 5,
}

impl VioletLog {
    pub fn new(severity: VioletLogSeverity, title: String, message: String) -> Self {
        Self {
            severity,
            title,
            message,
            stacktrace: None,
        }
    }

    // Com a mudança para pub(crate) essa função não possui uso.
    // pub fn set_stacktrace(&mut self, stacktrace: String) {
    //     self.stacktrace = Some(stacktrace);
    // }
}

impl From<u8> for VioletLogSeverity {
    fn from(el: u8) -> Self {
        match el {
            1 => VioletLogSeverity::Severe,
            2 => VioletLogSeverity::Error,
            3 => VioletLogSeverity::Warning,
            4 => VioletLogSeverity::Info,
            5 => VioletLogSeverity::Verbose,
            _ => VioletLogSeverity::NoDefined,
        }
    }
}

impl From<&VioletLogSeverity> for u8 {
    fn from(val: &VioletLogSeverity) -> Self {
        match val {
            VioletLogSeverity::NoDefined => 0,
            VioletLogSeverity::Severe => 1,
            VioletLogSeverity::Error => 2,
            VioletLogSeverity::Warning => 3,
            VioletLogSeverity::Info => 4,
            VioletLogSeverity::Verbose => 5,
        }
    }
}

impl From<Level> for VioletLogSeverity {
    fn from(el: Level) -> Self {
        match el {
            Level::Error => VioletLogSeverity::Error,
            Level::Warn => VioletLogSeverity::Warning,
            Level::Info => VioletLogSeverity::Info,
            Level::Debug => VioletLogSeverity::Verbose,
            Level::Trace => VioletLogSeverity::NoDefined,
        }
    }
}

pub(crate) fn convert_level_to_u8(level: &Level) -> u8 {
    match level {
        Level::Error => 2,
        Level::Warn => 3,
        Level::Info => 4,
        Level::Debug => 5,
        Level::Trace => 0,
    }
}

pub(crate) fn convert_level_to_string(level: &Level) -> String {
    let matc = match level {
        Level::Error => "ERRO",
        Level::Warn => "WARN",
        Level::Info => "INFO",
        Level::Debug => "DEBU",
        Level::Trace => "TRAC",
    };

    matc.into()
}
