# Product Philosophy of anima

[English](PHILOSOPHY.md) | [中文](PHILOSOPHY.zh-CN.md)

> The [Harness Specification](HARNESS-SPEC.md) and [Harness Guide](HARNESS-GUIDE.md) describe harness engineering as a discipline — the theory of designing environments for AI coding agents. This document describes **anima's interpretation** of that theory: the specific stance anima takes on what a harness should be, who it serves, and how it comes into being.
>
> The underlying logic is unchanged. The three structural contradictions (intent transfer, self-evaluation, entropy) and three axioms (governance, strategy, mechanism) remain the foundation. What changes is the **perspective** from which they are applied.

---

## 1. The Cultivation Thesis

The spec and guide adopt a natural default perspective: the harness as **control system**. The agent is powerful but unreliable; the harness constrains, corrects, and channels. This perspective is defensible and internally consistent. It is not the only valid interpretation of the same axioms.

anima adopts a different perspective: the harness as **cultivation system**. The agent is not a dangerous tool to be caged. It is a nascent collaborator to be nurtured. The harness is not a set of walls; it is soil, light, and water — the conditions under which the agent-project system grows in capability over time.

This is not a softer version of the same idea. It produces different design decisions:

| Decision Point | Control Perspective | Cultivation Perspective |
|---|---|---|
| What is the agent? | Powerful but unreliable executor | Nascent but capable explorer |
| What is the harness? | Constraint apparatus | Growth environment |
| Why store knowledge in the repo? | Prevent agent context loss | Accumulate project understanding |
| Why run feedback loops? | Correct agent errors | Enable agent-project co-evolution |
| What does time do? | Each session is independent | Each cycle compounds the project's wisdom |
| What does a new project need? | Correct initial configuration | A viable seed and room to grow |

Both perspectives are derived from the same axioms. The spec manages the contradictions by **constraining** the agent. anima manages them by **cultivating** the environment. The contradictions don't go away — they are channeled into growth rather than suppressed by control.

### Why cultivation and not control

Three reasons, ordered by importance.

**Cultivation matches the actual dynamic of human-agent collaboration.** When an engineer starts a new project with an agent, the first hours are not execution — they are exploration. What should the architecture be? Which patterns fit? What conventions emerge naturally? The agent is a co-discoverer in this process. A control framework has no vocabulary for this phase; a cultivation framework treats it as the foundation everything else grows from.

**Cultivation produces harnesses that are uniquely adapted to their projects.** A control-oriented harness tends toward standardization: the same rules, the same structure, the same templates across projects. This is efficient but brittle — it encodes assumptions that may not hold in every context. A cultivation-oriented harness grows from the project's own practice, producing rules and conventions that reflect what actually matters in this specific codebase. The harness becomes a record of the project's lived experience.

**Cultivation aligns with how models will evolve.** As models grow more capable, control-oriented components (stricter constraints, more detailed instructions, tighter guardrails) become overhead. But growth-oriented components (knowledge accumulation, feedback loops, exploration support) become *more* valuable with stronger models, because a stronger model extracts more value from a richer environment. Cultivation bets on a future where the environment's richness matters more than the environment's restrictiveness.

---

## 2. The Three Axioms, Reinterpreted

The axioms are unchanged in substance. What changes is what they ask of the harness.

### Axiom 1: Humans Direct, Agents Explore

> Original formulation: "Humans steer, agents execute."

The governance principle is non-negotiable: humans hold authority over direction, boundaries, and quality criteria. anima does not soften this.

What anima reinterprets is the agent's role within those boundaries. "Execute" implies a predefined instruction set — the agent's job is faithful reproduction. "Explore" implies an open space — the agent's job is to discover and propose within the boundaries the human has set.

An exploring agent may say: "I've noticed this project has three different error-handling patterns. Should I converge them into one?" A pure executor would never surface this — it wasn't in the instructions. But it is exactly the kind of discovery that makes the project better and the agent more valuable over time.

