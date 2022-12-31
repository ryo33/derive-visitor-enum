pub(crate) struct DataStructure {
    pub(crate) name: syn::Ident,
    pub(crate) variants: Vec<Variant>,
}

pub(crate) struct Variant {
    pub(crate) name: syn::Ident,
    pub(crate) return_type: syn::Ident,
    pub(crate) fields: Fields,
}

pub(crate) enum Fields {
    Struct(Vec<NamedField>),
    Tuple(Vec<Field>),
}

pub(crate) struct Field {
    pub(crate) name: syn::Ident,
    pub(crate) ty: Type,
    pub(crate) visitor: FieldVisitor,
}

pub(crate) enum Type {
    /// `Self`, `Box<Self>`, `Rc<Self>`, or `Arc<Self>`
    This,
    Vec(Box<Type>),
    Option(Box<Type>),
    Other(syn::Type),
}

pub(crate) enum FieldVisitor {
    None,
}

pub(crate) struct NamedField {
    pub(crate) name: syn::Ident,
    pub(crate) field: Field,
}

pub(crate) fn extract(input: syn::DeriveInput) -> DataStructure {
    todo!()
}
