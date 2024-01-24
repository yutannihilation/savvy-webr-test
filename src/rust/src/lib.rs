#![allow(unused_variables)]
use savvy::savvy;
use savvy::OwnedStringSexp;

/// @export
#[savvy]
fn int_vec() -> savvy::Result<savvy::Sexp> {
    let out = OwnedStringSexp::new(3)?;
    out.into()
}
