use std::cell::RefCell;

pub struct Ctxt {
    errors: RefCell<Option<Vec<darling::Error>>>,
}

impl Ctxt {
    pub fn new() -> Self {
        Self {
            errors: RefCell::new(Some(Vec::new())),
        }
    }

    pub fn darling_error(&self, err: darling::Error) {
        self.errors.borrow_mut().as_mut().unwrap().push(err)
    }

    pub fn check(self) -> Result<(), darling::Error> {
        let errors = self.errors.take().unwrap();

        match errors.len() {
            0 => Ok(()),
            _ => Err(darling::Error::multiple(errors)),
        }
    }
}

impl Drop for Ctxt {
    fn drop(&mut self) {
        if !std::thread::panicking() && self.errors.borrow().is_some() {
            panic!("forgot to check for errors");
        }
    }
}
