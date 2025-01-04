use crate::sys;
use std::fs::File;
use std::io;
use std::num::NonZeroU64;

#[derive(Debug, Default)]
pub struct ReflinkBlockBuilder<'a> {
    from: Option<&'a File>,
    from_offset: u64,
    to: Option<&'a File>,
    to_offset: u64,
    src_length: u64,
    cluster_size: Option<NonZeroU64>,
}

impl<'a> ReflinkBlockBuilder<'a> {
    #[must_use]
    pub fn from(mut self, from: &'a File) -> ReflinkBlockBuilder<'a> {
        self.from = Some(from);
        self
    }

    #[must_use]
    pub fn from_offset(mut self, from_offset: u64) -> Self {
        self.from_offset = from_offset;
        self
    }

    #[must_use]
    pub fn to(mut self, to: &'a File) -> ReflinkBlockBuilder<'a> {
        self.to = Some(to);
        self
    }

    #[must_use]
    pub fn to_offset(mut self, to_offset: u64) -> Self {
        self.to_offset = to_offset;
        self
    }

    #[must_use]
    pub fn src_length(mut self, src_length: NonZeroU64) -> Self {
        self.src_length = src_length.get();
        self
    }

    #[must_use]
    pub fn cluster_size(mut self, cluster_size: NonZeroU64) -> Self {
        self.cluster_size = Some(cluster_size);
        self
    }

    #[cfg_attr(not(windows), allow(unused_variables))]
    pub fn reflink_block(self) -> io::Result<()> {
        assert!(self.from.is_some(), "`from` is not set");
        assert!(self.to.is_some(), "`to` is not set");
        assert_ne!(self.src_length, 0, "`src_length` is not set");

        #[cfg(windows)]
        return sys::reflink_block(
            self.from.unwrap(),
            self.from_offset,
            self.to.unwrap(),
            self.to_offset,
            self.src_length,
            self.cluster_size,
        );
        #[cfg(not(windows))]
        Err(io::Error::other("Not implemented"))
    }
}
