//! Module with the common definitions for  the
//! [fields](https://github.com/typesense/typesense/blob/v0.19.0/include/field.)
//! available in Typesense.

mod field_type;
pub use field_type::*;
pub use typesense_codegen::models::{Field, FieldEmbed};

/// Builder for the `Field` struct.
#[derive(Debug, Default)]
pub struct FieldBuilder {
    name: String,
    typesense_type: FieldType,
    optional: Option<bool>,
    facet: Option<bool>,
    index: Option<bool>,
    locale: Option<String>,
    sort: Option<bool>,
    infix: Option<bool>,
    num_dim: Option<i32>,
    drop: Option<bool>,
    embed: Option<Box<FieldEmbed>>,
	reference: Option<String>,
    store: Option<bool>,
    vec_dist: Option<String>,
    range_index: Option<bool>,
    stem: Option<bool>,
    stem_dictionary: Option<String>,
    token_separators: Option<Vec<String>>,
    symbols_to_index: Option<Vec<String>>,
}

impl FieldBuilder {
    /// Create a Builder
    #[inline]
    pub fn new(name: impl Into<String>, typesense_type: FieldType) -> Self {
        Self {
            name: name.into(),
            typesense_type,
            ..Default::default()
        }
    }

    /// Set if field is optional.
    #[inline]
    pub fn optional(mut self, optional: Option<bool>) -> Self {
        self.optional = optional;
        self
    }

    /// Set if field is facet.
    #[inline]
    pub fn facet(mut self, facet: Option<bool>) -> Self {
        self.facet = facet;
        self
    }

    /// Set if field is index.
    #[inline]
    pub fn index(mut self, index: Option<bool>) -> Self {
        self.index = index;
        self
    }

    /// Set field locale.
    #[inline]
    pub fn locale(mut self, locale: Option<String>) -> Self {
        self.locale = locale;
        self
    }

    /// Set sort attribute for field
    #[inline]
    pub fn sort(mut self, sort: Option<bool>) -> Self {
        self.sort = sort;
        self
    }

    /// Set infix attribute for field
    #[inline]
    pub fn infix(mut self, infix: Option<bool>) -> Self {
        self.infix = infix;
        self
    }

    /// Set num_dim attribute for field
    #[inline]
    pub fn num_dim(mut self, num_dim: Option<i32>) -> Self {
        self.num_dim = num_dim;
        self
    }

    /// Set drop attribute for field
    #[inline]
    pub fn drop(mut self, drop: Option<bool>) -> Self {
        self.drop = drop;
        self
    }

    /// Set embed attribute for field
    #[inline]
    pub fn embed(mut self, embed: Option<Box<FieldEmbed>>) -> Self {
        self.embed = embed;
        self
    }

    /// Set reference attribute for field
    #[inline]
    pub fn reference(mut self, reference: Option<String>) -> Self {
        self.reference = reference;
        self
    }

    /// Set store attribute for field
    #[inline]
    pub fn store(mut self, store: Option<bool>) -> Self {
        self.store = store;
        self
    }

    /// Set vec_dist attribute for field
    #[inline]
    pub fn vec_dist(mut self, vec_dist: Option<String>) -> Self {
        self.vec_dist = vec_dist;
        self
    }

    /// Set range_index attribute for field
    #[inline]
    pub fn range_index(mut self, range_index: Option<bool>) -> Self {
        self.range_index = range_index;
        self
    }

    /// Set stem attribute for field
    #[inline]
    pub fn stem(mut self, stem: Option<bool>) -> Self {
        self.stem = stem;
        self
    }

    /// Set stem_dictionary attribute for field
    #[inline]
    pub fn stem_dictionary(mut self, stem_dictionary: Option<String>) -> Self {
        self.stem_dictionary = stem_dictionary;
        self
    }

    /// Set token_separators attribute for field
    #[inline]
    pub fn token_separators(mut self, token_separators: Option<Vec<String>>) -> Self {
        self.token_separators = token_separators;
        self
    }

    /// Set symbols_to_index attribute for field
    #[inline]
    pub fn symbols_to_index(mut self, symbols_to_index: Option<Vec<String>>) -> Self {
        self.symbols_to_index = symbols_to_index;
        self
    }

    /// Create a `Field` with the current values of the builder,
    #[inline]
    pub fn build(self) -> Field {
        Field {
            name: self.name,
            r#type: self.typesense_type,
            optional: self.optional,
            facet: self.facet,
            index: self.index,
            locale: self.locale,
            sort: self.sort,
            infix: self.infix,
            num_dim: self.num_dim,
            drop: self.drop,
            embed: self.embed,
            reference: self.reference,
            store: self.store,
            vec_dist: self.vec_dist,
            range_index: self.range_index,
            stem: self.stem,
            stem_dictionary: self.stem_dictionary,
            token_separators: self.token_separators,
            symbols_to_index: self.symbols_to_index,
        }
    }
}
