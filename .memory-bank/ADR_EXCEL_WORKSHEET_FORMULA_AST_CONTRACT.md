# ADR: Excel Worksheet Formula AST Contract

## Status

Design-only. No parser, Rust type, tool, or fixture is implemented by this ADR.

## Date

2026-06-26

## Context

Current `ext/excel` owners already provide bounded worksheet formula inventory, cached values, number-format metadata, workbook calculation flags, defined-name metadata, and lexical risk warnings. That read-only surface is enough to inspect formulas, but it is not enough to support any future worksheet-formula planner work safely.

Rows `039` and `040` depend on a typed worksheet-formula representation. Donor evidence from `tmp/excel/in2sql_dotNet_addin` is consistent on one point: regex evidence may classify, but it must not become the basis for SQL generation. A typed AST with explicit unsupported-node handling is the minimum safe prerequisite.

This ADR exists to define that contract without opening implementation.

## Decision

If worksheet formula analysis is reopened, it must start with a bounded read-only AST contract inside the current offline `ext/excel` owner.

The first slice must satisfy all of these rules:

- owner stays inside `ontocode-rs/ext/excel`
- source is worksheet formula text already available from `xlsx` and `xlsm` worksheet XML
- first slice parses A1 formulas only
- the AST is read-only metadata
- no SQL generation
- no formula evaluation
- no workbook mutation
- no named-range rewrite
- no dependency-graph claims

## Required Parse Behavior

The parser contract must be deterministic and side-effect free.

- malformed formulas must return bounded diagnostics, not panic or crash
- recognized-but-unsupported constructs must become explicit unsupported nodes or blockers, not guessed nodes
- parse success must be distinguishable from partial or blocked parse results
- source text must remain bounded and preserved enough for diagnostics and fallback display

## Minimum Node Coverage For First Slice

The first slice may open only with this minimum node family:

- numeric literal
- string literal
- boolean literal
- blank argument slot
- error literal
- unary operation
- binary operation
- postfix percent
- function call
- cell reference
- range reference
- sheet-qualified reference
- defined-name reference

Structured references may be recognized only if they are represented as explicit unsupported nodes in the first slice.

## Explicit Unsupported Or Blocked Categories

The first slice must treat these as unsupported or blocked:

- array constants
- dynamic-array spill markers
- spill-capable dynamic-array functions
- external workbook references
- volatile functions
- locale-only alternate separators outside the accepted first-slice locale contract
- R1C1 parsing
- structured table references if not explicitly implemented

Unsupported categories must be represented directly in the parse result. They must not silently degrade to plain identifiers or plain function calls.

## Output Contract

Any future Rust-owned parse result must contain:

- bounded original formula text
- optional root node
- success flag or equivalent parse state
- bounded diagnostics
- bounded unsupported reasons
- truncation marker when any source or diagnostic sample is shortened

This ADR does not approve the concrete Rust names, only the required content.

## Fixture Gate Before Code

Before implementation opens, a fresh senior-review pass must approve:

1. concrete Rust-owned node and diagnostic types
2. parser-backed fixture strategy
3. first-slice caps for formula length, diagnostic count, and unsupported sample count

The first synthetic fixture set must prove:

- operator precedence
- unary versus exponent behavior
- comparison and concatenation precedence
- sheet-qualified references
- ranges
- defined-name references
- malformed formula diagnostics
- explicit unsupported handling for array constants, dynamic arrays, external references, and volatile functions

## Non-Goals

This ADR does not approve:

- SQL planning or SQL emission
- cached-value validation logic
- graph extraction
- workbook mutation
- named-range rewrite
- live Excel / `Formula2`
- `.xls` support
- large-workbook budget policy changes

## Senior Challenge Outcome

The correct lazy path is to stop at the AST contract. It is the smallest slice that can later unblock rows `039` and `040` without reopening mutation, live Excel, or graph work prematurely.
