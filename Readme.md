
# Frc Utils

This is a collection of utilities for projects in the FRC for rust ecosystem.
The frc-util crate is simply re-exports of other crates found in this repository.
All crates in this repository must only depend on crates outside of FRC to prevent circular dependencies.

## Crates

### Units

A custom rolled unit library support SI units and imperial units built with custom macros.
Is currently not user-extensible, but that is planned for the future.
The current implementation has more dynamic dispatch than an alternative like UOM, but is also more ergonomic and optimizations are planned for the future.

### Values

A module centering around a FrcValue type that is an enum union of all the type variants that are typically
supported in external tools like Datalog and Shuffleboard along with an FrcStructure implementation for more efficient
for over-wire transmission of arbitrary data structures.

### Time

Allows for swapping time implementations from a default implementation that uses the system clock to alternatives.
Takes advantage of ctor to allow for easy swapping of implementations in life before main from any crate in the dependency graph.
Is partially unsafe and should not be used in FRC-usercode.

### Logging

Is a WIP but for now has result and option extensions for logging errors and warnings.

clippy::many_single_char_names,
clippy::get_unwrap,
clippy::unwrap_in_result,
clippy::unwrap_used,
clippy::panicking_unwrap,