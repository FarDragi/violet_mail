use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Serialize)]
pub struct VioletLog {
    severity: VioletLogSeverity,
    title: String,
    message: String,
    stacktrace: Option<String>
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum VioletLogSeverity {
    NoDefined = 0,
    Severe = 1,
    Error = 2,
    Warning = 3,
    Info = 4,
    Verbose = 5
}

impl VioletLog {
    pub fn new(severity: VioletLogSeverity, title: String, message: String) -> Self {
        Self {
            severity,
            title,
            message,
            stacktrace: None
        }
    }

    pub fn set_stacktrace(&mut self, stacktrace: String) {
        self.stacktrace = Some(stacktrace);
    }
}


impl From<u8> for VioletLogSeverity {
    fn from(el: u8) -> Self {
        match el {
            1 => VioletLogSeverity::Severe,
            2 => VioletLogSeverity::Error,
            3 => VioletLogSeverity::Warning,
            4 => VioletLogSeverity::Info,
            5 => VioletLogSeverity::Verbose,
            _ => VioletLogSeverity::NoDefined
        }
    }
}

impl From<VioletLogSeverity> for u8 {
    fn from(val: VioletLogSeverity) -> Self {
        match val {
            VioletLogSeverity::NoDefined => 0,
            VioletLogSeverity::Severe => 1,
            VioletLogSeverity::Error => 2,
            VioletLogSeverity::Warning => 3,
            VioletLogSeverity::Info => 4,
            VioletLogSeverity::Verbose => 5
        }
    }
}
