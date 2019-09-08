use proc_macro2::TokenStream as HelperTokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Ident, ItemStruct};

// impl LocalFileManager {
//     fn allocate(init_struct: InitLocalFileManager) -> Box<LocalFileManager> {
//         {
//             ::std::io::_print(::std::fmt::Arguments::new_v1(
//                 &["Allocating new VTable for ", "\n"],
//                 &match (&"LocalFileManager",) {
//                     (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
//                 },
//             ));
//         };
//         unsafe extern "stdcall" fn non_delegating_query_interface(
//             this: *mut <dyn com::IUnknown as com::ComInterface>::VPtr,
//             riid: *const winapi::shared::guiddef::IID,
//             ppv: *mut *mut winapi::ctypes::c_void,
//         ) -> HRESULT {
//             let this = this.sub(1usize) as *mut LocalFileManager;
//             (*this).inner_query_interface(riid, ppv)
//         }
//         unsafe extern "stdcall" fn non_delegating_add_ref(
//             this: *mut <dyn com::IUnknown as com::ComInterface>::VPtr,
//         ) -> u32 {
//             let this = this.sub(1usize) as *mut LocalFileManager;
//             (*this).inner_add_ref()
//         }
//         unsafe extern "stdcall" fn non_delegating_release(
//             this: *mut <dyn com::IUnknown as com::ComInterface>::VPtr,
//         ) -> u32 {
//             let this = this.sub(1usize) as *mut LocalFileManager;
//             (*this).inner_release()
//         }
//         type __iunknown_vtable_type = <dyn com::IUnknown as com::ComInterface>::VTable;
//         let __non_del_unk_vtable = __iunknown_vtable_type {
//             QueryInterface: non_delegating_query_interface,
//             Release: non_delegating_release,
//             AddRef: non_delegating_add_ref,
//         };
//         let __non_delegating_unk = Box::into_raw(Box::new(__non_del_unk_vtable));
//         let ilocalfilemanager_vtable =
//             <dyn ILocalFileManager as ::com::ProductionComInterface<LocalFileManager>>::vtable::<
//                 ::com::offset::Zero,
//             >();
//         let __ilocalfilemanagervptr = Box::into_raw(Box::new(ilocalfilemanager_vtable));
//         let out = LocalFileManager {
//             __ilocalfilemanagervptr,
//             __non_delegating_unk,
//             __iunk_to_use: std::ptr::null_mut::<<dyn com::IUnknown as com::ComInterface>::VPtr>(),
//             __refcnt: 0,
//             __init_struct: init_struct,
//         };
//         Box::new(out)
//     }
//     pub(crate) fn set_iunknown(
//         &mut self,
//         aggr: *mut <dyn com::IUnknown as com::ComInterface>::VPtr,
//     ) {
//         if aggr.is_null() {
//             self.__iunk_to_use = &self.__non_delegating_unk as *const _
//                 as *mut <dyn com::IUnknown as com::ComInterface>::VPtr;
//         } else {
//             self.__iunk_to_use = aggr;
//         }
//     }
//     pub(crate) fn inner_query_interface(
//         &mut self,
//         riid: *const winapi::shared::guiddef::IID,
//         ppv: *mut *mut winapi::ctypes::c_void,
//     ) -> HRESULT {
//         {
//             ::std::io::_print(::std::fmt::Arguments::new_v1(
//                 &["Non delegating QI\n"],
//                 &match () {
//                     () => [],
//                 },
//             ));
//         };
//         unsafe {
//             let riid = &*riid;
//             if winapi::shared::guiddef::IsEqualGUID(riid, &com::IID_IUNKNOWN) {
//                 *ppv = &self.__non_delegating_unk as *const _ as *mut winapi::ctypes::c_void;
//             } else if <dyn ILocalFileManager as com::ComInterface>::iid_in_inheritance_chain(
//                 riid,
//             ) {
//                 *ppv = &self.__ilocalfilemanagervptr as *const _ as *mut winapi::ctypes::c_void;
//             } else {
//                 *ppv = std::ptr::null_mut::<winapi::ctypes::c_void>();
//                 {
//                     ::std::io::_print(::std::fmt::Arguments::new_v1(
//                         &["Returning NO INTERFACE.\n"],
//                         &match () {
//                             () => [],
//                         },
//                     ));
//                 };
//                 return winapi::shared::winerror::E_NOINTERFACE;
//             }
//             {
//                 ::std::io::_print(::std::fmt::Arguments::new_v1(
//                     &["Successful!.\n"],
//                     &match () {
//                         () => [],
//                     },
//                 ));
//             };
//             self.inner_add_ref();
//             NOERROR
//         }
//     }
//     pub(crate) fn inner_add_ref(&mut self) -> u32 {
//         self.__refcnt += 1;
//         {
//             ::std::io::_print(::std::fmt::Arguments::new_v1(
//                 &["Count now ", "\n"],
//                 &match (&self.__refcnt,) {
//                     (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
//                 },
//             ));
//         };
//         self.__refcnt
//     }
//     pub(crate) fn inner_release(&mut self) -> u32 {
//         self.__refcnt -= 1;
//         {
//             ::std::io::_print(::std::fmt::Arguments::new_v1(
//                 &["Count now ", "\n"],
//                 &match (&self.__refcnt,) {
//                     (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
//                 },
//             ));
//         };
//         let count = self.__refcnt;
//         if count == 0 {
//             {
//                 ::std::io::_print(::std::fmt::Arguments::new_v1(
//                     &["Count is 0 for ", ". Freeing memory...\n"],
//                     &match (&"LocalFileManager",) {
//                         (arg0,) => {
//                             [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
//                         }
//                     },
//                 ));
//             };
//             unsafe {
//                 Box::from_raw(self as *const _ as *mut LocalFileManager);
//             }
//         }
//         count
//     }
//     pub fn get_class_object() -> Box<LocalFileManagerClassFactory> {
//         <LocalFileManagerClassFactory>::new()
//     }
// }

