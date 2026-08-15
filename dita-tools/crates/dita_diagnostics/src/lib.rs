use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DiagError {
    pub path: PathBuf,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct DiagWarning {
    pub path: PathBuf,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum Diagnostic {
    Error(DiagError),
    Warning(DiagWarning),
}

impl Diagnostic {
    pub fn error(path: impl Into<PathBuf>, msg: impl Into<String>) -> Self {
        Self::Error(DiagError {
            path: path.into(),
            message: msg.into(),
        })
    }

    pub fn warning(path: impl Into<PathBuf>, msg: impl Into<String>) -> Self {
        Self::Warning(DiagWarning {
            path: path.into(),
            message: msg.into(),
        })
    }

    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    #[must_use]
    pub fn path(&self) -> &PathBuf {
        match self {
            Self::Error(e) => &e.path,
            Self::Warning(w) => &w.path,
        }
    }

    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::Error(e) => &e.message,
            Self::Warning(w) => &w.message,
        }
    }
}

#[derive(Debug, Default)]
pub struct DiagnosticBag {
    pub items: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn push(&mut self, d: Diagnostic) {
        self.items.push(d);
    }

    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(Diagnostic::is_error)
    }

    #[must_use]
    pub fn error_count(&self) -> usize {
        self.items.iter().filter(|d| d.is_error()).count()
    }

    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.items.iter().filter(|d| !d.is_error()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag_detects_errors() {
        let mut bag = DiagnosticBag::default();
        bag.push(Diagnostic::warning("a.dita", "unused topic"));
        assert!(!bag.has_errors());
        assert_eq!(bag.warning_count(), 1);

        bag.push(Diagnostic::error("b.dita", "broken ref"));
        assert!(bag.has_errors());
        assert_eq!(bag.error_count(), 1);
    }
}
