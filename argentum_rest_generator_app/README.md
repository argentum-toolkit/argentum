Argentum ToolKit. REST Generator.
=================================

REST Generator is a tools that creates API libraries from OpenAPI specification.

Supported generators

| Language | Type                    | Supported versions |
|----------|-------------------------|--------------------|
| Rust     | Server side API package | 3.1 and 3          |

# Key features

- Parsing instead of validation
- Generated code compatible with Argentum REST
- Clean and human-readable error responses
- Low code duplication in generated code
- Extendable and reusable generated code

# How to install

## Via cargo install

```shell
cargo install argentum_rest_generator_app
```

```shell
git clone git@gitlab.com:argentum-toolkit/argentum.git

```

```shell
cargo install --path argentum_rest_generator_app/

```

# How to use

```shell
argentum_rest_generator_app --input api-doc/openapi.yaml --output rest/ --package-name user_account_rest
```

Or you can generate via `cargo run` (sources should be cloned)

```shell
cd argentum/argentum_rest_generator_app

cargo run --   --input ../argentum_user_account/api-doc/openapi.yaml --output ../argentum_user_account/rest/ --package-name argentum_user_account_rest
```

# Restrictions

- supports only **$ref** params in case if OpenAPI objects supports **$ref**.
  (Doesn&#8217;t support inline objects in this case)
- Schemas supports only Object type (Array, AnyOf, AllOf are not implemented yet)
- we don&#8217;t use OpenAPI tags.
  If you need similar functionality, we would like to recommend split your API to packages