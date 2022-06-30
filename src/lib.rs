//! Error recovery utilities.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Fcatch)](https://iteration-square.schichler.dev/#narrow/stream/project.2Fcatch)

#![doc(html_root_url = "https://docs.rs/catch/0.0.1")]
#![no_std]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::semicolon_if_nothing_returned)]

use core::iter;
use this_is_fine::Fine;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

/// [`Result`] and [`Fine`] extensions for splitting off errors.
pub trait CatchExt<E>: Sized {
	/// The resulting type after the error has been removed.
	type OkOnly;

	/// Calls `handler` with any error, leaving the success as-available.
	fn catch<F>(self, handler: F) -> Self::OkOnly
	where
		F: FnOnce(E);

	/// [`Extend`]s `collection` with any error, leaving the success as-available.
	fn catch_item<C>(self, collection: &mut C) -> Self::OkOnly
	where
		C: Extend<E>;

	/// [`Extend`]s `collection` with any error (first converting it via [`Into`]), leaving the success as-available.
	fn catch_into_item<C, Item>(self, collection: &mut C) -> Self::OkOnly
	where
		C: Extend<Item>,
		E: Into<Item>;
}

/// Strips any `E`, leaving an [`Option<T>`].
impl<T, E> CatchExt<E> for Result<T, E> {
	type OkOnly = Option<T>;

	fn catch<F>(self, handler: F) -> Self::OkOnly
	where
		F: FnOnce(E),
	{
		match self {
			Ok(t) => Some(t),
			Err(e) => {
				handler(e);
				None
			}
		}
	}

	fn catch_item<C>(self, collection: &mut C) -> Self::OkOnly
	where
		C: Extend<E>,
	{
		match self {
			Ok(t) => Some(t),
			Err(e) => {
				collection.extend(iter::once(e));
				None
			}
		}
	}

	fn catch_into_item<C, Item>(self, collection: &mut C) -> Self::OkOnly
	where
		C: Extend<Item>,
		E: Into<Item>,
	{
		match self {
			Ok(t) => Some(t),
			Err(e) => {
				collection.extend(iter::once(e.into()));
				None
			}
		}
	}
}

/// Strips any `E`, leaving a `T`.
impl<T, E> CatchExt<E> for Fine<T, E> {
	type OkOnly = T;

	fn catch<F>(self, handler: F) -> Self::OkOnly
	where
		F: FnOnce(E),
	{
		self.1.unwrap_or_else(handler);
		self.0
	}

	fn catch_item<C>(self, collection: &mut C) -> Self::OkOnly
	where
		C: Extend<E>,
	{
		self.1.unwrap_or_else(|e| collection.extend(iter::once(e)));
		self.0
	}

	fn catch_into_item<C, Item>(self, collection: &mut C) -> Self::OkOnly
	where
		C: Extend<Item>,
		E: Into<Item>,
	{
		self.1
			.unwrap_or_else(|e| collection.extend(iter::once(e.into())));
		self.0
	}
}
