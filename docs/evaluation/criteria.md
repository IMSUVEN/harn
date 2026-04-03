# Evaluation Criteria

These criteria define what "good" means for harn. When evaluating work — whether self-evaluating or reviewing someone else's output — grade against each criterion independently.

## Grading Scale

| Grade | Meaning |
|-------|---------|
| A | Excellent — exceeds expectations, no issues |
| B | Good — meets expectations, minor issues only |
| C | Acceptable — meets minimum bar, notable gaps |
| D | Below bar — significant issues that need addressing |
| F | Failing — does not meet requirements |

**Pass threshold**: Every criterion must be C or above for the work to pass.

---

## Criteria

### 1. Functionality (Weight: High)

Does the implementation do what the spec says? Do core workflows work end-to-end?

- **A**: All specified features work correctly. Edge cases handled. Error paths tested.
- **B**: Core features work. Minor edge cases may be unhandled.
- **C**: Core features work with caveats. Some features may be partial.
- **D**: Key features are broken or incomplete.
- **F**: Primary functionality does not work.

### 2. Product Depth (Weight: High)

Are features fully implemented with real logic, or are they stubs and placeholders?

- **A**: Features are fully implemented with real logic, proper validation, and meaningful output.
- **B**: Features work with minor gaps in depth.
- **C**: Some features are skeletal or stub-only.
- **D**: Multiple features are stubs or placeholders.
- **F**: Most features are non-functional facades.

### 3. Code Quality (Weight: Medium)

Is the code maintainable, well-structured, and consistent with architectural constraints?

- **A**: Clean architecture, comprehensive error handling with context chains, consistent patterns, typed domain models, tested. `clippy` and `rustfmt` clean.
- **B**: Generally clean. Minor inconsistencies. Error handling present but occasionally lacks context.
- **C**: Functional but with structural issues (raw `String` where newtypes belong, inconsistent error handling, missing boundary validation).
- **D**: Significant structural problems. Dependency direction violations. Hard to maintain.
- **F**: Spaghetti code. Architectural rules violated. No type safety at boundaries.

### 4. API Ergonomics (Weight: Medium)

Is the CLI interface intuitive, well-documented, and hard to misuse?

- **A**: Intuitive command structure, clear error messages with remediation, discoverable `--help` output, consistent flag naming, sensible defaults. Output is scannable and informative.
- **B**: Generally clean interface. Minor discoverability gaps. Error messages present but occasionally vague.
- **C**: Functional but requires reading docs to understand usage. Error messages say what failed but not what to do.
- **D**: Confusing subcommand structure, unhelpful errors, undocumented behavior, inconsistent flag conventions.
- **F**: Unusable without reading source code.

### 5. Originality (Weight: Low)

Are there deliberate design choices, or is this all defaults and boilerplate?

- **A**: Distinctive choices that serve the product goals (e.g., detection heuristics, template rendering approach, deferred generation strategy).
- **B**: Some deliberate choices visible.
- **C**: Mostly defaults, but appropriate ones.
- **D**: Generic boilerplate with no project-specific adaptation.
- **F**: No evidence of intentional decision-making.

---

## How to Use These Criteria

**For the agent building the feature**: Read these criteria before starting. Target B or above on every criterion. Self-evaluate your work against these before handing off.

**For the reviewing agent or human**: Grade each criterion independently. If any criterion is D or below, the work should be revised with specific feedback on what needs to change.