The harness design implication: instead of optimizing for precise instruction → precise execution, optimize for **clear boundaries + rich environment → high-quality exploration**.

### Axiom 2: Start as a Seed, Grow Organically

> Original formulation: "Start simple; earn complexity."

The principle is the same: don't front-load complexity. But the *metaphor* changes what "simple" means.

In the control frame, "start simple" means a minimal factory — one production line, proven to work, then add more. Complexity is **added as pre-built components**: add a linter, add an evaluator, add a merge queue.

In the cultivation frame, "start simple" means a viable seed — the smallest structure that can grow. Complexity is **not added but grown**: recurring mistakes become linter rules; repeated decisions become conventions; accumulated conventions become the project's identity. The harness doesn't get components bolted on; it develops them from its own practice.

The critical difference: a factory's components are designed externally and installed. A garden's features emerge from the interaction between the seed, the soil, and the gardener's care. anima init plants a seed. The project — through the human-agent collaboration that follows — grows the harness.

### Axiom 3: Everything Is a Growth Cycle

> Original formulation: "Everything is a loop."

The mechanism is unchanged: act → feedback → adjust. What changes is the **purpose** of the loop.

In the control frame, the loop's purpose is **error correction**: agent makes a mistake → environment signals the error → agent fixes it. The loop converges toward correctness.

In the cultivation frame, the loop's purpose is **knowledge accumulation**: agent explores → environment responds → valuable discoveries are sedimented into the project's permanent knowledge. The loop converges toward wisdom.

Error correction is not abandoned — it is reframed as a *mechanism of growth*. When an agent makes a mistake and the human promotes the fix to a linter rule, two things happen: the error is corrected (control) and the project permanently learned something (cultivation). In the control frame, the first outcome is the point. In the cultivation frame, the second outcome is the point — the error was fertilizer.

---

## 3. Four Core Concepts

These concepts are implicit in the spec and guide but never named. anima names them because they are central to the cultivation thesis.

### 3.1 Growth

The accumulation of project knowledge makes the agent progressively more effective in *this specific project*. The model's capability doesn't change. The environment's richness does.

When the project was born, the agent knew nothing about it. After a week of collaboration, the agent "knows" the project's conventions (via `AGENTS.md`), its architecture (via `ARCHITECTURE.md`), its past decisions (via `docs/decisions/`), its quality standards (via linter rules and tests), and its boundaries (via type definitions and safety configuration). This knowledge makes the agent's next session more productive than its first. That is growth.

The spec treats knowledge storage as a hygiene obligation — keep documentation current so agents aren't misled (§4.2). anima treats it as the **primary growth mechanism** — every piece of knowledge added to the repo is a permanent increase in the agent's effective capability in this project.

### 3.2 Memory

LLM sessions are stateless. Projects are not.

The spec acknowledges this: "Sessions are cattle; work artifacts are persistent" (§3.1). But it frames persistence as a durability concern — ensuring work isn't lost when sessions end. anima reframes it as **memory** — the mechanism by which a project accumulates wisdom across sessions.

Agent "memory" does not live in the agent. It lives in the repository:

- `AGENTS.md` evolves as the project discovers what the agent needs to know.
- `docs/decisions/` grows as the team makes and records choices.
- Linter rules accumulate as recurring mistakes are promoted to mechanical enforcement.
- Test cases grow as the project discovers its own edge cases.
- Type definitions tighten as the project's domain model crystallizes.

Each of these is something the agent "remembers" the next time it starts a session. Not because the agent has persistent state, but because the environment carries the memory. Sessions are mayflies; the repository is tree rings.

### 3.3 Exploration

New projects begin with a phase of mutual discovery, not execution.

The spec's workflow artifacts — ExecPlan, Sprint Contract, Acceptance Criteria — presuppose that the human knows what to build. This is true for well-defined tasks in established projects. It is not true for the first days of a new project, especially one that is not bound to a predetermined technology stack.

