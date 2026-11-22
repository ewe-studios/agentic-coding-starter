---
name: "Story-Driven Specification"
description: "How to write each feature as a self-contained story: use cases, bounded context, entities and verbs, laws and invariants, must-have rules, Given/When/Then acceptance scenarios, a technical breakdown, and the tests that prove each law"
approved: No
created: 2026-11-11
license: "MIT"
metadata:
  author: "Main Agent"
  version: "1.0"
  last_updated: "2026-11-11"
  tags: [specifications, features, stories, bounded-context, laws, invariants, bdd, acceptance-tests]
tools: []
files: []
---

# Story-Driven Specification

_Written in ASD-STE100 Simplified Technical English (STE)._

## Overview

A feature file is one story that a new reader can follow from start to end. It says what the
feature is for, which things work together, which rules are always true, how the behaviour
looks from outside, how it is built, and which tests prove it.

Use this skill each time you write or update a `feature.md`. The skill
`specifications-management` says where the files go. This skill says what is in them.

**Usage Type**: EDUCATIONAL. Learn the method, then write the feature.

## Why

A list of tasks tells an agent what to type. It does not tell the agent what must stay true.
An agent that does not know the rules makes code that passes its own tests and breaks the
feature beside it.

A story with laws fixes this:

- The **stories** give the purpose. An agent can check each decision against them.
- The **bounded context** gives the edges. An agent knows what it owns and what it must not
  touch.
- The **laws** give the rules that every input and every sequence must obey. They are the
  stable part. Code and tests can change; the laws stay.
- The **scenarios** give the behaviour in words that the owner can read and approve.
- The **tests** are tied to the laws, so "done" has one meaning.

## The method: eight steps, in this order

Do not start with types or files. Start with sentences.

### Step 1: Write the stories

Write each story as one sentence with a subject, a verb and an object. A person who does not
know the code must be able to read it.

```
S1: "A caller starts a sandbox from a root file system."
S2: "A caller runs a command and reads its output while it runs."
S3: "The guest stops answering during a command."
```

Rules:

- One story is one thing that an actor wants.
- Write the failure stories too ("the payment fails", "the guest stops answering"). A failed
  operation is a state, and it needs its own rules.
- Give each story an ID (`S1`, `S2`). Scenarios and tests refer to it.

Then write a use case table: ID, actor, trigger, result.

### Step 2: Extract the nouns and the verbs

- The **nouns** of the stories are the entities and the value objects.
- The **verbs** are the operations.
- Do not cut a real verb to keep a list short. A missing verb becomes a special case in the
  code, and special cases hide bugs.

Separate two kinds of verb:

| Kind | Meaning | Example |
|---|---|---|
| Entity verb | Changes one entity. It has a clear guard or a clear inverse. | `spawn`, `signal`, `debit` |
| Story verb | Changes several entities. It owns the order of the steps and what happens when a step fails. | `create sandbox`, `checkout` |

### Step 3: Draw the bounded context

A bounded context is the set of things that work together for this feature, with a clear
edge.

Write:

1. **Inside**: the entities that this feature owns.
2. **Outside**: the things that this feature uses and does not own. Name the feature or the
   crate that owns each one.
3. **One writer for each piece of data.** For each entity, name the one module that can
   change it. Other code reads a view or calls a verb.
4. **The words** (ubiquitous language): one word for one thing. The code uses the same
   words. If two contexts use one word for two things, say so.

Use a table for the entities (name, meaning, owner) and a table for the verbs (verb,
trigger, effect).

### Step 4: Find the laws

A law is a sentence that is true for every valid input and every valid sequence of
operations. A law does not call the implementation.

Ask **five questions of each verb**:

1. What does it change?
2. What must it never change?
3. What is its inverse? If it has none, say so.
4. What happens when it is applied two times? (rejected, ignored, or applied again)
5. What happens when it is reordered with a **different** verb? Run both orders.

A pair of verbs is **independent** (both orders agree), **enabling** (the first makes the
second possible) or **competing** (both want one limited thing; the first wins).

Ask **three questions of each entity**:

1. What is conserved? (a total that cannot change)
2. What is bounded? (a value that must stay in a range)
3. What is linked to another entity? (a fact that is in two places and must agree)

The answers are the laws. Name each kind:

| Kind | Shape |
|---|---|
| Conservation | A total does not change. |
| Bound | A value stays in a range. |
| Inverse | Verb B undoes verb A. |
| Idempotence | A second call has no more effect, or is rejected. |
| Ordering | Verb A must come before verb B. |
| Monotone | A value only moves one way. |
| Freeze | A value does not change after a point. |
| Linkage | Two entities agree about a fact. |
| Atomicity | A failed story leaves no partial change. |

Fill in the law worksheet for each entity before you write a scenario:

```
Entity: ____________________
Conserved value:         ____________________
Bound:                   ____________________
Inverse pairs:           ____________________
Idempotent verbs:        ____________________
Ordering pairs:          ____________________
Monotone values:         ____________________
Freeze points:           ____________________
Links to other entities: ____________________
```

Write "none" where a line has no answer. An empty line means that nobody asked the question.

**Bad laws.** Do not write these:

- A tautology: "the function returns what it returns".
- A copy of the code. Write the law from the rule, not from the implementation.
- An end-state-only law: "after `pay` two times, the status is PAID". It cannot see the
  second call. Say what the second call must do.
- A vague subject: "the system works". Name the value, the operation and the relation.

### Step 5: Write the rules lists

From the laws and the stories, write four short lists. Each line is one sentence that a
reviewer can check.