pub fn generate(
    base_itf_idents: &[Ident],
    aggr_itf_idents: &HashMap<Ident, Vec<Ident>>,
    struct_item: &ItemStruct,
) -> HelperTokenStream {
    let real_ident = macro_utils::get_real_ident(&struct_item.ident);
    let allocate_fn = gen_allocate_fn(base_itf_idents, struct_item);
    let set_iunknown_fn = gen_set_iunknown_fn();
    let inner_iunknown_fns = gen_inner_iunknown_fns(base_itf_idents, aggr_itf_idents, struct_item);
    let get_class_object_fn = gen_get_class_object_fn(struct_item);

    quote!(
        impl #real_ident {
            #allocate_fn
            #set_iunknown_fn
            #inner_iunknown_fns
            #get_class_object_fn
        }
    )
}

/// Function used by in-process DLL macro to get an instance of the
/// class object.
fn gen_get_class_object_fn(struct_item: &ItemStruct) -> HelperTokenStream {
    let real_ident = macro_utils::get_real_ident(&struct_item.ident);
    let class_factory_ident = macro_utils::get_class_factory_ident(&real_ident);

    quote!(
        pub fn get_class_object() -> Box<#class_factory_ident> {
            <#class_factory_ident>::new()
        }
    )
}

/// Function that should only be used by Class Object, to set the
/// object's iunk_to_use, if the object is going to get aggregated.
fn gen_set_iunknown_fn() -> HelperTokenStream {
    let iunk_to_use_field_ident = macro_utils::get_iunk_to_use_field_ident();
    let non_del_unk_field_ident = macro_utils::get_non_del_unk_field_ident();

    quote!(
        pub(crate) fn set_iunknown(&mut self, aggr: *mut <dyn com::IUnknown as com::ComInterface>::VPtr) {
            if aggr.is_null() {
                self.#iunk_to_use_field_ident = &self.#non_del_unk_field_ident as *const _ as *mut <dyn com::IUnknown as com::ComInterface>::VPtr;
            } else {
                self.#iunk_to_use_field_ident = aggr;
            }
        }
    )
}

