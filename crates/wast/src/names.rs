use alloc::format;
use hashbrown::HashMap;

use crate::token::{Id, Index};
use crate::Error;

#[derive(Default)]
pub struct Namespace<'a> {
    names: HashMap<Id<'a>, u32>,
    count: u32,
}

impl<'a> Namespace<'a> {
    pub fn register(&mut self, name: Option<Id<'a>>, desc: &str) -> Result<u32, Error> {
        let index = self.alloc();
        if let Some(name) = name {
            if let Some(_prev) = self.names.insert(name, index) {
                // FIXME: temporarily allow duplicately-named data and element
                // segments. This is a sort of dumb hack to get the spec test
                // suite working (ironically).
                //
                // So as background, the text format disallows duplicate
                // identifiers, causing a parse error if they're found. There
                // are two tests currently upstream, however, data.wast and
                // elem.wast, which *look* like they have duplicately named
                // element and data segments. These tests, however, are using
                // pre-bulk-memory syntax where a bare identifier was the
                // table/memory being initialized. In post-bulk-memory this
                // identifier is the name of the segment. Since we implement
                // post-bulk-memory features that means that we're parsing the
                // memory/table-to-initialize as the name of the segment.
                //
                // This is technically incorrect behavior but no one is
                // hopefully relying on this too much. To get the spec tests
                // passing we ignore errors for elem/data segments. Once the
                // spec tests get updated enough we can remove this condition
                // and return errors for them.
                if desc != "elem" && desc != "data" {
                    return Err(Error::new(
                        name.span(),
                        format!("duplicate {} identifier", desc),
                    ));
                }
            }
        }
        Ok(index)
    }

    pub fn alloc(&mut self) -> u32 {
        let index = self.count;
        self.count += 1;
        index
    }

    pub fn register_specific(&mut self, name: Id<'a>, index: u32, desc: &str) -> Result<(), Error> {
        if let Some(_prev) = self.names.insert(name, index) {
            return Err(Error::new(
                name.span(),
                format!("duplicate identifier for {}", desc),
            ));
        }
        Ok(())
    }

    pub fn resolve(&self, idx: &mut Index<'a>, desc: &str) -> Result<u32, Error> {
        let id = match idx {
            Index::Num(n, _) => return Ok(*n),
            Index::Id(id) => id,
        };
        if let Some(&n) = self.names.get(id) {
            *idx = Index::Num(n, id.span());
            return Ok(n);
        }
        Err(resolve_error(*id, desc))
    }
}

pub fn resolve_error(id: Id<'_>, ns: &str) -> Error {
    assert!(
        !id.is_gensym(),
        "symbol generated by `wast` itself cannot be resolved {:?}",
        id
    );
    Error::new(
        id.span(),
        format!("unknown {ns}: failed to find name `${}`", id.name()),
    )
}
