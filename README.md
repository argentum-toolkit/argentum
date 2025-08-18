Argentum ToolKit
================

<!-- TODO: toc -->

# Why "Argentum"?

Argentum ToolKit was made to provide the easiest way to implement microservices and back-end applications on Rust programming language.
Our goal is to allow everyone to create applications without deep knowledge of Rust.
Of course knowledge of the Rust will be great advantage.
But Argentum ToolKit helps you make your first steps as easy as possible and will save much time in the future.
Your learning process will be iterative and easier than without Argentum ToolKit.

:Run Argentum Demo Application; :Read Basic Rust Documentation; :Create Your new Argentum component or application; :Read more Rust Documentation; :Now you can quickly create RESTful applications with Argentum;

<!-- style:AdmonitionTip -->
> ***Getting Started with Argentum ToolKit***.
> You can try Argentum ToolKit with Docker without deep diving into Rust or the architecture of Argentum ToolKit.
> It is easy with link:docs/quick-start.adoc[Quick Start Guide]

# Documentation

## Argentum ToolKit Short Book

Currently, this book is not released yet, but it can help you learn a couple of features by example.

link:https://gitlab.com/argentum-toolkit/argentum-short-book/[Short Book]

## Components documentation

- 
- 
- 
- 
- 
- 
- 
- 
- 

## Tools documentation

- 

## Argentum ToolKit Architecture. Short description

Argentum ToolKit project is composed of components.
Each component is usually split to several crates:

- business crate for abstractions
- infrastructure crate for details of implementation.
- OpenAPI documentation (Swagger)
- Generated API package

What is ***abstractions***?
Abstractions are a clean business model.
It is the main core of the project which responsible for only business logic.
A Business package should not depend on 3-rd party packages it is possible.
To follow this restriction you should use Dependency Inversion Principle.

What is ***details***?
Details are: database adapters, RESTful API, RPC API and other connections with world.

<!-- style:AdmonitionNote -->
> Of course, you are free to follow other project structures.
> However, we advise you to pay maximum attention to the business logic of your application and separate it from the infrastructure.


rectangle "Argentum ToolKit" {     component Encryption as e {         package "Encryption Business" as abstractions         package "Encryption Infrastructure" as details

        details --> abstractions     } }

<!-- style:AdmonitionWarning -->
> Package with business logic should be independent or should depend only on other business packages.


<!-- style:AdmonitionNote -->
> Infrastructure packages can depend on business packages, on other infrastructure packages or on 3-rd party packages.


<!-- style:AdmonitionWarning -->
> Circular dependencies are forbidden.


# Features

- Independent components
- Dependency Injection

# Contributing Guideline

If you want to contribute to Argentum ToolKit you just need to follow couple easy rules:

- Be sure that you code follow code standards
- Your code with business logic doesn&#8217;t depend on infrastructure code and 3-rd party code

## How to create new component

We prepared the guide how to link:docs/create-component.adoc[create new component].

## How to check code

```bash
$ devops/scripts/check.sh
```

```bash
$ devops/scripts/check-item.sh {{package-name}}
```

## Allowed 3-rd party packages for business packages of Argentum ToolKit

- **thiserror** - errors
- **chrono** - time

# Dioxus dependencies

As soon as Argentum ToolKit uses Dioxus, we need to have some dependencies such as `perl`

```shell
cargo install dioxus-cli
```

# TODO

## V0.3

- DI builders: make it similar
- behavior tests
- rest-generator
    - tooling
        - scripts to simplify generation of API library
        - publish generator to hub.docker.com
    - create regex factory for all types of **path params**
    - generate **query params**
    - ?generate inline objects
    - tests
    - fix generator for cases when string field parses as integer
- DB
    - ?Foreign key for session
- documentation
    - ADR
    - other?
- New type for Password
- run docker images as a local user
- OpenTelemetry
- notifications
- macro for creation Id types (for user, for account for event)

# License

- Versions prior to **0.3**: BSD 3-Clause License
- Version **0.3 and later**: GNU Lesser General Public License v3.0 or later (LGPL-3.0-or-later)

See link:LICENSE[LICENSE] for full details.

© 2020–2025 Vital Leshchyk <vitalleshchyk@gmail.com>

# Contributing

Please read the [Contributor License Agreement](CONTRIBUTOR_LICENSE_AGREEMENT.adoc) before contributing.

To contribute, you ***must sign the CLA*** by adding a file in the `cla-signatures/` directory as described in the CLA document.

Your pull request or merge request will be accepted only if a valid CLA signature file is present for your GitHub/GitLab username.

Thank you for your contribution!