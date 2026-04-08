# 005: Seed Files in English Only

**Date**: 2026-04-08

## Decision

Seed files are written in English. No locale option, no bilingual format.

## Why

English is the lingua franca of open source and the language AI coding tools parse most reliably. The seed is planted into the user's project (not anima's own documentation), so anima's bilingual convention (English + Chinese translations) does not apply. Keeping the seed English-only avoids complexity in the CLI tool and ensures maximum compatibility across AI tools and developer ecosystems.

## Alternatives considered

- **Locale parameter** (`anima init --lang zh`). Adds CLI complexity and requires maintaining translated seed content. Marginal benefit — developers working with AI coding tools overwhelmingly operate in English-language toolchains.
- **Bilingual like anima's own docs.** The seed is not anima documentation — it becomes part of the user's project. Imposing bilingual format on user projects is overreach.
