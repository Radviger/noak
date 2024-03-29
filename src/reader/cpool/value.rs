use crate::error::*;
use crate::mutf8::MStr;
use crate::reader::cpool::{self, ConstantPool, Index};

pub trait ToValue<'input> {
    type Target;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Class<'input> {
    pub name: &'input MStr,
}

impl<'input> ToValue<'input> for Index<cpool::Class<'input>> {
    type Target = Class<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(Class {
            name: pool.retrieve(this.name)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldRef<'input> {
    pub class: Class<'input>,
    pub name_and_type: NameAndType<'input>,
}

impl<'input> ToValue<'input> for Index<cpool::FieldRef<'input>> {
    type Target = FieldRef<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(FieldRef {
            class: pool.retrieve(this.class)?,
            name_and_type: pool.retrieve(this.name_and_type)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MethodRef<'input> {
    pub class: Class<'input>,
    pub name_and_type: NameAndType<'input>,
}

impl<'input> ToValue<'input> for Index<cpool::MethodRef<'input>> {
    type Target = MethodRef<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(MethodRef {
            class: pool.retrieve(this.class)?,
            name_and_type: pool.retrieve(this.name_and_type)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InterfaceMethodRef<'input> {
    pub class: Class<'input>,
    pub name_and_type: NameAndType<'input>,
}

impl<'input> ToValue<'input> for Index<cpool::InterfaceMethodRef<'input>> {
    type Target = InterfaceMethodRef<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(InterfaceMethodRef {
            class: pool.retrieve(this.class)?,
            name_and_type: pool.retrieve(this.name_and_type)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct String<'input> {
    pub string: &'input MStr,
}

impl<'input> ToValue<'input> for Index<cpool::String<'input>> {
    type Target = String<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(String {
            string: pool.retrieve(this.string)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Integer {
    pub value: i32,
}

impl<'input> ToValue<'input> for Index<cpool::Integer> {
    type Target = Integer;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(Integer { value: this.value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Long {
    pub value: i64,
}

impl<'input> ToValue<'input> for Index<cpool::Long> {
    type Target = Long;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(Long { value: this.value })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Float {
    pub value: f32,
}

impl<'input> ToValue<'input> for Index<cpool::Float> {
    type Target = Float;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(Float { value: this.value })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Double {
    pub value: f64,
}

impl<'input> ToValue<'input> for Index<cpool::Double> {
    type Target = Double;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(Double { value: this.value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameAndType<'input> {
    pub name: &'input MStr,
    pub descriptor: &'input MStr,
}

impl<'input> ToValue<'input> for Index<cpool::NameAndType<'input>> {
    type Target = NameAndType<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(NameAndType {
            name: pool.retrieve(this.name)?,
            descriptor: pool.retrieve(this.descriptor)?,
        })
    }
}

impl<'input> ToValue<'input> for Index<cpool::Utf8<'input>> {
    type Target = &'input MStr;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(this.content)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MethodHandle<'input> {
    pub kind: cpool::MethodKind,
    pub reference: cpool::Item<'input>,
}

impl<'input> ToValue<'input> for Index<cpool::MethodHandle<'input>> {
    type Target = MethodHandle<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(MethodHandle {
            kind: this.kind,
            reference: pool.get(this.reference)?.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MethodType<'input> {
    pub descriptor: &'input MStr,
}

impl<'input> ToValue<'input> for Index<cpool::MethodType<'input>> {
    type Target = MethodType<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(MethodType {
            descriptor: pool.retrieve(this.descriptor)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dynamic<'input> {
    // actually an index into the bootstrap method table
    pub bootstrap_method_attr: u16,
    pub name_and_type: NameAndType<'input>,
}

impl<'input> ToValue<'input> for Index<cpool::Dynamic<'input>> {
    type Target = Dynamic<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(Dynamic {
            bootstrap_method_attr: this.bootstrap_method_attr,
            name_and_type: pool.retrieve(this.name_and_type)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvokeDynamic<'input> {
    // actually an index into the bootstrap method table
    pub bootstrap_method_attr: u16,
    pub name_and_type: NameAndType<'input>,
}

impl<'input> ToValue<'input> for Index<cpool::InvokeDynamic<'input>> {
    type Target = InvokeDynamic<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(InvokeDynamic {
            bootstrap_method_attr: this.bootstrap_method_attr,
            name_and_type: pool.retrieve(this.name_and_type)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Module<'input> {
    pub name: &'input MStr,
}

impl<'input> ToValue<'input> for Index<cpool::Module<'input>> {
    type Target = Module<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(Module {
            name: pool.retrieve(this.name)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Package<'input> {
    pub name: &'input MStr,
}

impl<'input> ToValue<'input> for Index<cpool::Package<'input>> {
    type Target = Package<'input>;

    fn retrieve_from(self, pool: &ConstantPool<'input>) -> Result<Self::Target, DecodeError> {
        let this = pool.get(self)?;
        Ok(Package {
            name: pool.retrieve(this.name)?,
        })
    }
}
