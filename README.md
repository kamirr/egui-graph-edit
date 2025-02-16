# Egui Graph Edit
> Fork of [egui_node_graph](https://github.com/setzer22/egui_node_graph)

[![Latest version](https://img.shields.io/crates/v/egui-graph-edit.svg)](https://crates.io/crates/egui-graph-edit)
[![Documentation](https://docs.rs/egui-graph-edit/badge.svg)](https://docs.rs/egui-graph-edit)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

![Showcase image](https://raw.githubusercontent.com/kamirr/egui-graph-edit/main/showcase.png)

**Egui Graph Edit** is a featureful, customizable library to create node graph
applications using [egui](https://github.com/emilk/egui). The library takes care
of presenting a node graph to your users, and allows customizing many aspects of
the interaction, creating the semantics you want for your specific application.

## Features and goals
This crate is meant to be a solid base for anyone wanting to expose a node graph
interface to their users. Its main design goal is to be completely agnostic to
the semantics of the graph, be it game logic, audio production, dialog trees,
shader generators... we have you covered!

The purpose of this library is to draw your graphs and handle the common user
interaction, like adding new nodes, moving nodes or creating connections. All
the additional functionality is provided by the user by means of custom user
types implementing several traits.

## Usage
To see a node graph in action, simply clone this repository and launch the
example using `cargo run`. This should open a window with an empty canvas. Right
clicking anywhere on the screen will bring up the *node finder* menu.

The [application code in the example](https://github.com/kamirr/egui-graph-edit/blob/main/egui-graph-edit-example/src/app.rs)
is thoroughly commented and serves as a good introduction to embedding this
library in your egui project. There is additional, [simpler example](https://github.com/kamirr/egui-graph-edit/blob/main/egui-graph-edit-example-simple/src/app.rs).

## A note on API visibility
Contrary to the general tendency in the Rust ecosytem, this library exposes all
types and fields that may be remotely relevant to a user as public. This is done
with the intent to be as flexible as possible, so no implementation details are
hidden from users who wish to tinker with the internals. Note that this crate
forbids use of `unsafe` so there is no risk of introducing UB by breaking any of
its invariants.

That being said, for the most typical use cases, you will want to stick to the
customization options this crate provides for you: The generic types in the
`GraphEditorState` object and their associated traits are the main API, all of
the other types and fields in this crate should be considered an implementation
detail. The example project contains a detailed explanation of all the
customization options and how are users supposed to interact with this crate.

Finally, this does not change the fact that this crate follows semantic
versioning, as is usual in the Rust ecosystem. Any change to a public field is
still considered breaking.

## Use cases

[Blackjack](https://github.com/setzer22/blackjack) is a 3d procedural modelling
software built in Rust using egui, rend3 and wgpu.

[Modal](https://github.com/kamirr/modal) is a modular sound synthesiser built
using **Egui Graph Edit** and a ton of math.

Are you using this crate for something cool? Add yourself to this section by
sending a PR!

## Contributing 
Contributions are welcome! Before writing a PR, please get in touch by filing an issue 😄
