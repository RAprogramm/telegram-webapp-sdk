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
