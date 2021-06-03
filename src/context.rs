//! Error contexts

use super::BoxError;
use crate::StdError;
#[cfg(feature = "backtrace")]
use backtrace::Backtrace;
use std::fmt::{self, Debug, Display};

/// Error context: stores an error source (as a [`BoxError`]) and backtrace
/// along with an error `Kind`.
#[derive(Debug)]
pub struct Context<Kind>
where
    Kind: Clone + Debug + Display + Into<BoxError>,
{
    /// Kind of error
    kind: Kind,

    /// Source of the error
    source: Option<BoxError>,

    /// Backtrace where error occurred
    #[cfg(feature = "backtrace")]
    backtrace: Option<Backtrace>,
}

impl<Kind> Context<Kind>
where
    Kind: Clone + Debug + Display + Into<BoxError>,
{
    /// Create a new error context
    pub fn new(kind: Kind, source: Option<BoxError>) -> Self {
        Context {
            kind,
            source,
            #[cfg(feature = "backtrace")]
            backtrace: Some(Backtrace::new()),
        }
    }

    /// Get the kind of error
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Get the backtrace associated with this error (if available)
    #[cfg(feature = "backtrace")]
    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.backtrace.as_ref()
    }
}

impl<Kind> Display for Context<Kind>
where
    Kind: Clone + Debug + Display + Into<BoxError>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.kind)?;

        if let Some(ref source) = self.source {
            write!(f, ": {}", source)?;
        }

        Ok(())
    }
}

impl<Kind> From<Kind> for Context<Kind>
where
    Kind: Clone + Debug + Display + Into<BoxError>,
{
    fn from(kind: Kind) -> Context<Kind> {
        Self::new(kind, None)
    }
}

impl<Kind> StdError for Context<Kind>
where
    Kind: Clone + Debug + Display + Into<BoxError>,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn StdError + 'static))
    }
}
