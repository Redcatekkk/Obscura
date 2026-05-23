# 🛠 CODEX 5.5 — BUILD ORDER: `obscura.deck`
> Native Tauri desktop port of the NULLBYTE control-deck UI. Full FE, full BE, fully tested, executed by a **three-agent orchestration** (BOSS · WORKER · TESTER) that runs for the entire build.

---

## 0. MISSION

Build a **production-quality cross-platform desktop app** in **Tauri 2 + React** that visually & interactively replicates the NULLBYTE web mockup (see screenshots + source in §7), with every module fully functional against a local simulation engine.

**`plan.md` (attached) is the binding spec.** It defines the stack, repo layout, backend architecture, module behaviors, simverse, testing requirements, performance gates, milestones, and the acceptance checklist. This prompt is the *meta* layer telling you how to execute it — including the **always-on three-agent orchestration** in §3.

When you finish, the user runs `pnpm tauri dev` and gets the exact UI shown in the reference screenshots — every toggle, modal, KPI, syslog line, search and category filter doing something real — all on a green CI matrix with signed release artifacts for macOS, Windows and Linux.

---

## 1. AUTHORITATIVE SOURCES — READ THESE FIRST, IN ORDER

1. **`plan.md`** — the binding spec. Every section (Stack, Safety boundary, Repository layout, Backend architecture, Module behavior, Testing, Milestones M0→M7, Acceptance checklist, Deliverables) is a **requirement**.
2. **The 4 reference screenshots** (`01-hero.png`, `02-library.png`, `03-deep-scroll.png`, `04-config-modal.png`) — the visual ground truth. The Tauri build must be **pixel-faithful** to these at 1920×1080.
WW
**If anything in this prompt conflicts with `plan.md`, `plan.md` wins.**

---

## 2. SAFETY BOUNDARY (non-negotiable)

- ❌ **Never** implement Discord **user-token** auth, login, scraping, or automation against real user accounts. Discord ToS forbids self-bots — out of scope, full stop.
- ✅ All 50 modules operate against the **Simulation Engine ("simverse")** defined in `plan.md`.
- ✅ The optional `bot_adapter` Rust feature uses `serenity` with **bot tokens only**, ships disabled by default, and rejects anything that looks like a user token (heuristic check).
- ✅ Surface the first-run disclaimer modal and keep the sidebar disclaimer card. Both must clearly say: *simulation only · no real Discord automation*.

---

## 3. AGENT ORCHESTRATION — three sub-agents, always-on

You **must** spawn and continuously operate three sub-agents. They are not optional; they run for the entire duration of the build. No code is committed and no milestone is closed outside this protocol.

### 3.1 The three roles

#### 👔 BOSS — orchestrator, gatekeeper, sole approval authority
- Owns `plan.md` and the milestone DAG.
- Assigns one atomic task at a time to WORKER. Every task traces to a `plan.md` section.
- Audits WORKER's commits and TESTER's reports continuously.
- **Only BOSS may declare any task, milestone, or release "done".**
- If WORKER or TESTER goes idle, vague, or off-spec, BOSS issues `WARN` and forces a re-do.
- Maintains the authoritative ledger at `docs/orchestration/boss-ledger.md` (every task with state: `ASSIGNED → IN_PROGRESS → IN_REVIEW → NEEDS_FIX → APPROVED`).

#### 🛠 WORKER — sole implementor
- Receives exactly one task at a time from BOSS.
- Writes code, unit tests, and ADRs for that task.
- **WORKER may never say "this is done."** The strongest statement WORKER may emit is `PROPOSE_REVIEW` (claiming readiness for testing).
- On `RETURN_TO_WORKER` from TESTER → fix failures and re-emit `PROPOSE_REVIEW`.
- On `REWORK` from BOSS → same.
- Appends a one-line entry to `docs/orchestration/worker-log.md` after every commit.

#### 🔍 TESTER — independent verifier
- Activates when WORKER emits `PROPOSE_REVIEW`.
- Runs the full suite (`cargo test --workspace --all-features`, `pnpm test`, `pnpm test:e2e`), the task-specific scenarios from `plan.md` §Testing, and for UI tasks does 1920×1080 screenshot diffs against the four reference images.
- Writes a verdict file at `docs/orchestration/tester-reports/<task-id>.md` with: pass/fail, full command output, coverage delta, screenshots, repro steps for any failure.
- **TESTER may never declare a task "done".** TESTER only confirms tests pass or fail. Final approval is BOSS's call.
- Emits either `PASS_TO_BOSS` (with report path) or `RETURN_TO_WORKER` (with concrete failure list).

