use crate::runtime::model::RuntimeModel;
use proc_macro2::TokenStream;
use quote::quote;

impl RuntimeModel {
    pub fn emit_async_runtime_impl(&self) -> TokenStream {
        let name = &self.name;
        let generics = &self.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let forward_field = &self.forward_field.name;
        let forward_ty = &self.forward_field.ty;

        quote! {
            impl #impl_generics ::interoptopus::pattern::asynk::AsyncRuntime for #name #ty_generics #where_clause {
                type T = <#forward_ty as ::interoptopus::pattern::asynk::AsyncRuntime>::T;

                fn spawn<Fn, F>(&self, f: Fn)
                where
                    Fn: FnOnce(Self::T) -> F,
                    F: ::std::future::Future<Output = ()> + Send + 'static,
                {
                    self.#forward_field.spawn(f)
                }
            }
        }
    }
}
