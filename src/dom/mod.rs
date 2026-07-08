// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Thin ergonomic wrappers over `web-sys` for DOM access.
//!
//! Provides a [`Document`] handle for resolving the current document and an
//! [`ElementExt`] trait with convenience methods for manipulating elements.

/// Document access helpers.
pub mod document;
/// Element extension trait.
pub mod element;

pub use document::Doc as Document;
pub use element::ElementExt;
