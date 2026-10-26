# Retrospective: render

2026-10-18. hearth's M1: the render and diff of a fleet's configuration,
with its store, client, protocols, fakes and worlds, and skein's part of it.

## 2. Outcome

Checks at 05:33 against hearth `f21b5c8`, skein `c0662b5` and smith `3ae65b4`
(done.md):

2. **The 21 M1 crate paths exist on `origin/main`: passes.** 21 of 21.
3. **`pub fn position(` and `fn state_digest(` are public in skein:
   passes.**
4. **The end-to-end render and diff pass with at least two tests:
   passes.** 3 tests in 0.081 s.
5. **The conformance profile passes within its 2-minute ceiling:
   passes.** 24 tests in 43.5 s.
6. **One `CLIENT_VERSION: u32 = 1`, in the records crate: passes.**
7. **The suites' medians of three runs, against their aims: passes.**
   - hearth: focused 7.291 s (aim 8), fuzzy 16.004 s (aim 30).
   - skein: focused 7.632 s (aim 7.96), fuzzy 43.777 s (aim 46.84).
   - During the run, hearth's focused suite was past its aim from 02:30
     to 04:16 (section 5, item 5).
   - skein's aims rest on x1's base (6.46 s and 41.84 s), not on this
     machine's (6.27 s and 30.04 s, `findings/measurements.md`). Its
     fuzzy median passes by x1's base, 13.7 s over this machine's, while
     the last landing's gate read 29.5 s.
8. **No budget raised: passes.** Only the added conformance profile's
   `global-timeout` and `period`; nothing in skein.
9. **smith pins skein's last `Plan: render` landing, or a later one:
   fails.**
   - smith pins `841f487`, skein-03.4's. skein's last `Plan: render`
     commit is `0b08745`, skein-03.s2's.
   - smith still pins `841f487`. A repin of smith to skein's `main`
     takes both supplements.
