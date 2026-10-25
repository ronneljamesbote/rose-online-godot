//! NPC dialogs from iROSE's CON files (LIST_EVENT.STB names them, ULNGTB_CON.LTB holds their
//! text): the menus as they are, and the compiled Lua 4 script turned into condition and
//! action blocks (`rose_data::DialogCase`).
//!
//! The scripts are short functions of `if` tests on what the game's script functions answer
//! (`QF_checkQuestCondition("124-01") == 1`, `GF_getVariable(SV_LEVEL) >= 10`) that return 1
//! or 0, or call a function that does something (`QF_doQuestTrigger`, `GF_openStore`). They
//! have no loops, tables or sums, so each function is run on every path through its tests:
//! the tests on a path become a case's conditions, what it calls its actions, and what it
//! returns whether the line shows. The game's own client ran the script with a Lua 4 VM; a
//! test runs both on the same answers and checks they agree
//! (`rose-data-export/tests/dialogs_round_trip.rs`).
//!
//! Like Lua, a global the script never set reads as nil (a few scripts name one by mistake).

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, bail, Context};
use rose_data::{
    AbilityType, Dialog, DialogAction, DialogCase, DialogCondition, DialogDatabase, DialogLine, DialogLineKind,
    DialogMenu, DialogQuestValue, DialogVariable,
};
use rose_file_readers::{
    lua4::{Lua4Function, Lua4Instruction as Op},
    ConFile, ConMessageType, LtbFile, QsdConditionOperator, VirtualFilesystem,
};

use crate::{decode_item_base1000, get_npc_conversations};

/// The dialog text file and the column with the English text.
pub const DIALOG_TEXT_FILE: &str = "3DDATA/EVENT/ULNGTB_CON.LTB";
const TEXT_COLUMN: usize = 2;

/// GF_getVariable's numbers (SV_SEX ...).
const SCRIPT_VARIABLES: [(&str, AbilityType); 15] = [
    ("SV_SEX", AbilityType::Gender),
    ("SV_BIRTH", AbilityType::Birthstone),
    ("SV_CLASS", AbilityType::Job),
    ("SV_UNION", AbilityType::Union),
    ("SV_RANK", AbilityType::Rank),
    ("SV_FAME", AbilityType::Fame),
    ("SV_STR", AbilityType::Strength),
    ("SV_DEX", AbilityType::Dexterity),
    ("SV_INT", AbilityType::Intelligence),
    ("SV_CON", AbilityType::Concentration),
    ("SV_CHA", AbilityType::Charm),
    ("SV_SEN", AbilityType::Sense),
    ("SV_EXP", AbilityType::Experience),
    ("SV_LEVEL", AbilityType::Level),
    ("SV_POINT", AbilityType::BonusPoint),
];

/// The script functions the game answered. Any other one is nil to the script, so calling
/// it stops the function.
const SCRIPT_FUNCTIONS: [&str; 28] = [
    "QF_checkQuestCondition",
    "QF_doQuestTrigger",
    "QF_findQuest",
    "QF_getEventOwner",
    "QF_getEpisodeVAR",
    "QF_getJobVAR",
    "QF_getPlanetVAR",
    "QF_getQuestCount",
    "QF_getQuestID",
    "QF_getQuestItemQuantity",
    "QF_getQuestSwitch",
    "QF_getQuestVar",
    "QF_getUserSwitch",
    "QF_getNpcQuestZeroVal",
    "GF_getVariable",
    "GF_openStore",
    "GF_openBank",
    "GF_organizeClan",
    "GF_disorganizeClan",
    "GF_GetMotionUseFile",
    "GF_SetMotion",
    "GF_appraisal",
    "GF_repair",
    "GF_openUpgrade",
    "GF_openSeparate",
    "GF_openDeliveryStore",
    "GF_setRevivePosition",
    "GF_getReviveZoneName",
];

/// Quest slots a character has (QF_findQuest answers -1 or one of them).
const QUEST_SLOTS: i32 = 10;