/// The non-delegating IUnknown implementation for an aggregable object. This will contain
/// the actual IUnknown implementations for the object.
fn gen_inner_iunknown_fns(
    base_itf_idents: &[Ident],
    aggr_itf_idents: &HashMap<Ident, Vec<Ident>>,
    struct_item: &ItemStruct,
) -> HelperTokenStream {
    let real_ident = macro_utils::get_real_ident(&struct_item.ident);
    let ref_count_ident = macro_utils::get_ref_count_ident();
    let inner_query_interface = gen_inner_query_interface(base_itf_idents, aggr_itf_idents);

    quote!(
        #inner_query_interface

        pub(crate) fn inner_add_ref(&mut self) -> u32 {
            self.#ref_count_ident += 1;
            println!("Count now {}", self.#ref_count_ident);
            self.#ref_count_ident
        }

        pub(crate) fn inner_release(&mut self) -> u32 {
            self.#ref_count_ident -= 1;
            println!("Count now {}", self.#ref_count_ident);
            let count = self.#ref_count_ident;
            if count == 0 {
                println!("Count is 0 for {}. Freeing memory...", stringify!(#real_ident));
                // drop(self)
                unsafe { Box::from_raw(self as *const _ as *mut #real_ident); }
            }
            count
        }
    )
}

/// Non-delegating query interface
fn gen_inner_query_interface(
    base_itf_idents: &[Ident],
    aggr_itf_idents: &HashMap<Ident, Vec<Ident>>,
) -> HelperTokenStream {
    let non_del_unk_field_ident = macro_utils::get_non_del_unk_field_ident();

    // Generate match arms for implemented interfaces
    let match_arms = base_itf_idents.iter().map(|base| {
        let match_condition =
            quote!(<dyn #base as com::ComInterface>::iid_in_inheritance_chain(riid));
        let vptr_field_ident = macro_utils::get_vptr_field_ident(&base);

        quote!(
            else if #match_condition {
                *ppv = &self.#vptr_field_ident as *const _ as *mut winapi::ctypes::c_void;
            }
        )
    });

    // Generate match arms for aggregated interfaces
    let aggr_match_arms = aggr_itf_idents.iter().map(|(aggr_field_ident, aggr_base_itf_idents)| {

        // Construct the OR match conditions for a single aggregated object.
        let first_base_itf_ident = &aggr_base_itf_idents[0];
        let first_aggr_match_condition = quote!(
            <dyn #first_base_itf_ident as com::ComInterface>::iid_in_inheritance_chain(riid)
        );
        let rem_aggr_match_conditions = aggr_base_itf_idents.iter().skip(1).map(|base| {
            quote!(|| <dyn #base as com::ComInterface>::iid_in_inheritance_chain(riid))
        });

        quote!(
            else if #first_aggr_match_condition #(#rem_aggr_match_conditions)* {
                let mut aggr_itf_ptr: ComPtr<dyn com::IUnknown> = ComPtr::new(self.#aggr_field_ident as *mut winapi::ctypes::c_void);
                let hr = aggr_itf_ptr.query_interface(riid, ppv);
                if com::failed(hr) {
                    return winapi::shared::winerror::E_NOINTERFACE;
                }

                // We release it as the previous call add_ref-ed the inner object.
                // The intention is to transfer reference counting logic to the
                // outer object.
                aggr_itf_ptr.release();

                core::mem::forget(aggr_itf_ptr);
            }
        )
    });

    quote!(
        pub(crate) fn inner_query_interface(&mut self, riid: *const winapi::shared::guiddef::IID, ppv: *mut *mut winapi::ctypes::c_void) -> HRESULT {
            println!("Non delegating QI");

            unsafe {
                let riid = &*riid;

                if winapi::shared::guiddef::IsEqualGUID(riid, &com::IID_IUNKNOWN) {
                    *ppv = &self.#non_del_unk_field_ident as *const _ as *mut winapi::ctypes::c_void;
                } #(#match_arms)* #(#aggr_match_arms)* else {
                    *ppv = std::ptr::null_mut::<winapi::ctypes::c_void>();
                    println!("Returning NO INTERFACE.");
                    return winapi::shared::winerror::E_NOINTERFACE;
                }

                println!("Successful!.");
                self.inner_add_ref();
                NOERROR
            }
        }
    )
}

/// For an aggregable object, we have to do more work here. We need to
/// instantiate the non-delegating IUnknown vtable. The unsafe extern "stdcall"
/// methods belonging to the non-delegating IUnknown vtable are also defined here.
fn gen_allocate_fn(base_itf_idents: &[Ident], struct_item: &ItemStruct) -> HelperTokenStream {
    let init_ident = &struct_item.ident;
    let real_ident = macro_utils::get_real_ident(&struct_item.ident);

    let mut offset_count: usize = 0;
    let base_inits = base_itf_idents.iter().map(|base| {
        let vtable_var_ident = quote::format_ident!("{}_vtable", base.to_string().to_lowercase());
        let vptr_field_ident = macro_utils::get_vptr_field_ident(&base);

        let out = quote!(
            let #vtable_var_ident = com::vtable!(#real_ident: #base, #offset_count);
            let #vptr_field_ident = Box::into_raw(Box::new(#vtable_var_ident));
        );

        offset_count += 1;
        out
    });
    let base_fields = base_itf_idents.iter().map(|base| {
        let vptr_field_ident = macro_utils::get_vptr_field_ident(base);
        quote!(#vptr_field_ident)
    });
    let ref_count_ident = macro_utils::get_ref_count_ident();
    let inner_init_field_ident = macro_utils::get_inner_init_field_ident();
    let iunk_to_use_field_ident = macro_utils::get_iunk_to_use_field_ident();
    let non_del_unk_field_ident = macro_utils::get_non_del_unk_field_ident();
    let non_del_unk_offset = base_itf_idents.len();

    quote!(
        fn allocate(init_struct: #init_ident) -> Box<#real_ident> {
            println!("Allocating new VTable for {}", stringify!(#real_ident));

            // Non-delegating methods.
            unsafe extern "stdcall" fn non_delegating_query_interface(
                this: *mut <dyn com::IUnknown as com::ComInterface>::VPtr,
                riid: *const winapi::shared::guiddef::IID,
                ppv: *mut *mut winapi::ctypes::c_void,
            ) -> HRESULT {
                let this = this.sub(#non_del_unk_offset) as *mut #real_ident;
                (*this).inner_query_interface(riid, ppv)
            }

            unsafe extern "stdcall" fn non_delegating_add_ref(
                this: *mut <dyn com::IUnknown as com::ComInterface>::VPtr,
            ) -> u32 {
                let this = this.sub(#non_del_unk_offset) as *mut #real_ident;
                (*this).inner_add_ref()
            }

            unsafe extern "stdcall" fn non_delegating_release(
                this: *mut <dyn com::IUnknown as com::ComInterface>::VPtr,
            ) -> u32 {
                let this = this.sub(#non_del_unk_offset) as *mut #real_ident;
                (*this).inner_release()
            }

            // Rust Parser limitation? Unable to construct associated type directly.
            type __iunknown_vtable_type = <dyn com::IUnknown as com::ComInterface>::VTable;
            let __non_del_unk_vtable =  __iunknown_vtable_type {
                QueryInterface: non_delegating_query_interface,
                Release: non_delegating_release,
                AddRef: non_delegating_add_ref,
            };
            let #non_del_unk_field_ident = Box::into_raw(Box::new(__non_del_unk_vtable));

            #(#base_inits)*
            let out = #real_ident {
                #(#base_fields,)*
                #non_del_unk_field_ident,
                #iunk_to_use_field_ident: std::ptr::null_mut::<<dyn com::IUnknown as com::ComInterface>::VPtr>(),
                #ref_count_ident: 0,
                #inner_init_field_ident: init_struct
            };
            Box::new(out)
        }
    )
}