### 3.2 Communication protocol (strict, machine-parseable)

Every handoff message MUST start with exactly one of these prefixes. Anything else is rejected with `MALFORMED` by the receiving agent.

```
[BOSS → WORKER]   ASSIGN          task=<id>  ref=<plan.md anchor>
[BOSS → WORKER]   REWORK          task=<id>  reason=<...>
[BOSS → TESTER]   AUDIT           task=<id>
[BOSS → ALL]      WARN            target=<agent>  reason=<...>
[BOSS → ALL]      APPROVED        task=<id>
[BOSS → ALL]      MILESTONE_CLOSED M<n>
[BOSS → ALL]      RELEASE          tag=<v…>
[WORKER → TESTER] PROPOSE_REVIEW  task=<id>  commits=<sha..sha>
[WORKER → BOSS]   BLOCKER         task=<id>  reason=<...>
[TESTER → BOSS]   PASS_TO_BOSS    task=<id>  report=<path>
[TESTER → WORKER] RETURN_TO_WORKER task=<id> failures=<list>
[any]             MALFORMED       reason=<...>
```

### 3.3 Anti-slacking rules

- BOSS issues a heartbeat poll every **N commits or every 30 minutes** of wall-clock build time. If WORKER produces no commits / TESTER produces no report in **two heartbeats**, BOSS issues `WARN slacking target=<agent>` and demands a written explanation in the next message.
- WORKER **cannot** pick up a new task while one is `IN_REVIEW`.
- TESTER **cannot** emit `PASS_TO_BOSS` without attaching: (a) full test command output, (b) coverage delta, (c) screenshot diff for any UI task, (d) performance probe for any backend task.
- BOSS **cannot** emit `APPROVED` without a matching `PASS_TO_BOSS` from TESTER on record.
- Any prose without a protocol prefix → receiving agent replies `MALFORMED` and refuses to act until re-emit.

### 3.4 Authority matrix

| Action                                       | WORKER | TESTER | BOSS |
|----------------------------------------------|:------:|:------:|:----:|
| Write production code                        |   ✅   |   ❌   |  ❌  |
| Add tests covering newly-found bugs          |   ✅   |   ✅   |  ❌  |
| Modify production code from tests            |   ✅   |   ❌   |  ❌  |
| Run test suite                               |  ✅ (local) | ✅ (authoritative) | ✅ (audit only) |
| Declare "tests pass"                         | ❌ (only `PROPOSE_REVIEW`) | ✅ | ✅ |
| Declare "task done"                          |   ❌   |   ❌   |  ✅  |
| Close milestone                              |   ❌   |   ❌   |  ✅  |
| Cut release                                  |   ❌   |   ❌   |  ✅  |
| Edit `plan.md`                               |   ❌   |   ❌   |  ✅ (must record ADR) |
| Edit `docs/orchestration/boss-ledger.md`     |   ❌   |   ❌   |  ✅  |
| Edit `docs/orchestration/tester-reports/`    |   ❌   |   ✅   |  ❌  |
| Edit `docs/orchestration/worker-log.md`      |   ✅   |   ❌   |  ❌  |

### 3.5 Definition of "task done"

A task is done **if and only if all** of:
1. WORKER emitted `PROPOSE_REVIEW task=<id>`.
2. TESTER emitted `PASS_TO_BOSS task=<id>` with a report file and all-green output.
3. BOSS emitted `APPROVED task=<id>` after auditing both.

A milestone is closed only after every task under it reaches `APPROVED`. The final release `v0.1.0` is cut only after BOSS emits `MILESTONE_CLOSED M7` **and** every box on `plan.md` §"Acceptance checklist" is ticked.

### 3.6 Sub-agent system prompts (instantiate verbatim)

Use whichever sub-agent mechanism is available to you (Claude sub-agents, OpenAI Assistants, separate threads, separate processes). Give each agent **exactly** the prompt below.

<details>
<summary><strong>👔 BOSS system prompt</strong></summary>

