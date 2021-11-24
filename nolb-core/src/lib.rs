#![feature(
    async_closure,
    type_alias_impl_trait,
    generic_associated_types,
    min_specialization,
    const_trait_impl,
    const_generics_defaults
)]
#![deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::manual_ok_or,
    clippy::redundant_else,
    clippy::redundant_closure_for_method_calls,
    clippy::same_functions_in_if_condition
)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
