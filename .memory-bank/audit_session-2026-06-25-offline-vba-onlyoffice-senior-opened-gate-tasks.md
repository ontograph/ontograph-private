# Offline VBA To ONLYOFFICE Senior Opened Gate Tasks

Date: 2026-06-25

## Scope

Open a new bounded senior queue after the micro-task closure.

## Decision

Open only three gate-definition tasks for the remaining recurring blocked families:

- palette-index formatting
- row-dimension formatting
- dynamic formula rewriting

## Guardrails

- no implementation slice is reopened by this note
- no parser dependency work is opened
- no runtime ONLYOFFICE validation is opened
- no public `excel.translate` work is opened
- no OntoIndex refresh is scheduled from this note