In the exploration phase, the primary activity is not "build this" but "let's figure out what this should be." The human has a direction but not a blueprint. The agent has capabilities but no project context. Together they discover:

- What technology choices fit the problem.
- What architectural patterns emerge naturally.
- What conventions feel right and which feel forced.
- What the project's core beliefs are — not imported from a template, but distilled from actual practice.

Exploration is not an absence of structure. It is a different kind of structure — one optimized for learning rather than producing. The agent's role in this phase is co-discoverer: proposing, prototyping, questioning, and helping the human articulate what they want through the act of building.

The output of the exploration phase is not a product. It is the project's **initial identity** — a set of decisions, conventions, and beliefs that grew from practice rather than being prescribed from a template. This identity becomes the foundation for all subsequent growth.

### 3.4 Identity

After sufficient growth, each project's agent environment is unique — not because the model differs, but because the repository differs.

A project that has accumulated six months of decisions, conventions, linter rules, test cases, and architectural documentation produces a qualitatively different agent experience than a freshly initialized project. The agent operating in the mature project has access to a rich context that shapes its behavior: it "knows" the project's style, respects its boundaries, avoids its historical mistakes, and builds on its established patterns.

This is identity. It is not prescribed; it is emergent. It grows from the accumulation of practice-driven knowledge in the repository. Two projects initialized from the same seed will develop different identities, because they will have different experiences.

Identity is what makes "raising" an agent meaningful. A template gives every project the same starting point and the same rules. A seed gives every project the same starting *capacity* — but what grows from it is shaped by the project's own history. The sense of nurturing, of watching something develop, comes from identity being earned through care rather than installed from a blueprint.

---

## 4. The Seed

anima init plants a seed, not a template.

### What a template does

A template generates a fixed set of configuration files for a specific technology stack. It encodes opinions as pre-written rules: this linter config, this test framework, this directory structure. The user receives a working harness from day one.

Templates are efficient but fragile. They assume the template author's context matches the user's context. They work well when that assumption holds (e.g., a team that has standardized on one stack). They work poorly when it doesn't — and the user is left modifying a template that was designed around someone else's practice.

### What a seed does

A seed generates the **minimal structure needed for a harness to grow from practice**. It does not prescribe specific linter rules, test frameworks, or architectural patterns. It provides:

- **A skeleton for knowledge sedimentation.** Directory structures and file templates that make it natural to record decisions, conventions, and architectural understanding as the project evolves. The skeleton is empty on day one — its value comes from being filled through practice.

- **An entry point for the agent.** A minimal `AGENTS.md` designed not to instruct the agent on a predefined codebase, but to orient a new agent in a project that is itself new. It answers: "How do I start working in this project?" not "Here are all the rules you must follow."

- **Growth mechanisms.** Lightweight conventions for how discoveries become permanent knowledge: when a decision is made, where it's recorded; when a mistake recurs, how it's promoted to a rule; when the project's understanding deepens, where that understanding is captured.

- **Exploration guidance.** A framework for the initial phase of human-agent collaboration, when the project's identity has not yet formed. Not a checklist of things to decide, but a structure that makes exploration productive — ensuring that the discoveries made during exploration are captured rather than lost.

### What a seed does not do

- It does not prescribe a technology stack.
- It does not generate linter configurations or test framework setup.
- It does not fill in architectural documentation with placeholder content.
- It does not assume the user knows what the project will become.

The seed trusts that the right rules will emerge from practice — because the user and agent will discover them together — and provides the mechanisms for those rules to be captured and enforced once discovered.

---

## 5. The Growth Lifecycle

A project initialized by anima passes through recognizable phases. These are not rigid stages with gates between them — they are gradients that describe how the agent-project relationship deepens over time.

### Germination — Exploration

The project is new. The agent knows nothing about it. The human has direction but not a plan.

Primary activity: mutual discovery. The human and agent explore technology choices, architectural patterns, and conventions together. The agent proposes; the human evaluates; useful discoveries are recorded.