The plan's 17 added steps:
- **From the plan (5):** skein-02.s1 (a test file missing from 02.1's
  Files), hearth-09.s1 (a count checked after its scan, against the
  session's own rule), hearth-12.s1 (a decode error no step owned),
  hearth-45.s1 (generic items, which skein's programming-model.md
  forbids, in the plan's own sketches), skein-03.s1 (a world API the
  plan assumed).
- **From neither (3):** skein-04.s1 and skein-03.s2, faults in skein's
  fakes; hearth-45.s2, code that missed a Proof the plan had right.
- **From the 1 GiB bound (9):** hearth-28.s1 to 28.s9 (section 5, item
  2).

## 5. What did not

2. **The 1 GiB bound was summed at the end.**
   - **Cause.**
     - The design asks that the builder's worst case at the shipped
       limits fit in 1 GiB. Each crate had its `worst_case`, and ten
       sessions tested their own layer's. Only 28.5, the last increment
       of session 28, summed them.
     - The bound's premise was answered without a measurement. The
       planner had asked it (H27): "At the shipped limits git's listing
       decoder alone holds more than 256 MiB". The answer, by a fork of
       the third design round, ran no probe. The planner then renamed
       the test to `the_shipped_limits_have_a_worst_case_within_1_gib`,
       unmeasured.
     - parity's adopted rule, "a bound's premise checked by a command
       in `findings/`", did not reach a bound that no single command
       could compute before the code existed.
   - **First visible:** 01:14:16, 28.5's report: 1,240,426,175 bytes.
   - **Acted on.**
     - Ruling 184 at 01:18 let 28.5 land without the test. The answer,
       36959af, said the defaults would be decided "once the peaks are
       measured"; ruling 192 at 01:59 added 28.s1 to s6.
     - Each step's tests then found another holder the formulas left
       out: 28.s7 (ruling 198), 28.s8 (200), 28.s9 (202, 203).
     - At 04:48, 28.s6 summed the peaks at 2,497,232,530 bytes. The
       designer lowered seven defaults (decd06a, 05:13), and 28.s6
       landed at 05:24:59 at 899,981,218.
   - **Cost:**
     - nine supplements, four design answers and 4 h 13 min;
     - at most two sessions from 03:25 and one from 04:16, and none for
       28 minutes while 28.s6 waited on the designer;
     - session 28 open for 11 h 19 min, with 14 threads and 11
       handovers (proposal 4).
5. **The focused suite stayed past its aim for 106 minutes.**
   - **Cause.**
     - The README says the next increment of a session that misses the
       aim first makes the slowest tests cheaper. Session 30, whose 30.1
       added 15 fuzzy tests and crossed 8 s at 02:30:01, never reported
       a measure run.
     - The 02:31 hourly check judged 8.3 s against the 15 s budget, not
       the 8 s aim ("the 02:31 check measured the focused suite against
       its budget, not its aim").
     - The pulse prints the budgets (`budgets: 15 60`), not the aims.
   - **First visible:** 02:30:01, 30.1's gate at 8.307 s.
   - **Acted on:** 03:42:10, a steer, after ten landings at 8.3 to 10.7
     s. 47.1 brought it to 7.342 s at 04:16, keeping each memory case's
     peak and every ceiling (proposal 7).
## 6. Decisions taken during the run

| Decision | Rulings | Reached the design? |
|---|---|---|
| A container's `mutable` is refused before M5; the tokenizer places string faults; the tree world keeps its name | 121, 122 | Yes, `3f16a56` |
| A guard's name is an identifier | 124, 128 | Yes, `180c4a5` |
| A container renders whatever else fails on its host | 162, 165 | Yes, `562994c` |
| A third script name, or an order out of range, is a decode error | 169, 170 | Yes, `a079343` |
| Keys are unique within their declaration | 183, 185 | Yes, `01ffde6` |
| Render's worst case is what it holds at once; each layer's formula is its peak; errors kept to 256 and 16 MiB | 184, 192 | Yes, `36959af` |
| A diff's second side decodes only the changed blobs | 196, 198 | Yes, `9a639f6` |
| A template counts 4 bytes a line | 199, 200 | Yes, `a28817a` |
| The shipped defaults change, to fit 1 GiB | 204 | Yes, `decd06a` |
| An expanded record counts as a map of its fields; a range field as its base and count | 202, 203 | render.md 2.3 as it stood; 203's byte counts are the plan's |
| `null` inside a list is a decode error, owned by 12.s1 | 145 | The design had it (format.md 4.1, render.md 7.2) |
| Client records' wordings and fields: forms, a store refusal, the resolution kind, `Ids` with its entry, `Owns::User` | 108, 139, 148, 149, 153, 157, 167, 174, 175 | No. A grep of `docs/design` finds none of their wordings; client/README 3.2 makes field names stable within a major version (inference) |
| No generic item in production code | 155 | skein's programming-model.md 1 and 10.3 already say it |
| Test shape: no constant-index read (137), a round trip's expected value is its input (144), written once (159), a row holds its expected value (176) | | The plan's README. testing-strategy.md 10 says the last two at `main`, not in the run's kit |

## 9. Debts left in the code

hearth has no `docs/development/tech-debt.md`. These belong in one,
with what was seen, why it matters and what pays it back. Paths are at
hearth `main`.

1. **Eleven files within 100 lines of the 800-line aim.** The next
   increment that touches one splits it first.
   - Tests: `hearth-protocol-builder/src/tests/translate.rs` 799,
     `tests/builder/src/fixtures.rs` 785,
     `hearth-protocol-builder/src/tests/fixtures.rs` 785,
     `hearth-domain-render/src/tests/fixtures.rs` 764,
     `tests/builder/tests/real.rs` 735.
   - Production: `hearth-client-protocol-records/src/errors.rs` 787,
     `hearth-protocol-builder/src/lib.rs` 784,
     `hearth-domain-render/src/compose.rs` 743, `schema.rs` 721,
     `entries.rs` 717, `call.rs` 704.
2. **`RANGE_BASE` and `RANGE_COUNT` are defined twice,** with
   different values: `hearth-domain-render/src/tests/fixtures/entries.rs:300`
   and `fixtures/collisions.rs:182`. Small.
3. **Render's `fixtures/hearth.rs` `target_with` takes `[Option<&str>;
   6]`,** six unnamed slots with string labels. The 01:40 steer asked
   for named enums; no increment edited the file after. Small.
4. **The oracle's rows wrap 63 expected values in
   `Outcome::Text(Cow::Borrowed(..))`** (`tests/render/tests/oracle.rs`,
   `oracle/engine.rs`). Small.
5. **47.1's six numbered cut-pipe tests** are one table. Small.
6. **No test proves that 31.1's record survives the scan**. Medium.
7. **The three at-the-limit memory tests** of the render and template
   worlds take 1.3 to 1.8 s each, 6.7 s of focused test time, and were
   flagged slow 1,000 times. They keep their peaks by the steer's rule;
   a smaller limit with the same formula would keep the proof. Medium.
8. **Tests that failed more than one gate,** cause not established:
   `thin::a_template_and_a_link_are_left_out_of_the_tree` (three gates,
   sessions 13 and 15), `compose::fleet_and_environment_keys_never_add_a_task`
   and `a_dropped_template_entry_is_never_run` (sessions 24 and 22), and
   `fuzzy_simulation::a_builder_killed_before_each_submission_restarts_cold_and_settles`
   (two gates in session 28). Medium.
9. **`#[must_use]` is ruled per session** (rulings 109, 118, 121, 123,
   125, 140, 157), never by a lint. Clippy's `must_use_candidate` could
   hold it. Small.
10. **Client records' wordings and fields set by rulings, not by the
    design** (section 6). The designer's, not code's. Medium.
