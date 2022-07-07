use convert_case::{Case, Casing as _};
use quote::quote;
use syn::{
  parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated, token::Paren, DataEnum,
  DeriveInput, Ident, LitInt, LitStr, Token,
};

struct Annotations {
  _paren: Paren,
  annotations: Punctuated<Annotation, Token![,]>,
}

impl Parse for Annotations {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let in_paren;
    let _paren = parenthesized!(in_paren in input);
    let annotations = Punctuated::parse_terminated(&in_paren)?;

    Ok(Self {
      _paren,
      annotations,
    })
  }
}

enum Annotation {
  Name(NameAnnot),
  Index(IndexAnnot),
}

impl Parse for Annotation {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let input_ = input.fork();

    if input_.parse::<NameAnnot>().is_ok() {
      let name_annot = input.parse()?;
      Ok(Annotation::Name(name_annot))
    } else {
      let index_annot = input.parse()?;
      Ok(Annotation::Index(index_annot))
    }
  }
}

struct NameAnnot {
  _name: Ident,
  _equal_token: Token![=],
  name_value: LitStr,
}

impl Parse for NameAnnot {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let _name: Ident = input.parse()?;

    if _name.to_string() != "name" {
      return Err(input.error("path should be name"));
    }

    let _equal_token = input.parse()?;
    let name_value = input.parse()?;

    Ok(Self {
      _name,
      _equal_token,
      name_value,
    })
  }
}

struct IndexAnnot {
  _index: Ident,
  _equal_token: Token![=],
  index_value: LitInt,
}

impl Parse for IndexAnnot {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let _index: Ident = input.parse()?;

    if _index.to_string() != "index" {
      return Err(input.error("path should be index"));
    }

    let _equal_token = input.parse()?;
    let index_value = input.parse()?;

    Ok(Self {
      _index,
      _equal_token,
      index_value,
    })
  }
}

#[proc_macro_derive(Semantics, attributes(sem))]
pub fn derive_semantics(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_item = parse_macro_input!(item as DeriveInput);

  match derive_item.data {
    syn::Data::Enum(enum_item) => on_enum(derive_item.ident, enum_item),
    syn::Data::Struct(_) => todo!(),
    syn::Data::Union(_) => todo!(),
  }
}

fn on_enum(enum_ident: Ident, enum_item: DataEnum) -> proc_macro::TokenStream {
  let mut explicit_indices = 0;
  let mut name_branches = Vec::new();
  let mut index_branches = Vec::new();
  let varians_len = enum_item.variants.len();

  for (k, variant) in enum_item.variants.into_iter().enumerate() {
    let k = k as u8;
    let variant_ident = &variant.ident;

    let mut name = None;
    let mut index = None;

    for attr in variant.attrs {
      match attr.path.get_ident() {
        Some(ident) if ident.to_string() != "sem" => continue,
        None => continue,
        _ => (),
      };

      let tokens = attr.tokens.into();
      let annots = parse_macro_input!(tokens as Annotations);

      for annot in annots.annotations {
        match annot {
          Annotation::Name(name_annot) if name.is_none() => {
            let name_value = name_annot.name_value;
            name = Some(quote! { #name_value });
          }

          Annotation::Index(index_annot) if index.is_none() => {
            let index_value = index_annot.index_value;
            index = Some(quote! { #index_value });
            explicit_indices += 1;
          }

          _ => {
            return quote! { compilation_error!("trying to set the same annotation twice") }.into()
          }
        }
      }

      break;
    }

    if name.is_none() {
      let new_name = variant.ident.to_string().to_case(Case::Snake);
      name = Some(quote! { #new_name });
    }

    if index.is_none() {
      index = Some(quote! { #k });
    }

    name_branches.push(quote! {
      #enum_ident::#variant_ident => #name
    });

    index_branches.push(quote! {
      #enum_ident::#variant_ident => #index
    });
  }

  if explicit_indices > 0 && explicit_indices < varians_len {
    return quote! { compilation_error!("some variants have an explicit index and some don’t") }
      .into();
  }

  let q = quote! {
    impl semantics::Semantics for #enum_ident {
      type Name = &'static str;
      type Index = u8;

      fn name(&self) -> Self::Name {
        match *self {
          #(#name_branches),*
        }
      }

      fn index(&self) -> Self::Index {
        match *self {
          #(#index_branches),*
        }
      }
    }
  };

  q.into()
}