| List | Content |
|---|---|
| **Invariants** | The laws that are true at every moment. Number them (`I1`, `I2`). |
| **Must have** | The things that must exist when the feature is done (a type, a mode, a limit). |
| **Must do** | The actions that the feature must take in each case (clean up, report, reject). |
| **Must not** | The actions that are forbidden, also when they look useful. |

### Step 6: Describe the behaviour

Write acceptance scenarios in Given / When / Then form (Gherkin). The owner approves these.

```gherkin
Scenario: A missing program does not start
  Given a ready sandbox
  When the caller runs ["/no/such/program"]
  Then the call fails with NotFound
  And no process started in the guest
```

Rules:

- One scenario checks one behaviour. Its name is the behaviour.
- `Given` is state. `When` is one action. `Then` is what an outside observer sees.
- Check the **outcome of the call**, not only the end state. A rejection that returns
  quietly is not a rejection.
- Write one scenario for each failure story, and one for each row of the fault table
  (step 7).
- Use real values (`exit 7`, `4096 bytes`), not "some value".
- Each scenario becomes one named test. The scenario text and the test name match.
- A driver (the code under `Given` and `When`) translates. It does not decide. A driver
  that hides a rule moves the rule out of the system.

### Step 7: Write the technical breakdown

Now, and only now, write how it is built:

- types and signatures, in full;
- state machines, as a table of states with work, output and next state;
- sequences, as Mermaid diagrams with a one-line caption;
- every limit, with its default and the source of the default;
- every error, with the condition that gives it;
- a **fault table** for each story verb: the step that fails, what the system does, what
  the caller sees, what is left behind (the answer must be "nothing" or a named thing);
- the files to create or change.

No hidden detail. If a behaviour depends on a fact about another system, find the fact and
cite the source (file and line, or a document). If you cannot find it, the feature is not
ready: do the research first.

### Step 8: Tie the tests to the laws

A test has two parts: a generator (which inputs) and an oracle (how it knows the answer).
Choose the cheapest oracle that can decide the law:

1. An example test, for one named scenario.
2. A property test, when the law is one line about the arguments and the result
   (identity, conservation, bound).
3. An expected-outcome check, when the law is about which error and when.
4. A stateful test with a model, when the answer depends on the order of several calls.
5. A metamorphic test, when you cannot compute the answer but you know how two runs relate.
6. A fault test, for an atomicity law. Inject the fault at a port, not by a patch inside.

Write a table: each scenario and each invariant, the test that proves it, and the kind of
oracle. A law with no test is not done. A test with no law is a question: which rule does it
protect?

Then write "Verify" (the exact commands) and "Done when" (the list that the reviewer checks).

## The feature file layout

Every `feature.md` has these sections, in this order.

| # | Section | Content |
|---|---|---|
| 1 | Goal | Two or three sentences. What exists when this is done. |
| 2 | Story | The story sentences and the use case table (step 1). |
| 3 | Bounded context | Inside, outside, entities, verbs, words (steps 2 and 3). |
| 4 | Rules | The law worksheet, then invariants, must have, must do, must not (steps 4 and 5). |
| 5 | Acceptance scenarios | Gherkin (step 6). |
| 6 | Technical breakdown | Types, state machines, sequences, limits, errors, fault tables, files (step 7). |
| 7 | Tests | The table from laws and scenarios to tests (step 8). |
| 8 | Verify | Commands. |
| 9 | Done when | The check list. |

A feature that only moves or renames code still has sections 1 to 5. They are short, and
they say which behaviour must not change.

## The check list

Use this list before you hand a feature to an agent, and when you review one.

```
[ ] Each story is one sentence with an actor.
[ ] The failure stories are written.
[ ] The nouns and the verbs are extracted from the stories.
[ ] Each entity has one owner (one writer).
[ ] The outside of the context names the owner of each thing.
[ ] The five questions were asked of each verb.
[ ] The three questions were asked of each entity.
[ ] The law worksheet has no empty line.
[ ] Each law is a sentence, not a copy of the code.
[ ] The invariants are numbered.
[ ] Must have, must do and must not are written.
[ ] Each story has one scenario or more.
[ ] Each scenario checks the outcome of the call.
[ ] Each story verb has a fault table.
[ ] Each fault row says what is left behind.
[ ] Each limit has a default and a source.
[ ] Each fact about another system has a source.
[ ] Each invariant and each scenario has a test.
[ ] Each test names its oracle.
[ ] No word such as "maybe", "if possible" or "to be decided" is in the file.
```

## Common mistakes

1. **Starting with the types.** The types then decide the stories. Start with sentences.
2. **Merging a story into an entity.** A story is a process, not a thing. Keep the entities
   separate and put the order of the steps in one place.
3. **Two writers for one piece of data.** The invariant then has no owner.
4. **Checking only the end state.** Two systems can reach one end state through different
   meanings.
5. **Forgetting the failed operation.** Write its scenario and its fault row.
6. **A law that calls the implementation.** It agrees with the bug.
7. **A scenario with no observable result.** If an outside observer cannot see it, it is
   not a `Then`.
8. **A guard around the assertion.** An `if` that skips the check makes a weak input look
   like a pass. Put the condition in the generator.
9. **"Measure later" in a feature.** A fact that the design depends on is found before the
   hand-off, or the design changes so that it does not depend on it.

## Source

The method follows "Scenarios Across a Domain: Three Entities, Their Laws, and the Drivers
That Join Them" (stories, nouns and verbs, bounded contexts, the five and three questions,
the law worksheet, drivers that translate, oracles, fault tables).

---

_Created: 2026-11-11_