/// A value on the script's stack.
#[derive(Clone, Debug, PartialEq)]
enum Sym {
    Nil,
    Num(f64),
    Str(String),
    /// The function's parameter: the conversation's owner.
    Owner,
    /// QF_getEventOwner(owner): the NPC we talk to.
    Npc,
    Closure(String),
    Rust(&'static str),
    /// What a script function answered (an index into `Exec::queries`).
    Val(usize),
}

#[derive(Clone, Debug, PartialEq)]
struct Query {
    function: &'static str,
    args: Vec<Sym>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cmp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Cmp {
    fn test(self, a: f64, b: f64) -> bool {
        match self {
            Cmp::Eq => a == b,
            Cmp::Ne => a != b,
            Cmp::Lt => a < b,
            Cmp::Le => a <= b,
            Cmp::Gt => a > b,
            Cmp::Ge => a >= b,
        }
    }

    fn not(self) -> Self {
        match self {
            Cmp::Eq => Cmp::Ne,
            Cmp::Ne => Cmp::Eq,
            Cmp::Lt => Cmp::Ge,
            Cmp::Le => Cmp::Gt,
            Cmp::Gt => Cmp::Le,
            Cmp::Ge => Cmp::Lt,
        }
    }

    /// `b op a` written as `a op' b`.
    fn swap(self) -> Self {
        match self {
            Cmp::Lt => Cmp::Gt,
            Cmp::Le => Cmp::Ge,
            Cmp::Gt => Cmp::Lt,
            Cmp::Ge => Cmp::Le,
            other => other,
        }
    }

    fn operator(self) -> QsdConditionOperator {
        match self {
            Cmp::Eq => QsdConditionOperator::Equals,
            Cmp::Ne => QsdConditionOperator::NotEqual,
            Cmp::Lt => QsdConditionOperator::LessThan,
            Cmp::Le => QsdConditionOperator::LessThanEqual,
            Cmp::Gt => QsdConditionOperator::GreaterThan,
            Cmp::Ge => QsdConditionOperator::GreaterThanEqual,
        }
    }
}

/// A test on a path: script answer `query` compared with `value` came out `holds`.
#[derive(Clone, Debug, PartialEq)]
struct Test {
    query: usize,
    cmp: Cmp,
    value: f64,
}

#[derive(Clone, Debug, PartialEq)]
enum Event {
    Test(Test, bool),
    Act(Query),
}

#[derive(Clone, Debug)]
enum End {
    Return(Vec<Sym>),
    /// The script stopped with an error: the game treats it as answering nothing.
    Error(String),
}

#[derive(Clone, Debug)]
struct Path {
    events: Vec<Event>,
    end: End,
    /// The globals the function set (some scripts keep a value in one instead of a local).
    globals: HashMap<String, Sym>,
}

/// What the answers to a query can be, when only a few: -1 and the quest slots for
/// QF_findQuest, 0 and 1 for a quest step check.
fn domain(query: &Query) -> Option<Vec<f64>> {
    match query.function {
        "QF_checkQuestCondition" | "QF_doQuestTrigger" => Some(vec![0.0, 1.0]),
        "QF_findQuest" => Some((-1..QUEST_SLOTS).map(f64::from).collect()),
        _ => None,
    }
}

struct Exec<'a> {
    functions: &'a HashMap<String, Arc<Lua4Function>>,
    /// Globals some function sets.
    set_globals: &'a [String],
    queries: Vec<Query>,
}

const MAX_PATHS: usize = 4096;
const MAX_CALL_DEPTH: usize = 8;

impl Exec<'_> {
    fn global(&self, name: &str, globals: &HashMap<String, Sym>) -> Result<Sym, anyhow::Error> {
        if let Some(value) = globals.get(name) {
            return Ok(value.clone());
        }
        if self.set_globals.iter().any(|g| g == name) {
            bail!("reads global {name}, which another call may have set");
        }
        Ok(self.global_value(name))
    }

    fn global_value(&self, name: &str) -> Sym {
        if let Some(i) = SCRIPT_VARIABLES.iter().position(|(n, _)| *n == name) {
            return Sym::Num(i as f64);
        }
        if self.functions.contains_key(name) {
            return Sym::Closure(name.to_string());
        }
        match SCRIPT_FUNCTIONS.iter().find(|f| **f == name) {
            Some(f) => Sym::Rust(f),
            None => Sym::Nil,
        }
    }

    fn query(&mut self, query: Query) -> usize {
        match self.queries.iter().position(|q| *q == query) {
            Some(i) => i,
            None => {
                self.queries.push(query);
                self.queries.len() - 1
            }
        }
    }

    /// Every path through function `name` called with `args`, each after `before`.
    fn run(&mut self, name: &str, args: &[Sym], before: Vec<Event>, globals: HashMap<String, Sym>, depth: usize) -> Result<Vec<Path>, anyhow::Error> {
        let function = self.functions.get(name).cloned().ok_or_else(|| anyhow!("no function {name}"))?;
        if depth > MAX_CALL_DEPTH {
            bail!("{name}: calls go too deep");
        }
        let stack: Vec<Sym> = (0..function.num_parameters as usize).map(|i| args.get(i).cloned().unwrap_or(Sym::Nil)).collect();
        let mut out = Vec::new();
        self.step(&function, 0, stack, before, globals, depth, &mut out)?;
        Ok(out)
    }

