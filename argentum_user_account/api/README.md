# Rust API for argentum_user_account_api

This is demo app

## Overview

This client/server was generated by the [openapi-generator]
(https://openapi-generator.tech) project.  By using the
[OpenAPI-Spec](https://github.com/OAI/OpenAPI-Specification) from a remote
server, you can easily generate a server stub.

To see how to make this your own, look here:

[README]((https://openapi-generator.tech))

- API version: 0.1.0-dev
- Build date: 2023-06-08T00:31:21.547097Z[Etc/UTC]

For more information, please visit [https://gitlab.com/argentum-toolkit/argentum](https://gitlab.com/argentum-toolkit/argentum)

This autogenerated project defines an API crate `argentum_user_account_api` which contains:
* An `Api` trait defining the API in Rust.
* Data types representing the underlying data model.
* A `Client` type which implements `Api` and issues HTTP requests for each operation.
* A router which accepts HTTP requests and invokes the appropriate `Api` method for each operation.

It also contains an example server and client which make use of `argentum_user_account_api`:

* The example server starts up a web server using the `argentum_user_account_api`
    router, and supplies a trivial implementation of `Api` which returns failure
    for every operation.
* The example client provides a CLI which lets you invoke
    any single operation on the `argentum_user_account_api` client by passing appropriate
    arguments on the command line.

You can use the example server and client as a basis for your own code.
See below for [more detail on implementing a server](#writing-a-server).

## Examples

Run examples with:

```
cargo run --example <example-name>
```

To pass in arguments to the examples, put them after `--`, for example:

```
cargo run --example client -- --help
```

### Running the example server
To run the server, follow these simple steps:

```
cargo run --example server
```

### Running the example client
To run a client, follow one of the following simple steps:

```
cargo run --example client AnonymousRegisters
```

### HTTPS
The examples can be run in HTTPS mode by passing in the flag `--https`, for example:

```
cargo run --example server -- --https
```

This will use the keys/certificates from the examples directory. Note that the
server chain is signed with `CN=localhost`.

## Using the generated library

The generated library has a few optional features that can be activated through Cargo.

* `server`
    * This defaults to enabled and creates the basic skeleton of a server implementation based on hyper
    * To create the server stack you'll need to provide an implementation of the API trait to provide the server function.
* `client`
    * This defaults to enabled and creates the basic skeleton of a client implementation based on hyper
    * The constructed client implements the API trait by making remote API call.
* `conversions`
    * This defaults to disabled and creates extra derives on models to allow "transmogrification" between objects of structurally similar types.

See https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section for how to use features in your `Cargo.toml`.

## Documentation for API Endpoints

All URIs are relative to *http://localhost:8082/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**anonymousRegisters**](docs/anonymous_api.md#anonymousRegisters) | **POST** /user/anonymous-register | Anonymous registers
[**changePasswordWithToken**](docs/user_account_api.md#changePasswordWithToken) | **POST** /user/restore-password/change-password | User with token changes his password
[**loginWithPassword**](docs/user_account_api.md#loginWithPassword) | **POST** /user/password-login | Login as an user
[**registerWithPassword**](docs/user_account_api.md#registerWithPassword) | **POST** /user/register | User registers with password
[**requestRestoreToken**](docs/user_account_api.md#requestRestoreToken) | **POST** /user/restore-password/token-request | Anonymous requests restore password token


## Documentation For Models

 - [AnonymousRegistrationResult](docs/AnonymousRegistrationResult.md)
 - [ChangePasswordSchema](docs/ChangePasswordSchema.md)
 - [EmptyResponse](docs/EmptyResponse.md)
 - [LoginResult](docs/LoginResult.md)
 - [LoginWithPasswordSchema](docs/LoginWithPasswordSchema.md)
 - [ProblemDetail](docs/ProblemDetail.md)
 - [RegistrationWithPasswordResult](docs/RegistrationWithPasswordResult.md)
 - [RegistrationWithPasswordSchema](docs/RegistrationWithPasswordSchema.md)
 - [RequestRestoreTokenSchema](docs/RequestRestoreTokenSchema.md)
 - [UserName](docs/UserName.md)


## Documentation For Authorization

Authentication schemes defined for the API:
### bearerAuth
- **Type**: Bearer token authentication

Example
```
```

## Author



