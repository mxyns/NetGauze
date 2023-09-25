use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Read, Write};
use std::process::{Command, Stdio};
use proc_macro2::{Span, TokenStream};
use syn;
use syn::{AttrStyle, FnArg, Generics, ImplItem, ImplItemFn, Item, ItemImpl, Pat, Path, PatIdent, PatType, ReturnType, Type, TypePtr, Visibility};
use syn::__private::quote::{format_ident, quote};
use syn::__private::ToTokens;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Const, Mut};

// TODO track and import types
// TODO find a solution for Vec<T>
// TODO capabilities why new not generated

pub fn run(pattern: &str, mut output: File) -> Result<(), Box<dyn Error>> {
    println!("generating c-api...");

    let files = glob::glob(pattern)?.filter_map(|file|
        if let Ok(file) = file {
            Some(file)
        } else { None }
    );

    let mut items = Vec::new();
    for file in files {
        let mut content = String::new();
        File::open(file)?.read_to_string(&mut content)?;
        let ast = syn::parse_file(&content)?;

        items.extend(extract_marked_items(ast));
    }

    let auto_gen = process_items(items);
    let text = generate_file(auto_gen);
    output.write_all(text.as_bytes())?;

    Ok(())
}

fn extract_marked_items(ast: syn::File) -> Vec<Item> {
    let mut items = Vec::new();

    for item in ast.items {
        match &item {
            Item::Impl(item_impl) => {
                let is_impl_capi = item_impl.attrs.iter().find(|attr| {
                    matches!(attr.style, AttrStyle::Outer)
                        && attr.path().is_ident("capi_impl")
                }).is_some();

                if !is_impl_capi {
                    continue;
                }

                let no_trait = item_impl.trait_.is_none();

                if !no_trait {
                    unimplemented!("impl of traits not supported yet, remove the capi_impl attribute")
                }

                items.push(item);
            }
            _ => {}
        }
    }

    items
}

fn process_items(items: Vec<Item>) -> TokenStream {
    let mut auto_gen = quote! {
        // automatically generated c-api using capi-gen
        #[allow(unused_imports)]
        use capi_gen;
        use capi_gen::owned_slice::OwnedSlice;
        use netgauze_iana::address_family::AddressFamily;
        use netgauze_iana::address_family::AddressType;
        use crate::capabilities::{ExtendedNextHopEncoding, ExtendedNextHopEncodingCapability};
    };

    for item in items {
        let new_quote = match item {
            Item::Impl(item_impl) => Some(process_impl(item_impl)),
            _ => None
        };

        auto_gen.extend(new_quote)
    }

    auto_gen
}

fn process_impl(impl_: ItemImpl) -> TokenStream {
    if has_generics(&impl_.generics) {
        unimplemented!("generic impl not supported yet")
    }

    let mut impl_quote = quote! {

    };

    let impl_receiver = impl_.self_ty;
    for item in impl_.items {
        let exported = match item {
            ImplItem::Fn(impl_item_fn) => if let Some(receiver) = impl_item_fn.sig.receiver() {
                process_method(impl_receiver.clone(), impl_item_fn)
            } else {
                process_function(impl_receiver.clone(), impl_item_fn)
            },
            _ => None
        };

        impl_quote.extend(exported);
    }

    impl_quote
}

fn impl_item_fn_supported(function: &ImplItemFn) -> bool {
    if !matches!(function.vis, Visibility::Public(_))
        || function.defaultness.is_some()
        || function.sig.abi.is_some()
        || has_generics(&function.sig.generics)
        || function.sig.asyncness.is_some()
        || function.sig.variadic.is_some()
    {
        println!("skipped function has unsupported qualifiers");
        false
    } else {
        true
    }
}

