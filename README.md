# bevy_sokoban

This is Sokoban implemented by following [@iolivia](https://github.com/iolivia)'s [Rust sokoban](https://sokoban.iolivia.me) tutorial — but in [Bevy](https://bevyengine.org/) instead of [ggez](https://ggez.rs/).

This is literally my first experience in gamedev, so the code is probably not very idiomatic and/or clean — all feedback is very much appreciated!

[This line](https://github.com/ropewalker/bevy_sokoban/blob/master/src/systems/setup.rs#L19) is supposed to enable UI Camera, which would allow to display text labels with game state and number of turns. It is commented out because of https://github.com/bevyengine/bevy/issues/134 (otherwise the game would crash with ```thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: MismatchedDynamicOffsetCount { actual: 0, expected: 2 }'``` on launch). Consider uncommenting the line when the issue is fixed.

Cargo.toml currently points at Bevy v0.2.0. Feel free to try and update it to the latest version available, but please keep in mind that at the moment of writing Bevy is still at the very early stages of development. Please consider [contributing](https://bevyengine.org/learn/book/contributing/) or [donating](https://github.com/sponsors/cart) :)