    fn step(
        &mut self,
        f: &Lua4Function,
        mut pc: usize,
        mut stack: Vec<Sym>,
        mut events: Vec<Event>,
        mut globals: HashMap<String, Sym>,
        depth: usize,
        out: &mut Vec<Path>,
    ) -> Result<(), anyhow::Error> {
        if out.len() > MAX_PATHS {
            bail!("too many paths");
        }
        let pop = |stack: &mut Vec<Sym>| stack.pop().ok_or_else(|| anyhow!("empty stack"));
        let mut steps = 0;
        loop {
            steps += 1;
            if steps > 100_000 {
                bail!("the function doesn't end");
            }
            let instruction = *f.instructions.get(pc).ok_or_else(|| anyhow!("ran past the end"))?;
            pc += 1;
            match instruction {
                Op::OP_END => {
                    out.push(Path { events, end: End::Return(Vec::new()), globals });
                    return Ok(());
                }
                Op::OP_RETURN(a) => {
                    let values = stack.get(a as usize..).map(<[Sym]>::to_vec).unwrap_or_default();
                    out.push(Path { events, end: End::Return(values), globals });
                    return Ok(());
                }
                Op::OP_PUSHNIL(n) => stack.extend(std::iter::repeat(Sym::Nil).take(n as usize)),
                Op::OP_POP(n) => {
                    let keep = stack.len().saturating_sub(n as usize);
                    stack.truncate(keep);
                }
                Op::OP_PUSHINT(v) => stack.push(Sym::Num(v as f64)),
                Op::OP_PUSHSTRING(k) => stack.push(Sym::Str(constant(&f.constant_strings, k)?.clone())),
                Op::OP_PUSHNUM(k) => stack.push(Sym::Num(*constant(&f.constant_numbers, k)?)),
                Op::OP_PUSHNEGNUM(k) => stack.push(Sym::Num(-*constant(&f.constant_numbers, k)?)),
                Op::OP_GETLOCAL(i) => stack.push(stack.get(i as usize).cloned().ok_or_else(|| anyhow!("no local {i}"))?),
                Op::OP_SETLOCAL(i) => {
                    let value = pop(&mut stack)?;
                    *stack.get_mut(i as usize).ok_or_else(|| anyhow!("no local {i}"))? = value;
                }
                Op::OP_GETGLOBAL(k) => stack.push(self.global(constant(&f.constant_strings, k)?, &globals)?),
                Op::OP_JMP(s) => pc = jump(pc, s)?,
                Op::OP_CALL(a, results) => {
                    let base = a as usize;
                    if base >= stack.len() {
                        bail!("call past the stack");
                    }
                    let args = stack.split_off(base + 1);
                    let callee = pop(&mut stack)?;
                    let push = |stack: &mut Vec<Sym>, values: &[Sym]| {
                        for i in 0..results as usize {
                            stack.push(values.get(i).cloned().unwrap_or(Sym::Nil));
                        }
                    };
                    match callee {
                        Sym::Closure(name) => {
                            for sub in self.run(&name, &args, events.clone(), globals.clone(), depth + 1)? {
                                match sub.end {
                                    End::Error(_) => out.push(sub),
                                    End::Return(values) => {
                                        let mut stack = stack.clone();
                                        push(&mut stack, &values);
                                        self.step(f, pc, stack, sub.events, sub.globals, depth, out)?;
                                    }
                                }
                            }
                            return Ok(());
                        }
                        Sym::Rust(function) => {
                            let query = Query { function, args };
                            match function {
                                "QF_getEventOwner" => {
                                    let value = if query.args.first() == Some(&Sym::Owner) { Sym::Npc } else { Sym::Num(0.0) };
                                    push(&mut stack, &[value]);
                                }
                                // Checks the step; when it passes, asks for it and answers 1.
                                "QF_doQuestTrigger" => {
                                    // The same answer as checking the step.
                                    let id = self.query(Query { function: "QF_checkQuestCondition", args: query.args.clone() });
                                    for passes in [true, false] {
                                        let mut events = events.clone();
                                        events.push(Event::Test(Test { query: id, cmp: Cmp::Eq, value: 1.0 }, passes));
                                        if passes {
                                            events.push(Event::Act(query.clone()));
                                        }
                                        let mut stack = stack.clone();
                                        push(&mut stack, &[Sym::Num(if passes { 1.0 } else { 0.0 })]);
                                        self.step(f, pc, stack, events, globals.clone(), depth, out)?;
                                    }
                                    return Ok(());
                                }
                                "QF_checkQuestCondition" | "QF_findQuest" | "QF_getEpisodeVAR" | "QF_getJobVAR"
                                | "QF_getPlanetVAR" | "QF_getQuestCount" | "QF_getQuestID" | "QF_getQuestItemQuantity"
                                | "QF_getQuestSwitch" | "QF_getQuestVar" | "QF_getUserSwitch" | "QF_getNpcQuestZeroVal"
                                | "GF_getVariable" | "GF_getReviveZoneName" | "GF_GetMotionUseFile" => {
                                    // A quest value of no quest slot (a global never set):
                                    // the game answered -1.
                                    if matches!(function, "QF_getQuestSwitch" | "QF_getQuestVar") && query.args.first() == Some(&Sym::Nil) {
                                        push(&mut stack, &[Sym::Num(-1.0)]);
                                        continue;
                                    }
                                    let id = self.query(query);
                                    push(&mut stack, &[Sym::Val(id)]);
                                }
                                _ => {
                                    events.push(Event::Act(query));
                                    push(&mut stack, &[]);
                                }
                            }
                        }
                        other => {
                            out.push(Path { events, end: End::Error(format!("calls {other:?}")), globals });
                            return Ok(());
                        }
                    }
                }
                Op::OP_JMPNE(s) | Op::OP_JMPEQ(s) | Op::OP_JMPLT(s) | Op::OP_JMPLE(s) | Op::OP_JMPGT(s) | Op::OP_JMPGE(s) => {
                    let rhs = pop(&mut stack)?;
                    let lhs = pop(&mut stack)?;
                    let cmp = match instruction {
                        Op::OP_JMPNE(_) => Cmp::Ne,
                        Op::OP_JMPEQ(_) => Cmp::Eq,
                        Op::OP_JMPLT(_) => Cmp::Lt,
                        Op::OP_JMPLE(_) => Cmp::Le,
                        Op::OP_JMPGT(_) => Cmp::Gt,
                        _ => Cmp::Ge,
                    };
                    let target = jump(pc, s)?;
                    let test = match (&lhs, &rhs) {
                        (Sym::Val(q), Sym::Num(n)) => Test { query: *q, cmp, value: *n },
                        (Sym::Num(n), Sym::Val(q)) => Test { query: *q, cmp: cmp.swap(), value: *n },
                        (Sym::Val(_), _) | (_, Sym::Val(_)) => bail!("compares {lhs:?} with {rhs:?}"),
                        _ => {
                            if fixed_compare(&lhs, cmp, &rhs) {
                                pc = target;
                            }
                            continue;
                        }
                    };
                    let known = domain(&self.queries[test.query])
                        .map(|values| values.iter().map(|v| test.cmp.test(*v, test.value)).collect::<Vec<_>>());
                    if let Some(known) = known.filter(|k| k.iter().all(|t| *t == k[0])) {
                        if known[0] {
                            pc = target;
                        }
                        continue;
                    }
                    for jumps in [false, true] {
                        let mut events = events.clone();
                        events.push(Event::Test(test.clone(), jumps));
                        if possible(&self.queries, &events) {
                            self.step(f, if jumps { target } else { pc }, stack.clone(), events, globals.clone(), depth, out)?;
                        }
                    }
                    return Ok(());
                }
                Op::OP_SETGLOBAL(k) => {
                    let value = pop(&mut stack)?;
                    globals.insert(constant(&f.constant_strings, k)?.clone(), value);
                }
                other => bail!("instruction {other:?}"),
            }
        }
    }
}

