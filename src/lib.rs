use proc_macro::TokenStream;

mod gql;
mod sql;

#[proc_macro_derive(Object)]
pub fn object(item: TokenStream) -> TokenStream {
    gql::object::object(item)
}

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    gql::handler::handler(attr, item)
}

#[proc_macro_attribute]
pub fn table(attr: TokenStream, item: TokenStream) -> TokenStream {
    sql::table::table(attr, item)
}

#[proc_macro_attribute]
pub fn select_from(attr: TokenStream, item: TokenStream) -> TokenStream {
    sql::select_from::select_from(attr, item)
}

#[proc_macro_attribute]
pub fn db_init(attr: TokenStream, item: TokenStream) -> TokenStream {
    sql::db_init(attr, item)
}
