# Aggregate

Aggregate attributes of structs to be used at runtime using a simple macro.

## Disclaimer

Aggregate is an extremely young library.

It is not fully implemented, and may face many breaking changes.

Contributions are welcomed, especially those made by people more experienced
in metaprogramming than I am.

## Features

By default, all features are enabled.

- `derive` re-exports `aggregate_derive`.

- `debug` implements `fmt::Debug` for `aggregate_types`.

- `serde` implements serde serialization for `aggregate_types`.
  This depends on the `debug` feature.

## Known limitations

- Enums cannot be aggregated yet. Support is planned.

- Native types like `Option<T>` cannot be aggregated yet. Support is planned.

- Unions cannot be aggregated. Support might happen, but is not prioritized.