fn constant<T>(list: &[T], k: u32) -> Result<&T, anyhow::Error> {
    list.get(k as usize).ok_or_else(|| anyhow!("no constant {k}"))
}

fn jump(pc: usize, by: i32) -> Result<usize, anyhow::Error> {
    usize::try_from(pc as i64 + by as i64).map_err(|_| anyhow!("jumps before the start"))
}

/// A comparison of two values the script made itself, as the game's Lua 4 VM compares
/// (values of different kinds are never equal, and never less or greater).
fn fixed_compare(a: &Sym, cmp: Cmp, b: &Sym) -> bool {
    match (a, b) {
        (Sym::Num(a), Sym::Num(b)) => cmp.test(*a, *b),
        (Sym::Str(a), Sym::Str(b)) => match cmp {
            Cmp::Eq => a == b,
            Cmp::Ne => a != b,
            Cmp::Lt => a < b,
            Cmp::Le => a <= b,
            Cmp::Gt => a > b,
            Cmp::Ge => a >= b,
        },
        (Sym::Nil, Sym::Nil) => cmp == Cmp::Eq,
        (Sym::Npc, _) | (_, Sym::Npc) => false,
        _ => cmp == Cmp::Ne,
    }
}

/// Whether some answers pass every test on the path (whole numbers, within the query's
/// domain when it has one).
fn possible(queries: &[Query], events: &[Event]) -> bool {
    let mut tests: HashMap<usize, Vec<(Cmp, f64, bool)>> = HashMap::new();
    for event in events {
        if let Event::Test(t, holds) = event {
            tests.entry(t.query).or_default().push((t.cmp, t.value, *holds));
        }
    }
    tests.iter().all(|(query, tests)| {
        let candidates = domain(&queries[*query]).unwrap_or_else(|| {
            let mut c: Vec<f64> = tests.iter().flat_map(|t| [t.1 - 1.0, t.1, t.1 + 1.0]).collect();
            c.extend([-1e9, 1e9]);
            c
        });
        candidates.iter().any(|v| tests.iter().all(|(cmp, value, holds)| cmp.test(*v, *value) == *holds))
    })
}

/// What a function's answer is used for.
#[derive(Clone, Copy, PartialEq)]
enum Role {
    /// A line's condition: shows when the answer isn't 0.
    Show,
    /// The dialog's opening check: talks when the answer is 1 or more.
    Open,
    /// An answer's action: the answer isn't used.
    Act,
}

/// A path's events as a tree, so a case can leave out what an earlier case already ruled
/// out.
enum Tree {
    Act(Query, Box<Tree>),
    Test(Test, Box<Tree>, Box<Tree>),
    End(End),
}

fn tree(paths: &[&Path], at: usize) -> Result<Tree, anyhow::Error> {
    let first = paths.first().ok_or_else(|| anyhow!("no path"))?;
    match first.events.get(at) {
        None => {
            if paths.len() != 1 {
                bail!("paths that end the same way");
            }
            Ok(Tree::End(first.end.clone()))
        }
        Some(Event::Act(q)) => {
            if paths.iter().any(|p| p.events.get(at) != Some(&Event::Act(q.clone()))) {
                bail!("paths that part on an action");
            }
            Ok(Tree::Act(q.clone(), Box::new(tree(paths, at + 1)?)))
        }
        Some(Event::Test(t, _)) => {
            let side = |holds: bool| -> Vec<&Path> {
                paths.iter().copied().filter(|p| p.events.get(at) == Some(&Event::Test(t.clone(), holds))).collect()
            };
            let (yes, no) = (side(true), side(false));
            if yes.len() + no.len() != paths.len() {
                bail!("paths that part on different tests");
            }
            let branch = |p: Vec<&Path>| -> Result<Tree, anyhow::Error> {
                if p.is_empty() {
                    // That side can't happen: an error stand-in no case reaches.
                    Ok(Tree::End(End::Error("can't happen".into())))
                } else {
                    tree(&p, at + 1)
                }
            };
            Ok(Tree::Test(t.clone(), Box::new(branch(yes)?), Box::new(branch(no)?)))
        }
    }
}

