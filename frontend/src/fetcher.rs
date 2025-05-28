//! General functions to "asynchrously" fetch data from some remote api
//! since we have no native multitask support in WASM
//!
//! This is a one-time fetch, if fails it won't try again, once finished it'll be useless.

use std::sync::{Arc, Mutex};

/// Wrapper over the fetched data
pub struct FetchedData<T> {
    /// The final data
    pub data: Option<T>,
    /// Function to convert an [`ehttp::Response`] to the final data
    ///
    /// If the conversion failed, it should return a None
    convert: fn(&ehttp::Response) -> Option<T>,
    response: Arc<Mutex<Option<Result<ehttp::Response, String>>>>,
    /// Whether we have finished parsing the data
    ///
    /// In this state, this [`FetchedData`] is no longer "functional" other than reading the
    /// wrapped data.
    /// If everything good, the data field should be Some(T), but if it's None, it means either:
    /// - The request failed (HTTP error)
    /// - Or the fetched data cannot be parsed properly
    ///
    /// In this case, you have to create a new [`FetchedData`] to try again.
    pub done: bool,
}

impl<T> FetchedData<T> {
    /// Creates a [`FetchedData`] and makes the request
    pub fn new(url: String, convert: fn(&ehttp::Response) -> Option<T>) -> Self {
        let response = Arc::new(Mutex::new(None));
        let response_clone = response.clone();
        let req = ehttp::Request::get(url);
        ehttp::fetch(req, move |response| {
            *response_clone.lock().unwrap() = Some(response);
        });
        Self {
            data: None,
            convert,
            response,
            done: false,
        }
    }

    /// This should be called each cycle
    pub fn poll(&mut self, ctx: &egui::Context) {
        if !self.done {
            let response = self.got_response();
            if response {
                let data = self.response.lock().unwrap();
                if let Some(Ok(real_response)) = &*data {
                    self.data = (self.convert)(real_response);
                }
                ctx.request_repaint();
                self.done = true;
            }
        }
    }

    fn got_response(&self) -> bool {
        if let Ok(lock) = self.response.try_lock() {
            return lock.is_some();
        }
        false
    }
}
