# SATURDAY → TUESDAY INTERVIEW PLAN
# Today: Saturday | Interview: Tuesday | Role: Staff SDE @ Zscaler

**Vicky — follow this only.** Do not try to read every optional file.

Open checklist master: [`00-READ-IN-THIS-ORDER.md`](00-READ-IN-THIS-ORDER.md)

---

## Time you have

| Day | Date role | Goal |
|-----|-----------|------|
| **Sat** | Today | Understand gaps + finish main study book |
| **Sun** | Practice day | Speak answers out loud + design on paper |
| **Mon** | Polish + mock | One full 60-min mock + weak spots only |
| **Tue** | Interview | Light review only — no new topics |

---

## SATURDAY (Today) — Read + understand

**Block 1 — 60–90 min**
1. [ ] `01-START-HERE.md`
2. [ ] `02-RESUME-VS-STAFF-BAR.md`
3. [ ] `03-WHAT-THEY-WILL-ASK-FROM-RESUME.md`

**Block 2 — 2.5–3.5 hours (most important)**
4. [ ] `04-FINAL-STAFF-SDE-STUDY-GUIDE.md` — read fully once  
   Mark with highlighter / notes where you feel weak:
   - [ ] Rust Send/Sync / async
   - [ ] Proxy design
   - [ ] TLS / threat model
   - [ ] Leadership scripts

**Block 3 — 45 min**
5. [ ] `06-ZSCALER-BRIDGE-STORIES.md`
6. [ ] Write your **real incident** in 8 bullets (detect → fix → prevent)

**Saturday done when:** You finished file **04** once and know your weak boxes.

**Do tonight:** Sleep. No all-nighter.

---

## SUNDAY — Speak + draw (this is what clears the interview)

**Morning — 90 min**
1. [ ] `05-ANSWER-SCRIPTS-60MIN.md` — read once
2. [ ] Practice **Q1 ownership story** out loud **5 times** (timer 3:00 max)
3. [ ] Practice **incident story** out loud **3 times**

**Afternoon — 2 hours**
4. [ ] Draw **proxy design** on paper **5 times** while talking (control vs data plane)
5. [ ] Practice out loud:
   - Fail open vs fail closed
   - Tenant A vs Tenant B isolation
6. [ ] Practice Rust aloud (10 min each):
   - Send / Sync
   - CPU on async runtime
   - unwrap rules
   - p99 debug steps

**Evening — 60 min**
7. [ ] Security aloud:
   - TLS 1.3 + why resumption matters
   - 8 threat-model risks (no notes)
   - “Rust safe ≠ appsec”
8. [ ] Leadership aloud:
   - API contracts / cross-role impact + metrics
   - Blocked dangerous ship

**Sunday done when:** You can do ownership + design + security without reading the screen.

---

## MONDAY — Mock like the real interview

**Morning — 30 min**
1. [ ] `07-HOW-THEY-WILL-SCORE-YOU.md`
2. [ ] `09-QUESTIONS-AND-80L-POSITIONING.md` — pick 3 questions to ask them

**Midday — 70 min FULL MOCK**
3. [ ] Phone timer 60:00  
4. [ ] Friend asks from file 07 question list **OR** you self-run in order  
5. [ ] Score yourself harshly with file 07 rules (need ≥18 feel)

**Afternoon — 90 min (only weak parts)**
6. [ ] If Rust weak → `10-OPTIONAL-RUST-EXTRA.md`  
7. [ ] If design weak → `11-OPTIONAL-DESIGN-EXTRA.md`  
8. [ ] If security weak → `12-OPTIONAL-SECURITY-EXTRA.md` (**priority for Zscaler**)  
9. [ ] Re-practice only the failed answers out loud

**Evening — 30 min**
10. [ ] `08-7DAY-PREP-AND-DAY-OF.md` — day-of section only  
11. [ ] Pack: resume PDF, charger, water, quiet room plan, paper for drawing  
12. [ ] Sleep early

**Monday rule:** No new big topics after 8 PM.

---

## TUESDAY — Interview day

**90 min before**
- [ ] Skim `05-ANSWER-SCRIPTS-60MIN.md` headers only (do not re-read all of 04)
- [ ] Say ownership story **once** out loud
- [ ] Draw proxy **once** on paper
- [ ] List 8 threats once from memory
- [ ] Review your 3 questions for them

**During call**
- First answer uses **“I owned…”** in 20 seconds
- Every claim: number or invariant
- Design: clarify → draw → fail mode → tenancy
- If unsure: framework + honest “I won’t invent”

**Do not**
- Cram new TLS RFCs that morning
- Lead with AI/LangGraph
- Invent millions of RPS

---

## Minimum success path (if behind schedule)

| If behind… | Still must finish |
|------------|-------------------|
| Sat late | File **04** |
| Sun short | Ownership + design + security out loud |
| Mon short | One 60-min mock + security drill |

Optional files **13–14** only if Monday morning free.

---

## Mindset for Tuesday

You are not trying to sound like a FAANG Staff with fake scale.  
You are trying to sound like a **Staff-thinking owner**: invariants, tenancy, failure modes, measured Rust wins, honest scale + clear 100× thinking.

**Next action right now (Saturday):** open `01-START-HERE.md` and start the Saturday Block 1.