/// A function's tree with its actions gathered at the ends: each end what the function does
/// (`actions`) and whether the line shows, on the answers the tests above it saw.
enum Outcome {
    Test(Test, Box<Outcome>, Box<Outcome>),
    End { actions: Vec<Query>, show: bool, error: Option<String> },
}

fn outcomes(tree: &Tree, role: Role, actions: &mut Vec<Query>) -> Outcome {
    match tree {
        Tree::Act(query, next) => {
            actions.push(query.clone());
            let out = outcomes(next, role, actions);
            actions.pop();
            out
        }
        Tree::Test(test, yes, no) => {
            Outcome::Test(test.clone(), Box::new(outcomes(yes, role, actions)), Box::new(outcomes(no, role, actions)))
        }
        Tree::End(End::Error(e)) => Outcome::End { actions: actions.clone(), show: false, error: Some(e.clone()) },
        Tree::End(End::Return(values)) => {
            let end = |show: bool| Outcome::End { actions: actions.clone(), show: show && role != Role::Act, error: None };
            // The game reads the answer as a whole number: 1 or more opens the dialog, not 0
            // shows a line.
            let passes = |n: f64| if role == Role::Open { n >= 1.0 } else { n.trunc() != 0.0 };
            match values.first() {
                Some(Sym::Num(n)) => end(passes(*n)),
                Some(Sym::Str(s)) => end(s.trim().parse::<f64>().is_ok_and(passes)),
                Some(Sym::Val(q)) if role != Role::Act => {
                    let test = match role {
                        Role::Open => Test { query: *q, cmp: Cmp::Ge, value: 1.0 },
                        _ => Test { query: *q, cmp: Cmp::Ne, value: 0.0 },
                    };
                    Outcome::Test(test, Box::new(end(true)), Box::new(end(false)))
                }
                _ => end(false),
            }
        }
    }
}

/// What an end does as the game sees it, for the answers `values` (one per query): whether
/// the line shows and the actions, a quest step only when it passes (running one checks it
/// first anyway).
fn seen(queries: &[Query], values: &[f64], actions: &[Query], show: bool) -> (bool, Vec<Query>) {
    let passes = |q: &Query| {
        queries
            .iter()
            .position(|x| x.function == "QF_checkQuestCondition" && x.args == q.args)
            .map_or(true, |i| values[i] == 1.0)
    };
    let kept = actions.iter().filter(|a| a.function != "QF_doQuestTrigger" || passes(a)).cloned().collect();
    (show, kept)
}

fn end_for<'a>(outcome: &'a Outcome, values: &[f64]) -> &'a Outcome {
    match outcome {
        Outcome::Test(t, yes, no) => end_for(if t.cmp.test(values[t.query], t.value) { yes } else { no }, values),
        end => end,
    }
}

/// The answers worth trying for each query: its domain, or the numbers it is compared with
/// and one either side, and two far away.
fn candidates(queries: &[Query], outcome: &Outcome) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = queries.iter().map(|q| domain(q).unwrap_or_else(|| vec![-1e9, 1e9])).collect();
    fn walk(o: &Outcome, queries: &[Query], out: &mut Vec<Vec<f64>>) {
        if let Outcome::Test(t, yes, no) = o {
            if domain(&queries[t.query]).is_none() {
                out[t.query].extend([t.value - 1.0, t.value, t.value + 1.0]);
            }
            walk(yes, queries, out);
            walk(no, queries, out);
        }
    }
    walk(outcome, queries, &mut out);
    for c in &mut out {
        c.sort_by(f64::total_cmp);
        c.dedup();
    }
    out
}

/// Every combination of answers, when there aren't too many to try.
const MAX_STATES: usize = 1 << 20;

fn states(candidates: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let count = candidates.iter().try_fold(1usize, |n, c| n.checked_mul(c.len()).filter(|n| *n <= MAX_STATES))?;
    let mut out = Vec::with_capacity(count);
    let mut at = vec![0usize; candidates.len()];
    loop {
        out.push(at.iter().enumerate().map(|(q, i)| candidates[q][*i]).collect());
        let mut q = 0;
        loop {
            if q == at.len() {
                return Some(out);
            }
            at[q] += 1;
            if at[q] < candidates[q].len() {
                break;
            }
            at[q] = 0;
            q += 1;
        }
    }
}

/// One way to an end: the tests on it (query, comparison, came out) and the end.
fn ways<'a>(outcome: &'a Outcome, tests: &mut Vec<(Test, bool)>, out: &mut Vec<(Vec<(Test, bool)>, &'a Outcome)>) {
    match outcome {
        Outcome::Test(t, yes, no) => {
            tests.push((t.clone(), true));
            ways(yes, tests, out);
            tests.pop();
            tests.push((t.clone(), false));
            ways(no, tests, out);
            tests.pop();
        }
        end => out.push((tests.clone(), end)),
    }
}

/// One function turned into dialog cases.
struct Translate<'a> {
    queries: &'a [Query],
    notes: &'a mut Vec<String>,
    function: &'a str,
}

