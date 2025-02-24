use std::{borrow::Cow, str::CharIndices};

/// A reference id for source codes.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct SrcId(pub usize);

impl From<usize> for SrcId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

/// Source code used by [`Renderer`]
pub trait Src {
    /// Return source file name.
    fn name(&self) -> &str;

    /// Return a slice pointing to source content.
    fn content(&self) -> &[u8];

    /// Return text-based content iterator.
    fn text_based(&self) -> Option<CharIndices<'_>>;
}

/// A default implementation of [`Src`] trait.
pub struct ContentWithName {
    name: Cow<'static, str>,
    content: Cow<'static, str>,
}

impl Src for ContentWithName {
    fn name(&self) -> &str {
        &self.name
    }

    fn content(&self) -> &[u8] {
        self.content.as_bytes()
    }

    fn text_based(&self) -> Option<CharIndices<'_>> {
        Some(self.content.char_indices())
    }
}

/// Create a source.
pub fn src<N, C>(name: N, content: C) -> ContentWithName
where
    Cow<'static, str>: From<N> + From<C>,
{
    ContentWithName {
        name: name.into(),
        content: content.into(),
    }
}

/// A collection of source codes.
#[derive(Default)]
pub struct Sources {
    srcs: Vec<Box<dyn Src>>,
}

impl Sources {
    /// Add a new sources.
    pub fn add<S>(&mut self, src: S) -> SrcId
    where
        S: Src + 'static,
    {
        let id = SrcId(self.srcs.len());

        self.srcs.push(Box::new(src));

        id
    }

    /// Get the source by id.
    ///
    /// If the source is not eixsts, returns `None`.
    pub fn get(&self, id: SrcId) -> Option<&dyn Src> {
        self.srcs.get(id.0).map(|v| v.as_ref())
    }
}