What grows: initial `AGENTS.md`, first architectural decisions, early conventions. The project's identity begins to form.

The harness at this phase is minimal — barely more than the seed. And that is correct. Premature rules would constrain exploration that hasn't happened yet.

### Rooting — Foundation

Core decisions have been made and recorded. The project has a technology stack, an architectural direction, and a set of emerging conventions.

Primary activity: establishing the feedback loop. Tests are written for the first time — not from a coverage mandate, but because the project now has enough structure to test. Linter rules appear — not imported from a template, but promoted from patterns the human noticed during exploration. Type boundaries are defined at the edges the project actually has.

What grows: a working feedback loop (tests + linters + types), a populated `ARCHITECTURE.md`, the first entries in `docs/decisions/`. The agent's sessions become noticeably more productive than they were during germination, because the environment is richer.

### Growth — Accumulation

The project is established. The feedback loop works. Development enters a rhythm: the agent works, the environment responds, knowledge accumulates.

Primary activity: the growth cycle. Each development cycle has two outputs: the intended product change and the knowledge sedimented from the process. A bug fix that exposed a missing test case → the test is added. A feature that required a new convention → the convention is recorded. A mistake that recurred three times → a linter rule is created.

What grows: the repository steadily enriches. The agent's effective capability in this project increases with each cycle. The harness becomes increasingly specific to this project — reflecting its actual history, not a generic template.

### Maturity — Identity

The project has a distinct character expressed in its repository. An agent starting a new session in this project behaves meaningfully differently from an agent in a freshly initialized project — not because of different prompting, but because the environment it reads is deeply informative.

The harness at this phase is a living record of the project's practice. It was not designed; it grew. And because it grew from practice, it is uniquely adapted to this project's actual needs.

Maturity is not an endpoint. It is a steady state where growth continues but the project's core identity is stable. New knowledge still accumulates; obsolete rules are still retired; the harness still evolves. But the evolution is refinement, not formation.

### Continuous: Pruning

Throughout all phases, growth includes deliberate removal. As models improve, scaffolding components that compensated for model weaknesses are retired (per Spec §4.3). As the project evolves, rules that no longer serve their purpose are removed. As conventions change, outdated documentation is pruned.

Pruning is not the opposite of growth — it is part of growth. A garden that only adds and never removes becomes overgrown. A harness that only accumulates and never simplifies becomes a burden. Healthy growth is growth with editing.

---

## 6. Relationship to the Foundational Documents

This philosophy does not replace the Harness Specification or the Harness Guide. It interprets them.

The spec and guide describe harness engineering as a discipline — the general theory of designing environments for AI coding agents. They adopt a control perspective because that is the natural default for a prescriptive specification. Their obligations (MUST, SHOULD, MAY) are derived from the three axioms and remain valid.

This document describes anima's specific application of that theory. anima chooses the cultivation perspective — not because the control perspective is wrong, but because cultivation better matches anima's purpose: enabling individual engineers to grow effective agent collaborators for their projects, starting from nothing, with no predetermined technology stack.

Every obligation in the spec remains applicable. What changes is the *motivation* for fulfilling them:

| Spec Obligation | Control Motivation | Cultivation Motivation |
|---|---|---|
| Store knowledge in the repo (§1.1) | Prevent agent context loss | Accumulate project wisdom |
| Maintain AGENTS.md (§1.1) | Give the agent a correct map | Give the agent a living, evolving understanding |
| Enforce via linters (§1.4) | Prevent agent violations | Sediment project learning into mechanical rules |
| Keep tests fast (§1.5) | Enable tight error-correction loops | Enable tight growth cycles |
| Manage entropy (§4.1) | Prevent codebase degradation | Maintain growth health; prune as part of growing |
| Evolve the harness (§4.3) | Keep assumptions current | Ensure growth remains organic, not calcified |

The spec is the physics. This philosophy is the biology. Same laws, different lens, different design intuitions.
