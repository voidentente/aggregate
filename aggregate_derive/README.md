# Aggregate

Aggregate attributes of structs to be used at runtime using a simple macro.

## Features

By default, all features are enabled.

- `derive` re-exports `aggregate_derive`.

- `debug` implements `Debug` for `aggregate_types`.

- `helper` implements ease-of-life traits for `aggregate_types` newtypes.

## Usage

`aggregate` is extremely simple to use.

In order to aggregate attributes of your struct/enum/union, simply derive it:

```rs
// The prelude is not required for derive,
// however in order to use `aggregate`,
// the trait must be in scope
use aggregate::prelude::*;

/// This attribute is paired with the type
#[derive(Aggregate)]
struct Config {
  /// This attribute is paired with the field
  switch: bool,
}
```

Aggregate supports nesting:

```rs
/// This attribute is paired with the type.
#[derive(Aggregate)]
struct ConfigFile {
  /// This attribute is paired with the field
  ///
  /// This field has an `inner`, which will include the aggregation of `Config`
  ///
  /// In order for `aggregate_derive` to notice nested structures, you must
  /// mark the field with the `#[aggregate]` attribute:
  #[aggregate]
  my_config: Config,
}
```

The `#[aggregate]` attribute is not required on enum variants,
but is again required inside enum variant structs and tuples:

```rs
#[derive(Aggregate)]
enum MyEnum {
  /// Variants are automatically included
  VariantOne {
    /// Fields must be marked
    #[aggregate]
    field_1: Inner,
  },

  /// Unnamed structs like this are also supported;
  /// `aggregate` simply enumerates the fields for representation
  VariantTwo(#[aggregate] Inner),
}
```
