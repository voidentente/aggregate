//! "Aggregate" attributes of structs and enums at compile-time
//! to make them available at runtime using a simple Derive macro.

#[cfg(feature = "derive")]
pub use aggregate_derive as derive;
pub use aggregate_types as types;
pub use syn;

pub mod prelude {
    #[cfg(feature = "derive")]
    pub use crate::derive::Aggregate;
    pub use crate::Aggregate;
}

/// This trait can be derived using the `aggregate_derive` crate,
/// which is included by default.
pub trait Aggregate {
    /// `aggregate` does three things:
    ///
    /// 1. Collect all attributes of the current TokenStream.
    /// 2. Call `aggregate` on nested structures.
    /// 3. Return the merged structure.
    ///
    /// To include nested structures, one must mark them using
    /// the `#[aggregate]` helper attribute.
    ///
    /// This is a more permissive alternative to calling `aggregate`
    /// on all fields, which will not work if any field does not
    /// implement `Aggregate`.
    fn aggregate() -> types::Amalgamate;
}

impl<T: Aggregate> Aggregate for Option<T> {
    fn aggregate() -> types::Amalgamate {
        T::aggregate()
    }
}

#[cfg(test)]
#[cfg(feature = "derive")]
mod tests {
    #![allow(dead_code)]

    use crate as aggregate;
    use aggregate::prelude::*;

    /// UnitStruct
    #[derive(Aggregate)]
    struct UnitStruct;

    /// TupleStruct
    #[derive(Aggregate)]
    struct TupleStruct(bool, #[aggregate] StructWithFields);

    /// StructWithFields
    #[derive(Aggregate)]
    struct StructWithFields {
        /// StructWithFields.field_1
        field_1: bool,
        /// StructWithFields.field_2
        #[aggregate]
        field_2: StructWithOption,
    }

    /// StructWithOption
    #[derive(Aggregate)]
    struct StructWithOption(#[aggregate] Option<UnitStruct>);

    /// UnitEnum
    #[derive(Aggregate)]
    enum UnitEnum {
        /// UnitEnum.Unit1
        Unit1,
        /// UnitEnum.Unit2
        Unit2,
    }

    /// EnumWithFieldVariants
    #[derive(Aggregate)]
    enum EnumWithFieldVariants {
        /// EnumWithFieldVariants.FieldOne
        FieldOne {
            /// EnumWithFieldVariants.FieldOne.field_1
            #[aggregate]
            field_1: UnitStruct,
        },
        /// EnumWithFieldVariants.FieldTwo
        FieldTwo {
            /// EnumWithFieldVariants.FieldTwo.field_2
            #[aggregate]
            field_2: StructWithOption,
        },
    }

    /// EnumWithTuples
    #[derive(Aggregate)]
    enum EnumWithTuples {
        /// EnumWithTuples.TupleOne
        TupleOne(#[aggregate] TupleStruct),
        /// EnumWithTuples.TupleTwo
        TupleTwo(bool),
    }

    /// MixedEnum
    #[derive(Aggregate)]
    enum MixedEnum {
        /// MixedEnum.Unit
        Unit,
        /// MixedEnum.Fields
        Fields {
            /// MixedEnum.Fields.field_1
            #[aggregate]
            field_1: Option<EnumWithTuples>,
        },
        /// MixedEnum.Tuples
        Tuples(#[aggregate] StructWithOption),
    }

    /// BasicUnion
    #[derive(Aggregate)]
    union BasicUnion {
        /// BasicUnion.field_1
        field_1: f32,
        /// BasicUnion.field_2
        field_2: f64,
    }

    #[test]
    fn struct_unit() {
        UnitStruct::aggregate();
    }

    #[test]
    fn struct_tuple() {
        TupleStruct::aggregate();
    }

    #[test]
    fn struct_fields() {
        StructWithFields::aggregate();
    }

    #[test]
    fn struct_optional() {
        StructWithOption::aggregate();
    }

    #[test]
    fn enum_unit() {
        UnitEnum::aggregate();
    }

    #[test]
    fn enum_fields() {
        EnumWithFieldVariants::aggregate();
    }

    #[test]
    fn enum_tuples() {
        EnumWithTuples::aggregate();
    }

    #[test]
    fn enum_mixed() {
        MixedEnum::aggregate();
    }

    #[test]
    fn union_basic() {
        BasicUnion::aggregate();
    }
}
