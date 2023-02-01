# Aggregate

Access attributes of structs, enums, unions, and their fields using a
simple Derive macro and without allocating.

## About

Aggregate utilizes the power of macros, perfect hash functions,
and lazy loading for extreme performance.

`Amalgamate`s are singletons representing a struct/enum/union's
attribute structure that lie in shared static memory and can be accessed via
a type function call.

Aggregate works recursively by calling `aggregate()` on nested structures
without limiting the types that can be used, and without runtime overhead
by simply linking to nested `Amalgamate`s by reference.

Attributes are kept intact, which means that on access, they are
represented as `syn::Attribute`s. However, parsing *all* attributes from tokens
can be costly and wasteful, which is why they are lazy-loaded and only parsed
when accessed.

## Features

By default, all features are enabled.

- `derive` re-exports `aggregate_derive`.

- `impl` adds default implementations for common types like `Option<T>`.

- `fmt` implements `fmt::Debug` for `Amalgamate`.

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
  /// This field has an `inner`, which will 
  /// include the aggregation of `Config`
  ///
  /// In order for `aggregate_derive` to notice 
  /// nested structures, you must mark the field 
  /// with the `#[aggregate]` attribute:
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

  /// Unnamed structs like this are also 
  /// supported; `aggregate` simply enumerates 
  /// the fields for representation
  VariantTwo(#[aggregate] Inner),
}
```