impl Translate<'_> {
    fn condition(&mut self, test: &Test, holds: bool) -> Result<DialogCondition, anyhow::Error> {
        let query = &self.queries[test.query];
        let cmp = if holds { test.cmp } else { test.cmp.not() };
        let value = test.value;
        if value.fract() != 0.0 || value.abs() > i32::MAX as f64 {
            bail!("compares with {value}");
        }
        let (operator, value) = (cmp.operator(), value as i32);
        let num = |i: usize| -> Result<f64, anyhow::Error> {
            match query.args.get(i) {
                Some(Sym::Num(n)) if n.fract() == 0.0 && *n >= 0.0 => Ok(*n),
                other => bail!("{} with {other:?}", query.function),
            }
        };
        let text = |i: usize| -> Result<String, anyhow::Error> {
            match query.args.get(i) {
                Some(Sym::Str(s)) => Ok(s.clone()),
                other => bail!("{} with {other:?}", query.function),
            }
        };
        // A quest's slot, as QF_findQuest answered it: the quest.
        let slot_quest = |i: usize| -> Result<u16, anyhow::Error> {
            match query.args.get(i) {
                Some(Sym::Val(q)) if self.queries[*q].function == "QF_findQuest" => match self.queries[*q].args.first() {
                    Some(Sym::Num(n)) if n.fract() == 0.0 && *n >= 0.0 => Ok(*n as u16),
                    other => bail!("QF_findQuest with {other:?}"),
                },
                other => bail!("{} on slot {other:?}", query.function),
            }
        };
        Ok(match query.function {
            "QF_checkQuestCondition" => DialogCondition::QuestStep { trigger: text(0)?, passes: cmp.test(1.0, test.value) },
            "QF_findQuest" => {
                let missing = cmp.test(-1.0, test.value);
                let slots: Vec<bool> = (0..QUEST_SLOTS).map(|s| cmp.test(s as f64, test.value)).collect();
                if slots.iter().any(|s| *s == missing) || slots.iter().any(|s| *s != slots[0]) {
                    bail!("QF_findQuest {cmp:?} {value}: compares a quest's slot");
                }
                DialogCondition::Quest { quest: num(0)? as u16, active: slots[0] }
            }
            "QF_getQuestItemQuantity" => {
                let item = decode_item_base1000(num(1)? as usize).ok_or_else(|| anyhow!("no item {}", num(1).unwrap_or(0.0)))?;
                DialogCondition::QuestItem { quest: num(0)? as u16, item, operator, value }
            }
            "GF_getVariable" => {
                let ability = SCRIPT_VARIABLES.get(num(0)? as usize).ok_or_else(|| anyhow!("no script variable"))?.1;
                DialogCondition::Ability { ability, operator, value }
            }
            "QF_getNpcQuestZeroVal" => {
                if query.args.first() != Some(&Sym::Npc) {
                    bail!("QF_getNpcQuestZeroVal of {:?}", query.args.first());
                }
                DialogCondition::NpcSwitch { operator, value }
            }
            "QF_getUserSwitch" => DialogCondition::QuestSwitch { switch: num(0)? as u16, operator, value },
            "QF_getEpisodeVAR" | "QF_getJobVAR" | "QF_getPlanetVAR" => DialogCondition::Variable {
                variable: match query.function {
                    "QF_getEpisodeVAR" => DialogVariable::Episode,
                    "QF_getJobVAR" => DialogVariable::Job,
                    _ => DialogVariable::Planet,
                },
                index: num(0)? as u16,
                operator,
                value,
            },
            "QF_getQuestSwitch" | "QF_getQuestVar" => DialogCondition::QuestValue {
                quest: slot_quest(0)?,
                value_type: if query.function == "QF_getQuestSwitch" { DialogQuestValue::Switch } else { DialogQuestValue::Variable },
                index: num(1)? as u16,
                operator,
                value,
            },
            "QF_getQuestCount" => DialogCondition::QuestCount { operator, value },
            other => bail!("tests {other}"),
        })
    }

    fn action(&mut self, query: &Query) -> Result<Option<DialogAction>, anyhow::Error> {
        Ok(Some(match query.function {
            "QF_doQuestTrigger" => match query.args.first() {
                Some(Sym::Str(s)) => DialogAction::QuestStep { trigger: s.clone() },
                other => bail!("QF_doQuestTrigger with {other:?}"),
            },
            "GF_openStore" => {
                if query.args.first() != Some(&Sym::Npc) {
                    bail!("GF_openStore of {:?}", query.args.first());
                }
                DialogAction::OpenShop
            }
            "GF_openBank" => DialogAction::OpenStorage,
            "GF_openUpgrade" => DialogAction::OpenRefining,
            "GF_openSeparate" => DialogAction::OpenDisassembly,
            "GF_repair" => DialogAction::OpenRepair,
            "GF_setRevivePosition" => DialogAction::SetSavePoint,
            "GF_appraisal" => DialogAction::Appraisal,
            "GF_openDeliveryStore" => DialogAction::OpenDelivery,
            "GF_organizeClan" => DialogAction::FoundClan,
            "GF_disorganizeClan" => DialogAction::DisbandClan,
            // How the NPC moves while talking: the game doesn't show it.
            "GF_SetMotion" => {
                self.notes.push(format!("{}: NPC motion left out", self.function));
                return Ok(None);
            }
            other => bail!("calls {other}"),
        }))
    }

    /// The cases: one for each end that does something (shows, or acts), with only the tests
    /// it needs: a test is left out when every answer passing the others still ends the same
    /// way. Ends that hide and do nothing need no case (no case passing hides the line).
    fn cases(&mut self, outcome: &Outcome) -> Result<Vec<DialogCase>, anyhow::Error> {
        let mut all = Vec::new();
        ways(outcome, &mut Vec::new(), &mut all);
        let states = states(&candidates(self.queries, outcome));
        let mut seen_at: Vec<(Vec<f64>, (bool, Vec<Query>))> = Vec::new();
        if let Some(states) = &states {
            for values in states {
                let Outcome::End { actions, show, .. } = end_for(outcome, values) else { unreachable!() };
                let s = seen(self.queries, values, actions, *show);
                seen_at.push((values.clone(), s));
            }
        }
        let mut found: Vec<(Vec<(Test, bool)>, (bool, Vec<Query>))> = Vec::new();
        for (tests, end) in all {
            let Outcome::End { actions, show, error } = end else { unreachable!() };
            if let Some(e) = error.as_ref().filter(|e| *e != "can't happen") {
                self.notes.push(format!("{}: stops with an error ({e}), as in the game", self.function));
            }
            if error.as_deref() == Some("can't happen") || (!*show && actions.is_empty()) {
                continue;
            }
            let goal = (*show, actions.clone());
            let mut tests = tests;
            if states.is_some() {
                // Every answer this end sees does the same with the quest steps it runs
                // checked, so compare with that.
                let same = |tests: &[(Test, bool)]| {
                    seen_at.iter().filter(|(v, _)| tests.iter().all(|(t, h)| t.cmp.test(v[t.query], t.value) == *h)).all(|(v, s)| {
                        let want = seen(self.queries, v, &goal.1, goal.0);
                        *s == want
                    })
                };
                let mut i = 0;
                while i < tests.len() {
                    let mut fewer = tests.clone();
                    fewer.remove(i);
                    if same(&fewer) {
                        tests = fewer;
                    } else {
                        i += 1;
                    }
                }
            }
            // An end already covered by a case doing the same with fewer tests adds nothing.
            if found.iter().any(|(t, g)| *g == goal && t.iter().all(|x| tests.contains(x))) {
                continue;
            }
            found.retain(|(t, g)| !(*g == goal && tests.iter().all(|x| t.contains(x))));
            found.push((tests, goal));
        }
        let mut cases = Vec::new();
        for (tests, (show, actions)) in found {
            let conditions = tests.iter().map(|(t, h)| self.condition(t, *h)).collect::<Result<_, _>>()?;
            let mut out = Vec::new();
            for a in &actions {
                out.extend(self.action(a)?);
            }
            if !show && out.is_empty() {
                continue;
            }
            cases.push(DialogCase { conditions, actions: out, show });
        }
        Ok(cases)
    }
}

