# 00 — READ THIS FIRST
# How to Study Fast (Human Method) — Do Not Memorize the Whole Book

**Open this before every other file.**  
If you try to memorize all `.md` files word-for-word, you will fail. Staff engineers, CTOs, and strong CEOs do **not** study that way.

---

## The only goal before Tuesday

Be able to **speak 6 answers out loud in your own words**.  
Not recite 400 lines. Not finish every optional file.

---

# PART 1 — PROVEN METHOD (WHAT STRONG LEADERS ACTUALLY DO)

Big CTOs / Staff engineers / strong operators use the same learning loop:

## 1) Active recall (not re-reading)
Close the doc. Try to speak the answer.  
Re-reading feels safe but creates fake memory.  
**Speaking from memory creates real memory.**

## 2) Feynman method (explain simply)
If you cannot explain it in simple English to a friend, you do not know it yet.  
Example: don’t say “Send trait semantics.” Say “Can this value move to another thread safely?”

## 3) Spaced repetition (few reps across days)
Better: 3 days × few reps  
Worse: 1 night × 50 panicked reads

## 4) Teach-back / mock
One full mock interview beats 5 silent readings.

## 5) Compression
Compress a long answer into **5 bullets**.  
If you remember the bullets, you can rebuild the speech live.

## 6) Recovery > perfection
Experts prepare a line for when they forget.  
Juniors panic. Staff recover.

**Your recovery line:**
> “Let me structure this from my WMS path: goal → design → failure → metric.”

---

# PART 2 — HOW VICKY SHOULD READ THESE DOCS

## Do this for every section

| Step | What you do | Time |
|------|-------------|------|
| 1 | Read silently **once** to understand | 5–15 min |
| 2 | Close the file | 5 sec |
| 3 | Speak out loud in **your own words** | 1–2 min |
| 4 | Open file, fix only missing points | 2 min |
| 5 | Speak again **1–2 times** | 2–4 min |
| STOP | Do not do 20 reps of the same paragraph | — |

## Do NOT do this
- Read every file 10 times silently  
- Memorize full scripts like an actor  
- Open random Google/YouTube research this weekend  
- Study optional files before core 6 answers  

## Silent vs loud
- **Silent read** = understand  
- **Loud speak** = remember for interview  
If you only read in your mind, you will blank in the call.

---

# PART 3 — SECRET TO UNDERSTAND FAST

## Secret 1 — Only 6 answers matter
Ignore 90% of detail until these 6 are strong:

1. Who I am (45 sec)  
2. WMS zero-to-one ownership (3 min)  
3. Rust: Send/Sync + async + unwrap (2 min)  
4. Proxy design while drawing (5 min)  
5. Security: 8 threats + honest depth (2 min)  
6. Leadership + blocked unsafe ship (2 min)  

## Secret 2 — Remember “shape,” not sentences
Every answer uses:
**Point → Why → WMS example → Failure → Metric**

## Secret 3 — Remember 5 WMS invariants (these unlock many answers)
1. Stock must stay correct under concurrency  
2. Facility/tenant isolation always  
3. PostgreSQL is stock source of truth (not Redis)  
4. Heavy jobs off request path  
5. Missing authz context = deny  

## Secret 4 — Remember only these numbers
- P99 **800 → 120**  
- **10K+** tx/day  
- **5+** facilities  
- **5K+** jobs/day  
- **40+** APIs  
- Deploy **45 → 8** min  

## Secret 5 — Map unknown questions back to WMS
Unknown Zscaler topic → say framework → map to WMS hot path / tenancy / jobs-off-path / nginx-Cloudflare layers.

## Secret 6 — Honesty is a weapon
If security is deep beyond you:
> “I own production tenant isolation (JWT/RBAC/ABAC/RLS). I’m not a cryptographer. Default deny + threat checklist. I’ll deepen here.”

---

# PART 4 — HOW MANY TIMES (SAT → TUE)

For each of the **6 core answers**:

| Day | Reps (speak out loud) |
|-----|------------------------|
| Saturday | Understand + speak **2×** (identity + ownership first) |
| Sunday | Speak each core answer **3×** |
| Monday | Full mock **1×** + weak answers **2×** |
| Tuesday morning | Speak identity + ownership + 8 threats **1×** only |

**Total per core answer ≈ 6–8 spoken reps.**  
That is enough. More creates fatigue and confusion.

---

# PART 5 — DAILY MICRO ROUTINE (45–90 MIN BLOCKS)

## Block template
1. Pick 1 core answer  
2. Read once  
3. Close doc  
4. Speak aloud  
5. Check  
6. Speak aloud again  
7. Write 5 bullets on paper from memory  
8. Move on  

## Sunday special
Draw proxy diagram **5 times** while talking.  
Drawing memory is stronger than text memory.

## Monday special
One timer mock (60 min). Record yourself if possible. Listen once.

---

# PART 6 — IF YOU FORGET DURING INTERVIEW (NORMAL)

You are human. Forgetting is expected.

Do this:
1. Pause 2 seconds  
2. Say recovery line  
3. Give WMS example  
4. Ask which part to deepen  

**Emergency lines**
- “In plain words…”  
- “Three points: architecture, failure, metric.”  
- “Short answer is ___. Trade-off is ___.”  
- “Let me correct that…” (if you misspoke)

---

# PART 7 — WHAT TO OPEN AFTER THIS FILE

1. [`SATURDAY-TO-TUESDAY-PLAN.md`](SATURDAY-TO-TUESDAY-PLAN.md)  
2. [`01-START-HERE.md`](01-START-HERE.md)  
3. Then `02 → 09`  
4. Final polish: [`15-FINAL-INTERVIEW-GUIDANCE-SENIOR-STAFF-TIPS.md`](15-FINAL-INTERVIEW-GUIDANCE-SENIOR-STAFF-TIPS.md)  

**Do not start research outside this folder.**

---

# PART 8 — ONE-LINE RULE (PUT ON YOUR PHONE LOCK SCREEN)

**Read once to understand. Speak 6 answers aloud across 3 days. Remember invariants + numbers. Recover when stuck. Do not memorize the book.**
