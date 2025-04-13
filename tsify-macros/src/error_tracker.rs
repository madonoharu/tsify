use std::cell::RefCell;

/// Tracks errors during macro expansion. This struct implements a panic on `Drop`
/// if there are accumulated errors that weren't checked.
///
/// By using an error tracker, you can accumulate errors inside of closures and still propagate
/// them when needed.
///
/// # Example
/// ```ignore
/// let errors = ErrorTracker::new();
/// errors.syn_error(syn::Error::new_spanned(ident, "error message"));
/// // Make sure to check the errors or else you'll get a panic.
/// errors.check()?;
/// ```
pub struct ErrorTracker {
    errors: RefCell<Option<Vec<syn::Error>>>,
}

impl ErrorTracker {
    pub fn new() -> Self {
        Self {
            errors: RefCell::new(Some(Vec::new())),
        }
    }

    /// Add a `syn::Error` to the list of errors.
    pub fn syn_error(&self, err: syn::Error) {
        self.errors.borrow_mut().as_mut().unwrap().push(err)
    }

    /// Return all accumulated errors. This also clears the list of errors.
    pub fn check(self) -> syn::Result<()> {
        let mut errors = self.errors.take().unwrap().into_iter();

        let mut combined = match errors.next() {
            Some(first) => first,
            None => return Ok(()),
        };

        for rest in errors {
            combined.combine(rest);
        }

        Err(combined)
    }
}

impl Drop for ErrorTracker {
    fn drop(&mut self) {
        if !std::thread::panicking() && self.errors.borrow().is_some() {
            panic!("forgot to check for errors");
        }
    }
}
