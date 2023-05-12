
#[derive(Clone)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl From<LogLevel> for mint::Vector4<f32> {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Info => [0.5, 0.5, 0.5, 1.0].into(),
            LogLevel::Warning => [1.0, 0.64, 0.0, 1.0].into(),
            LogLevel::Error => [1.0, 0.0, 0.0, 1.0].into(),
        }
    }
}

#[derive(Clone)]
pub struct LogMessage {
    pub level: LogLevel,
    pub message: String,
}

impl LogMessage {
    pub fn new(level: LogLevel, message: &str) -> Self {
        let severity = match level {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        };

        Self {
            level,
            message: format!("[{}]: {}", severity, message),
        }
    }
}

pub struct Log {
    pub history: Vec<LogMessage>,
    pub history_index: i32,
}

impl Log {
    pub fn clear(&mut self) {
        self.history.clear();
        self.history_index = 0;
    }
}

impl Default for Log {
    fn default() -> Self {
        Self {
            history: Vec::new(),
            history_index: 0,
        }
    }
}
