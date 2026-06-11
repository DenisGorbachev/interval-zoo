# Project guidelines

## Interval type

A struct with two fields (`a`, `b`).

- Must derive `Eq, PartialEq, Hash, Clone, Copy, Debug`
- Must implement `Contains`
- Must implement `Overlaps`
- Must have `new_ordered` constructor that enforces `a <= b` by reordering.

## Relaxed interval type

An interval type that doesn't enforce `a <= b`.

- Must have `pub` fields
- Must have `normalize(&mut self)` method

## Strict interval type

An interval type that enforces `a <= b`.

- Must have private fields
- Must derive `Getters, Into`
- Must implement `TryFrom<(T, T)>` that enforces `a <= b` by returning an error with `OrderCheckFailed` variant

## Finite interval

An interval type with two [finite bounds](#finite-bound).

## Runtime bound

A bound whose value is determined at runtime.

## Comptime bound

A bound whose value is determined at compile-time.

## Finite bound

A bound that has an exact finite value.

## Non-finite bound

A bound that can be either an exact finite value or infinity.