/// None: a line's (or the opening) condition that never shows and does nothing.
fn simplify(cases: Vec<DialogCase>, role: Role) -> Option<Vec<DialogCase>> {
    if role == Role::Act {
        return Some(cases);
    }
    if cases.is_empty() {
        return None;
    }
    if cases.iter().any(|c| c.conditions.is_empty() && c.actions.is_empty() && c.show) && cases.iter().all(|c| c.actions.is_empty()) {
        return Some(Vec::new());
    }
    Some(cases)
}

/// A CON file's script: its functions, each turned into cases on request.
pub struct DialogScript {
    functions: HashMap<String, Arc<Lua4Function>>,
    /// Globals some function sets.
    set_globals: Vec<String>,
}

/// What a script function's answer is used for (`DialogScript::cases`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogFunctionUse {
    /// A line's condition: shows when the answer isn't 0.
    Line,
    /// The dialog's opening check: talks when the answer is 1 or more.
    Opening,
    /// An answer's action: the answer isn't used.
    Action,
}

impl DialogScript {
    pub fn load(con: &ConFile) -> Result<Self, anyhow::Error> {
        let main = Lua4Function::from_bytes(&con.script_binary).context("the script")?;
        let mut functions = HashMap::new();
        let mut pending = None;
        for instruction in &main.instructions {
            match *instruction {
                Op::OP_CLOSURE(k, _) => pending = main.constant_functions.get(k as usize).cloned(),
                Op::OP_SETGLOBAL(k) => {
                    if let (Some(f), Some(name)) = (pending.take(), main.constant_strings.get(k as usize)) {
                        functions.insert(name.clone(), f);
                    }
                }
                Op::OP_END => {}
                other => bail!("the script's main part runs {other:?}"),
            }
        }
        let mut set_globals = Vec::new();
        for f in &main.constant_functions {
            for instruction in &f.instructions {
                if let Op::OP_SETGLOBAL(k) = *instruction {
                    set_globals.extend(f.constant_strings.get(k as usize).cloned());
                }
            }
        }
        Ok(Self { functions, set_globals })
    }

