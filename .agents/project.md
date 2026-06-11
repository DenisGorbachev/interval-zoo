# Project guidelines

## Interval type

A struct with two fields of the same type.

- Must derive `Eq, PartialEq, Hash, Clone, Copy, Debug`
- Must implement `Contains`
- Must implement `Overlaps`
- Must implement `Length`
- Must have a doc comment that contains the following lines:
  - This type intentionally doesn't implement `Ord` or `PartialOrd`, because a single interval has multiple values that can be compared (for example: field values, length value). Users should compare the values directly.

## Relaxed interval type

An interval type that doesn't perform any validation.

- Must public fields `a` and `b`
- Must have `normalize(&mut self)` method
- Must have `new_ordered` constructor that enforces `a <= b` by reordering.

## Strict interval type

An interval type that enforces `a <= b`.

- Must private fields `lo` and `hi`
- Must derive `Getters, Into`
- Must implement `TryFrom<(T, T)>` that enforces `lo <= hi` by returning an error with `OrderCheckFailed` variant
- Must have `new_ordered` constructor that enforces `lo <= hi` by reordering.

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
