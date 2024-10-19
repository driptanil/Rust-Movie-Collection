#[derive(Debug, derive_more::Display)]
pub enum Error {
    #[display(fmt = "Error {}: {}", _0, _1)] 
    External(StatusCode, String),

    #[display(fmt = "Status: {}\n{}", _0, _1)]
    Internal(StatusCode, eyre::Error),

    #[display(fmt = "{}", _0)] Other(eyre::Error),
}

#[allow(dead_code)]
impl Error {
    fn with_error_log<T: Into<eyre::Error>>(&self, source: T) {
        let e: eyre::Error = source.into();
        tracing::error!("{:?}", e);
    }

    fn with_debug_log<T: Into<eyre::Error>>(&self, source: T) {
        let e: eyre::Error = source.into();
        tracing::debug!("{:?}", e);
    }
}