    /// Function `function` as cases. None: a condition that never passes and does nothing
    /// (no function name: always passes, `Some([])`). `label` names it in `notes`.
    pub fn cases(
        &self,
        function: &str,
        used: DialogFunctionUse,
        label: &str,
        notes: &mut Vec<String>,
    ) -> Result<Option<Vec<DialogCase>>, anyhow::Error> {
        let role = match used {
            DialogFunctionUse::Line => Role::Show,
            DialogFunctionUse::Opening => Role::Open,
            DialogFunctionUse::Action => Role::Act,
        };
        if function.is_empty() {
            return Ok(Some(Vec::new()));
        }
        if !self.functions.contains_key(function) {
            notes.push(format!("{label}: {function} isn't in the script, as in the game"));
            return Ok(if role == Role::Act { Some(Vec::new()) } else { None });
        }
        let label = format!("{label} {function}");
        let mut exec = Exec { functions: &self.functions, set_globals: &self.set_globals, queries: Vec::new() };
        let paths = exec.run(function, &[Sym::Owner], Vec::new(), HashMap::new(), 0).with_context(|| label.clone())?;
        let refs: Vec<&Path> = paths.iter().collect();
        let tree = tree(&refs, 0).with_context(|| label.clone())?;
        let outcome = outcomes(&tree, role, &mut Vec::new());
        let cases = Translate { queries: &exec.queries, notes, function: &label }.cases(&outcome).with_context(|| label.clone())?;
        Ok(simplify(cases, role))
    }
}

/// One CON file as a dialog. `notes` gets what was left out or works as in iROSE only.
pub fn dialog_from_con(key: &str, name: &str, con: &ConFile, text: &LtbFile, notes: &mut Vec<String>) -> Result<Dialog, anyhow::Error> {
    let script = DialogScript::load(con)?;
    let mut translate = |function: &str, role: Role| -> Result<Option<Vec<DialogCase>>, anyhow::Error> {
        let used = match role {
            Role::Show => DialogFunctionUse::Line,
            Role::Open => DialogFunctionUse::Opening,
            Role::Act => DialogFunctionUse::Action,
        };
        script.cases(function, used, key, notes)
    };

    let open_check = con.initial_messages.first().map(|m| m.condition_function.as_str()).unwrap_or("");
    let Some(open_if) = translate(open_check, Role::Open)? else {
        bail!("{key}: the NPC never talks");
    };

    // Every line as it is, then the menus nothing reaches are left out.
    let mut skipped = Vec::new();
    let mut menus = Vec::new();
    for (m, menu) in con.menus.iter().enumerate() {
        let mut lines = Vec::new();
        for message in &menu.messages {
            let Some(show_if) = translate(&message.condition_function, Role::Show)? else { continue };
            let Some(line_text) = text.get_string(message.string_id as usize, TEXT_COLUMN) else {
                if show_if.iter().any(|c| !c.actions.is_empty()) {
                    bail!("{key} menu {m}: a line with no text does something");
                }
                skipped.push(format!("{key} menu {m}: a line with no text left out (the game skipped it)"));
                continue;
            };
            let (kind, next_menu) = match message.message_type {
                ConMessageType::NextMessage | ConMessageType::ShowMessage => (DialogLineKind::Npc, message.message_value),
                ConMessageType::PlayerSelect | ConMessageType::JumpSelect | ConMessageType::Close => {
                    (DialogLineKind::Answer, message.message_value)
                }
            };
            let on_choose = if kind == DialogLineKind::Answer {
                translate(&message.action_function, Role::Act)?.unwrap_or_default()
            } else {
                Vec::new()
            };
            let next_menu = usize::try_from(next_menu).ok().filter(|n| *n < con.menus.len());
            // The text table ends some lines with a NUL, which Godot's strings can't hold.
            let text = line_text.replace('\0', "");
            lines.push(DialogLine { kind, text, show_if, on_choose, next_menu });
        }
        menus.push(DialogMenu { lines });
    }
    drop(translate);
    notes.extend(skipped);
    let mut reached = vec![false; menus.len()];
    let mut todo = vec![0usize];
    while let Some(m) = todo.pop() {
        if m >= menus.len() || reached[m] {
            continue;
        }
        reached[m] = true;
        todo.extend(menus[m].lines.iter().filter_map(|l| l.next_menu));
    }
    let mut renumber = vec![None; menus.len()];
    let mut kept = Vec::new();
    for (m, menu) in menus.into_iter().enumerate() {
        if reached[m] {
            renumber[m] = Some(kept.len());
            kept.push(menu);
        }
    }
    for menu in &mut kept {
        for line in &mut menu.lines {
            line.next_menu = line.next_menu.and_then(|n| renumber[n]);
        }
    }
    Ok(Dialog { key: key.to_string(), name: name.to_string(), open_if, menus: kept })
}

/// Every dialog LIST_EVENT.STB names whose CON file is there and can be turned into blocks.
/// The second list says what was left out (dialogs that couldn't be, and parts of others).
pub fn get_dialog_database(vfs: &VirtualFilesystem) -> Result<(DialogDatabase, Vec<String>), anyhow::Error> {
    let text = vfs.read_file::<LtbFile, _>(DIALOG_TEXT_FILE)?;
    let mut notes = Vec::new();
    let mut dialogs = Vec::new();
    let mut files: Vec<_> = get_npc_conversations(vfs)?.into_iter().collect();
    files.sort_by(|a, b| a.0.cmp(&b.0));
    for (key, data) in files {
        let Ok(con) = vfs.read_file::<ConFile, _>(data.filename.as_str()) else { continue };
        match dialog_from_con(&key, &data.name, &con, &text, &mut notes) {
            Ok(dialog) => dialogs.push(dialog),
            Err(e) => notes.push(format!("{key}: not converted: {e:#}")),
        }
    }
    Ok((DialogDatabase::new(dialogs), notes))
}
