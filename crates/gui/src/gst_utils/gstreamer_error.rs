use std::{error::Error, fmt::Display};

use gstreamer::glib;

#[derive(Debug, Clone)]
pub enum GstreamerError {
    GlibError(glib::Error),
    BoolError(Box<glib::BoolError>),
    PadLinkError(gstreamer::PadLinkError),
    StateChangeError(gstreamer::StateChangeError),
    FlowError(gstreamer::FlowError),
}

impl From<glib::Error> for GstreamerError {
    fn from(value: glib::Error) -> Self {
        GstreamerError::GlibError(value)
    }
}

impl From<glib::BoolError> for GstreamerError {
    fn from(value: glib::BoolError) -> Self {
        GstreamerError::BoolError(Box::new(value))
    }
}

impl From<gstreamer::PadLinkError> for GstreamerError {
    fn from(value: gstreamer::PadLinkError) -> Self {
        GstreamerError::PadLinkError(value)
    }
}

impl From<gstreamer::StateChangeError> for GstreamerError {
    fn from(value: gstreamer::StateChangeError) -> Self {
        GstreamerError::StateChangeError(value)
    }
}

impl From<gstreamer::FlowError> for GstreamerError {
    fn from(value: gstreamer::FlowError) -> Self {
        GstreamerError::FlowError(value)
    }
}

impl Display for GstreamerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GstreamerError::GlibError(e) => e.fmt(f),
            GstreamerError::BoolError(e) => e.fmt(f),
            GstreamerError::PadLinkError(e) => e.fmt(f),
            GstreamerError::StateChangeError(e) => e.fmt(f),
            GstreamerError::FlowError(e) => e.fmt(f),
        }
    }
}

impl Error for GstreamerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GstreamerError::GlibError(e) => Some(e),
            GstreamerError::BoolError(e) => Some(e),
            GstreamerError::PadLinkError(e) => Some(e),
            GstreamerError::StateChangeError(e) => Some(e),
            GstreamerError::FlowError(e) => Some(e),
        }
    }
}
