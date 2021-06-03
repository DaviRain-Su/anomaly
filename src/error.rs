//! Error type which is generic around a `Kind`

use crate::{BoxError, Context};
use crate::StdError;
use std::{
    fmt::{self, Debug, Display},
    ops::Deref,
};

/// Error type which is generic around a `Kind`.
///
/// Provides a `Box`-ed wrapper around a [`Context`], ensuring error
/// propagation is a cheap operation (pointer copy).
#[derive(Debug)]
pub struct Error<Kind>(Box<Context<Kind>>)
where
    Kind: Clone + Debug + Display + Into<BoxError>;

impl<Kind> Deref for Error<Kind>
where
    Kind: Clone + Debug + Display + Into<BoxError>,
{
    type Target = Context<Kind>;

    fn deref(&self) -> &Context<Kind> {
        &self.0
    }
}

impl<Kind> Display for Error<Kind>
where
    Kind: Clone + Debug + Display + Into<BoxError>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<Kind> StdError for Error<Kind>
where
    Kind: Clone + Debug + Display + Into<BoxError>,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.source()
    }
}

impl<Kind> From<Kind> for Error<Kind>
where
    Kind: Clone + Debug + Display + Into<BoxError>,
{
    fn from(kind: Kind) -> Self {
        Context::new(kind, None).into()
    }
}

impl<Kind> From<Context<Kind>> for Error<Kind>
where
    Kind: Clone + Debug + Display + Into<BoxError>,
{
    fn from(context: Context<Kind>) -> Self {
        Error(Box::new(context))
    }
}
