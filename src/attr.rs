use proc_macro2::{Delimiter, TokenTree};
use syn::{
    Attribute, TraitItemMethod,
    visit_mut::{VisitMut, visit_item_trait_mut},
};

use crate::{
    diag::{DiagnosticExt, SpanExt},
    proxy::{parse_types, ProxyType},
    spanned::Spanned
};



/// Removes all `#[auto_impl]` attributes that are attached to methods of the
/// given trait.
crate fn remove_our_attrs(trait_def: &mut syn::ItemTrait) {
    struct AttrRemover;
    impl VisitMut for AttrRemover {
        fn visit_trait_item_method_mut(&mut self, m: &mut TraitItemMethod) {
            m.attrs.retain(|a| !is_our_attr(a));
        }
    }

    visit_item_trait_mut(&mut AttrRemover, trait_def);
}

/// Checks if the given attribute is "our" attribute. That means that it's path
/// is `auto_impl`.
crate fn is_our_attr(attr: &Attribute) -> bool {
    attr.path.segments.len() == 1
        && attr.path.segments.iter().next().map(|seg| {
            seg.ident == "auto_impl" && seg.arguments.is_empty()
        }).unwrap()
}

/// Tries to parse the given attribute as one of our own `auto_impl`
/// attributes. If it's invalid, an error is emitted and `Err(())` is returned.
/// You have to make sure that `attr` is one of our attrs with `is_our_attr`
/// before calling this function!
crate fn parse_our_attr(attr: &Attribute) -> Result<OurAttr, ()> {
    assert!(is_our_attr(attr));

    let tokens = attr.tts.clone().into_iter().collect::<Vec<_>>();
    let body = match &*tokens {
        [TokenTree::Group(g)] => g.stream(),
        _ => {
            return attr.tts.span()
                .err(format!("expected single group delimitted by`()`, found '{:?}'", tokens))
                .emit_with_attr_note();
        }
    };

    let mut it = body.clone().into_iter();

    let name = match it.next() {
        Some(TokenTree::Ident(x)) => x,
        Some(other) => {
            return Spanned::span(&other)
                .err(format!("expected ident, found '{}'", other))
                .emit_with_attr_note();
        }
        None => {
            return attr.tts.span()
                .err("expected ident, found nothing")
                .emit_with_attr_note();
        }
    };

    let params = match it.next() {
        Some(TokenTree::Group(ref g)) if g.delimiter() == Delimiter::Parenthesis => {
            g.stream()
        }
        Some(other) => {
            let msg = format!(
                "expected arguments for '{}' in parenthesis `()`, found `{}`",
                name,
                other,
            );
            return Spanned::span(&other)
                .err(msg)
                .emit_with_attr_note();
        }
        None => {
            let msg = format!(
                "expected arguments for '{}' in parenthesis `()`, found nothing",
                name,
            );
            return body.span()
                .err(msg)
                .emit_with_attr_note();
        }
    };

    let out = match () {
        () if name == "keep_default_for" => {
            let proxy_types = parse_types(params.into())?;
            OurAttr::KeepDefaultFor(proxy_types)
        }
        _ => {
            return Spanned::span(&name)
                .err(format!("invalid attribute '{}'", name))
                .emit_with_attr_note();
        }
    };

    Ok(out)
}

#[derive(Clone, PartialEq, Debug)]
crate enum OurAttr {
    KeepDefaultFor(Vec<ProxyType>),
}
