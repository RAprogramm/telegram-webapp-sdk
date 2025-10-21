// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use inventory::collect;

/// Represents a single routable page.
#[derive(Copy, Clone)]
pub struct Page {
    pub path:    &'static str,
    pub handler: fn()
}

collect!(Page);

/// Returns iterator over registered pages.
pub fn iter() -> inventory::iter<Page> {
    inventory::iter::<Page>
}