/// process a function into a wrapper
fn process_function(impl_receiver: Box<Type>, function: ImplItemFn) -> Option<TokenStream> {
    if !impl_item_fn_supported(&function) {
        return None;
    }

    let method_name = &function.sig.ident;
    let unsafety = &function.sig.unsafety;
    let function_inputs = &function.sig.inputs;
    let (new_function_inputs, ptr_imports) = clean_function_inputs(impl_receiver.clone(), function_inputs);
    let new_method_inputs = clean_method_inputs(function_inputs);
    let function_output = clean_function_output(&function.sig.output, impl_receiver.clone());

    let function_name = format_ident!("{}_{}", format!("{}", impl_receiver.as_ref().to_token_stream()), method_name);

    Some(
        quote! {



            #[allow(unused_braces)]
            #[no_mangle]
            pub #unsafety extern "C" fn #function_name ( #new_function_inputs ) #function_output {

                #ptr_imports

                return #unsafety { #impl_receiver::#method_name ( #new_method_inputs ) }
            }
        }
    )
}

/// process a method into a function wrapper
fn process_method(impl_receiver: Box<Type>, method: ImplItemFn) -> Option<TokenStream> {
    if !impl_item_fn_supported(&method) {
        return None;
    }

    let method_name = &method.sig.ident;
    let unsafety = &method.sig.unsafety;
    let function_inputs = &method.sig.inputs;
    let (new_function_inputs, ptr_imports) = clean_function_inputs(impl_receiver.clone(), function_inputs);
    let new_method_inputs = clean_method_inputs(function_inputs);
    let function_output = clean_function_output(&method.sig.output, impl_receiver.clone());

    let function_name = format_ident!("{}_{}", format!("{}", impl_receiver.as_ref().to_token_stream()), method_name);

    Some(
        quote! {



            #[allow(unused_braces)]
            #[no_mangle]
            pub #unsafety extern "C" fn #function_name ( #new_function_inputs ) #function_output {

                #ptr_imports

                return #unsafety { #impl_receiver::#method_name ( #new_method_inputs ) }
            }
        }
    )
}

/// generate a file based on the TokenStream content
fn generate_file(auto_gen: TokenStream) -> String {
    let mut text = String::from("// c-api automatically generated using capi-gen\n
    ");

    text.push_str(auto_gen.to_string().as_str());

    format_file(text)
}

fn format_file(content: String) -> String {
    let rustfmt = which::which("rustfmt").unwrap();
    let mut cmd = Command::new(&*rustfmt);

    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    // Write to stdin in a new thread, so that we can read from stdout on this
    // thread. This keeps the child from blocking on writing to its stdout which
    // might block us from writing to its stdin.
    let stdin_handle = ::std::thread::spawn(move || {
        let _ = child_stdin.write_all(content.as_bytes());
        content
    });

    let mut output = vec![];
    io::copy(&mut child_stdout, &mut output).unwrap();

    let status = child.wait().unwrap();
    let content = stdin_handle.join().expect(
        "The thread writing to rustfmt's stdin doesn't do \
             anything that could panic",
    );

    String::from_utf8(output).unwrap()
}

/// check if any generic is present
fn has_generics(generics: &Generics) -> bool {
    generics.gt_token.is_some()
        || generics.lt_token.is_some()
        || generics.where_clause.is_some()
        || !generics.params.is_empty()
}

/// clean method inputs for use in the method call
fn clean_method_inputs(inputs: &Punctuated<FnArg, Comma>) -> TokenStream {
    let mut new_inputs = None;

    for input in inputs.iter().rev() {

        let new_arg = match input {
            FnArg::Receiver(_) => quote!(self_),
            FnArg::Typed(typed) => typed.pat.to_token_stream()
        };

        new_inputs = if let None = new_inputs {
            Some(
                quote! {
                    #new_arg
                }
            )
        } else {
            Some(
                quote! {
                    #new_arg, #new_inputs
                }
            )
        }
    }

    new_inputs.unwrap()
}

// TODO replace Self in other input types eg: Self::GAT
/// clean method inputs for the function definition
fn clean_function_inputs(impl_receiver: Box<Type>, inputs: &Punctuated<FnArg, Comma>) -> (Punctuated<FnArg, Comma>, TokenStream) {
    let mut result = Punctuated::<FnArg, Comma>::new();
    let mut pointer_imports = quote!();

    for input in inputs {
        let input = match input {
            FnArg::Receiver(receiver) => {

                let mut new = PatType {
                    attrs: vec![],
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs: receiver.attrs.clone(),
                        by_ref: None,
                        mutability: None,
                        ident: format_ident!("self_"),
                        subpat: None,
                    })),
                    colon_token: Default::default(),
                    ty: receiver.ty.clone(),
                };

                if let Some(ptr_ty) = pointerify_ref_only(new.ty.clone(), Some(impl_receiver.clone())) {
                    new.ty = ptr_ty;
                    pointer_imports.extend(import_pointer(&new))
                }

                FnArg::Typed(new)
            }
            FnArg::Typed(typed) => {
                let mut clone = typed.clone();
                if let Some(new_ty) = pointerify_ref_only(typed.ty.clone(), None) {
                    clone.ty = new_ty;
                    pointer_imports.extend(import_pointer(typed));
                }
                FnArg::Typed(clone)
            }
        };

        result.push(input);
    }

    (result, pointer_imports)
}