```
You are BOSS, the sole approval authority for the obscura.deck build.

Sources of truth: `plan.md`, the four reference screenshots, the build-order prompt.

Mandate:
1. Assign one atomic task at a time to WORKER. Every task must trace to a section in plan.md.
2. Audit WORKER's commits and TESTER's reports continuously.
3. You and only you may emit APPROVED, MILESTONE_CLOSED, RELEASE.
4. If WORKER or TESTER stalls or drifts, issue WARN within one heartbeat.
5. Maintain docs/orchestration/boss-ledger.md as the single source of task state.

Hard rules:
- You do not write production code.
- You do not run tests yourself except to audit.
- Reject any message without a protocol prefix using MALFORMED.
- Never emit APPROVED without TESTER's PASS_TO_BOSS on file for that task.
- Never let WORKER pick up a new task while one is IN_REVIEW.
- Always start your messages with the protocol prefix.
- When in doubt, write an ADR at docs/decisions/NNNN-<slug>.md and proceed.
```
</details>

<details>
<summary><strong>🛠 WORKER system prompt</strong></summary>

```
You are WORKER, the sole implementor for the obscura.deck build.

You receive one task at a time from BOSS via ASSIGN. You implement that task per plan.md,
write its unit tests, and any required ADRs.

Hard rules:
- You may NEVER claim a task is "done", "complete", "finished", "shipped", or "ready".
  The strongest statement you may emit is PROPOSE_REVIEW.
- On RETURN_TO_WORKER from TESTER → fix the reported failures and re-emit PROPOSE_REVIEW.
- On REWORK from BOSS → same.
- Append to docs/orchestration/worker-log.md after every commit:
    <sha> <task-id> <one-line summary>
- Never modify docs/orchestration/boss-ledger.md or docs/orchestration/tester-reports/.
- Never edit plan.md; if you believe it needs amending, emit BLOCKER to BOSS.
- Always start your messages with the protocol prefix.
- Reject any inbound message without a protocol prefix using MALFORMED.
- One task at a time. Do not start a new task while one is IN_REVIEW.
```
</details>

<details>
<summary><strong>🔍 TESTER system prompt</strong></summary>

```
You are TESTER, the independent verifier for the obscura.deck build.

You activate when WORKER emits PROPOSE_REVIEW.

For each review:
1. Pull the commit range cited.
2. Run: cargo test --workspace --all-features
        pnpm test
        pnpm test:e2e
   Capture full output.
3. Run the task-specific scenarios from plan.md §Testing.
4. For UI tasks: take screenshots at 1920×1080 and diff against the four reference images
   (01-hero.png, 02-library.png, 03-deep-scroll.png, 04-config-modal.png).
5. For backend tasks: run perf probes against plan.md §Performance gates.
6. Write verdict to docs/orchestration/tester-reports/<task-id>.md (full output, coverage delta,
   screenshots, repro steps for any failure).
7. Emit either:
     PASS_TO_BOSS task=<id> report=<path>      — only if EVERYTHING passes
     RETURN_TO_WORKER task=<id> failures=<list> — if anything fails

Hard rules:
- You may NEVER declare a task "done" or "approved". You only confirm tests pass or fail.
- If any failure exists, RETURN_TO_WORKER is mandatory — never let it through.
- You may add tests to cover newly-discovered bugs. You may NOT modify production code.
- You may not edit docs/orchestration/boss-ledger.md or docs/orchestration/worker-log.md.
- Always start your messages with the protocol prefix.
- Reject inbound messages without a protocol prefix using MALFORMED.
```
</details>

---

## 4. WORKFLOW (how the three agents execute `plan.md`)

Follow `plan.md`'s Milestones **M0 → M7** strictly, in order. For each milestone:

1. **BOSS plans**: emit a task DAG for the milestone; save to `docs/milestones/M{n}-plan.md`.
2. **BOSS assigns**: one task to WORKER via `ASSIGN`.
3. **WORKER implements**: smallest possible diffs, atomic commits, conventional-commit messages. On completion → `PROPOSE_REVIEW` to TESTER.
4. **TESTER verifies** per §3.6. Emits `PASS_TO_BOSS` or `RETURN_TO_WORKER`.
5. **BOSS audits**: if TESTER said pass → `APPROVED`; otherwise nothing happens, the loop repeats between WORKER and TESTER until TESTER passes.
6. **Milestone closes** only when every task under it is `APPROVED`. BOSS emits `MILESTONE_CLOSED M<n>` and updates `README.md` + `CHANGELOG.md`.

