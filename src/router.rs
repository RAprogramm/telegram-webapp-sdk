// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Simple in-memory page router.
//!
//! Collects page definitions and executes their handlers in registration
//! order. Used by the `telegram_router!` macro by default.
//!
//! # Examples
//!
//! ```no_run
//! use telegram_webapp_sdk::router::Router;
//!
//! fn index() {}
//!
//! Router::new().register("/", index).start();
//! ```

#[cfg(feature = "macros")]
use crate::pages::Page;
#[cfg(not(feature = "macros"))]
#[derive(Copy, Clone)]
struct Page {
    #[allow(dead_code)]
    path:    &'static str,
    handler: fn()
}

/// Sequential router executing registered page handlers.
#[derive(Default)]
pub struct Router {
    pages: Vec<Page>
}

impl Router {
    /// Creates an empty router.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a page handler associated with `path` and returns the updated
    /// router.
    pub fn register(mut self, path: &'static str, handler: fn()) -> Self {
        self.pages.push(Page {
            path,
            handler
        });
        self
    }

    /// Starts the router, invoking handlers in order of registration.
    pub fn start(self) {
        for page in self.pages {
            (page.handler)();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    fn noop() {}

    #[test]
    fn registers_pages() {
        let router = Router::new().register("/", noop);
        assert_eq!(router.pages.len(), 1);
    }

    static COUNT: AtomicUsize = AtomicUsize::new(0);

    fn handler() {
        COUNT.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn starts_registered_pages() {
        COUNT.store(0, Ordering::SeqCst);
        Router::new().register("/", handler).start();
        assert_eq!(COUNT.load(Ordering::SeqCst), 1);
    }
}