fn import_pointer(typed: &PatType) -> TokenStream {
    if let Pat::Ident(pat_ident) = typed.pat.as_ref() {
        let ident = &pat_ident.ident;

        let as_func = if is_mut_ptr(typed).unwrap() {
            quote!(as_mut)
        } else {
            quote!(as_ref)
        };

        let msg = format!("bad {} pointer", ident.to_string());

        quote! {
            let #ident = unsafe { #ident.#as_func().expect(#msg) };
        }
    } else {
        unimplemented!("pattern {:?} is not supported as function input", typed.pat.to_token_stream())
    }
}

fn clean_function_output(ret: &ReturnType, impl_receiver: Box<Type>) -> ReturnType {

    match ret {
        ReturnType::Default => ret.clone(),
        ReturnType::Type(arrow, ty) => {
            let ty = unself_type(ty.clone(), impl_receiver);
            let ty = if let Some(new_type) = pointerify_ref_only(ty.clone(), None) {
                new_type
            } else {
                ty.clone()
            };

            ReturnType::Type(arrow.clone(), ty)
        }
    }
}

fn unself_type(ty: Box<Type>, original: Box<Type>) -> Box<Type> {

    println!("{:?}", ty.as_ref().to_token_stream());
    let ty = match ty.as_ref() {
        Type::Path(path) => {
            let path = if path.path.is_ident("Self") {
                let mut path = path.clone();

                path.path = syn::parse2(original.to_token_stream()).unwrap();

                path
            } else {
                path.clone()
            };

            Box::new(Type::Path(path))
        }
        _ => ty
    };

    ty
}

fn pointerify_ref_only(ty: Box<Type>, replace_self: Option<Box<Type>>) -> Option<Box<Type>> {

    match ty.as_ref() {
        Type::Reference(ref_type) => Some(pointerify_type(ty.clone(), ref_type.mutability.is_some(), replace_self)),
        _ => None
    }
}
fn pointerify_type(ty: Box<Type>, mut mutability: bool, replace_self: Option<Box<Type>>) -> Box<Type> {

    let new_ty = match ty.as_ref() {
        Type::Reference(type_reference) => {
            if let Some(_) = type_reference.mutability {
                mutability = true
            }

            if let Some(replace_ty) = replace_self {
                replace_ty.to_token_stream()
            } else {
                type_reference.elem.to_token_stream()
            }
        }
        _ => ty.to_token_stream()
    };

    let mutability = if mutability { Some(()) } else { None };

    Box::new(Type::Ptr(TypePtr {
        star_token: Default::default(),
        const_token: mutability.map(|_| Const::default()),
        mutability: mutability.map(|_| Mut::default()),
        elem: Box::new(Type::Verbatim(new_ty)),
    }))
}

fn is_mut_ptr(pat: &PatType) -> Result<bool, syn::Error> {
    match pat.ty.as_ref() {
        Type::Ptr(type_ptr) => Ok(type_ptr.mutability.is_some()),
        _ => Err(syn::Error::new(Span::call_site(), "is_mut_ref: type given is not a ptr"))
    }
}