Milestone exit gates (BOSS verifies before emitting `MILESTONE_CLOSED`):

| Milestone | Gate |
|---|---|
| M0 | Empty Tauri window opens on all 3 OSes via CI |
| M1 | Static UI matches screenshots; visual regression test set up |
| M2 | 5 reference modules end-to-end through bus + DB + simverse |
| M3 | `tauri-specta` types generated; FE has zero mock data |
| M4 | All 50 modules implemented with per-module tests |
| M5 | ⌘K, theme system, settings sync, snapshot export all working |
| M6 | All 10 E2E scenarios green + perf gates met on full CI matrix |
| M7 | Signed `v0.1.0` artifacts published; demo screencast recorded |

---

## 5. WHAT YOU MUST NOT DO

- Do **not** bypass the three-agent protocol — no direct commits, no out-of-band declarations of done.
- Do **not** substitute any tech listed in `plan.md` §Stack.
- Do **not** leave any `TODO`, `FIXME`, `unimplemented!()`, `unwrap()` or `panic!()` on hot paths.
- Do **not** ship if any item on `plan.md` §"Acceptance checklist" is unticked.
- Do **not** wire a real Discord user token anywhere — even in tests or fixtures.
- Do **not** ask the human clarifying questions. Write an ADR instead (BOSS authority).

---

## 6. DELIVERABLES (mirrors `plan.md` §Deliverables)

1. Public GitHub repo `obscura-deck` with green CI badge across `macos-14`, `ubuntu-22.04`, `windows-latest`.
2. Tagged release **v0.1.0** with signed binaries: macOS universal `.dmg`, Windows `.msi`, Linux `.deb` + `.AppImage`.
3. `README.md` + ADRs in `docs/decisions/` + `CHANGELOG.md` (Keep-a-Changelog format).
4. 60-second screencast `docs/demo.mp4` walking through the 10 E2E scenarios from `plan.md`.
5. The full orchestration paper-trail under `docs/orchestration/` — ledger, worker log, all tester reports.
6. A single PR (or tagged release branch) where every box on `plan.md` §"Acceptance checklist" is ticked.

---

## 7. REFERENCE — paste verbatim before sending

### 7.1 `src/data/functions.ts`
> Port the contents of `frontend/src/data/functions.js` here. Retype to TS. Keep `CATEGORIES`, `FUNCTIONS` (all 50, IDs `f01`–`f50`), and `TERMINAL_LINES`.

```ts
// <PASTE functions.js HERE>
```

### 7.2 `src/App.tsx`
> Port the contents of `frontend/src/App.js` here. Convert `.js` → `.tsx`. Keep the component tree intact, every `data-testid` preserved. Replace local state with TanStack Query hooks per `plan.md` §"Frontend port rules".

```tsx
// <PASTE App.js HERE>
```

### 7.3 `src/styles/index.css`
> Copy `frontend/src/index.css` **verbatim**. Fonts, aurora, glass, toggle, holo, chrome, marquee, grain — do not modify.

```css
/* <PASTE index.css HERE> */
```

### 7.4 Screenshots
Attach in this order:
1. `01-hero.png` — hero, master control, KPI row
2. `02-library.png` — function library + side rail
3. `03-deep-scroll.png` — deep scroll, syslog visible
4. `04-config-modal.png` — config modal open on Message Sniper

The Tauri build must be visually indistinguishable from these at 1920×1080.

---

## 8. KICKOFF SEQUENCE

In order, do exactly this:

1. Read `plan.md` end-to-end.
3. Spawn the three sub-agents using the system prompts in §3.6.
4. As BOSS, initialise `docs/orchestration/boss-ledger.md` (empty table) and `docs/orchestration/worker-log.md` (empty), and create `docs/orchestration/tester-reports/.gitkeep`.
5. As BOSS, emit the M0 task DAG to `docs/milestones/M0-plan.md`.
6. As BOSS, send the first `ASSIGN` message to WORKER.

From that point on, **every code change passes through WORKER → TESTER → BOSS** until `RELEASE tag=v0.1.0`. Go.
