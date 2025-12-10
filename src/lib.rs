// Copyright © 2025 Stephan Kunz
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod any_extensions;
mod port;
mod port_declaration;
mod port_implementation;

// flatten
