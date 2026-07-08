// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use inventory::collect;

/// Represents a single routable page.
#[derive(Copy, Clone)]
pub struct Page {
    /// URL path this page is mounted at.
    pub path:    &'static str,
    /// Callback rendering the page when its path is matched.
    pub handler: fn()
}

collect!(Page);

/// Returns iterator over registered pages.
pub fn iter() -> inventory::iter<Page> {
    inventory::iter::<Page>
}
