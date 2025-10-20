//! Data loading states

#[derive(Debug, Clone)]
pub enum DataLoadingState<P> {
    Loading,
    Error(String),
    Loaded(P),
}

impl<P> DataLoadingState<P> {
    pub fn to_option<'a>(&'a self) -> Option<&'a P> {
        match self {
            Self::Loaded(data) => Some(data),
            _ => None,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Self::Loaded(_) => false,
            _ => true,
        }
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}
