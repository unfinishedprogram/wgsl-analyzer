use crate::{
    diagnostic::Diagnostic,
    module::{declaration::r#type::Type, store::handle::Handle, type_store::TypeStore},
};

pub enum Overload {
    Simple {
        arg_types: Vec<Handle<Type>>,
        return_type: Option<Handle<Type>>,
    },
}

pub struct Builtin {
    pub ident: &'static str,
    pub overloads: Vec<Overload>,
}

impl Builtin {
    pub fn get_all_builtin_functions(type_store: &TypeStore) -> Result<Vec<Builtin>, Diagnostic> {
        let h_bool = type_store.get_raw_ident_type("bool")?;
        let h_i32 = type_store.get_raw_ident_type("i32")?;
        let h_u32 = type_store.get_raw_ident_type("u32")?;
        let h_f32 = type_store.get_raw_ident_type("f32")?;
        let h_f16 = type_store.get_raw_ident_type("f16")?;

        Ok(vec![
            Builtin::new("bool").overload(vec![], Some(h_bool)),
            Builtin::new("i32").overload(vec![], Some(h_i32)),
            Builtin::new("u32").overload(vec![], Some(h_u32)),
            Builtin::new("f32").overload(vec![], Some(h_f32)),
            Builtin::new("f16").overload(vec![], Some(h_f16)),
        ])
    }
}

impl Builtin {
    pub fn new(ident: &'static str) -> Self {
        Self {
            ident,
            overloads: vec![],
        }
    }

    pub fn overload(
        mut self,
        arguments: Vec<Handle<Type>>,
        return_type: Option<Handle<Type>>,
    ) -> Self {
        self.overloads.push(Overload::Simple {
            arg_types: arguments,
            return_type,
        });

        self
    }
}
