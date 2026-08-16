use std::collections::HashMap;

// =============================================================================
// 1. SYMBOLIC EXPRESSIONS (AST za SMT Solver)
// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var(String),
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self, env: &HashMap<String, i64>) -> i64 {
        match self {
            Expr::Var(name) => *env.get(name).unwrap_or(&0),
            Expr::Const(val) => *val,
            Expr::Add(l, r) => l.eval(env) + r.eval(env),
            Expr::Sub(l, r) => l.eval(env) - r.eval(env),
            Expr::Mul(l, r) => l.eval(env) * r.eval(env),
        }
    }
}

// =============================================================================
// 2. CONSTRAINTS & INVARIANTS
// =============================================================================

#[derive(Debug, Clone)]
pub enum Constraint {
    Eq(Expr, Expr),
    LessThan(Expr, Expr),
    LessOrEq(Expr, Expr),
    GreaterThan(Expr, Expr),
    And(Box<Constraint>, Box<Constraint>),
    Not(Box<Constraint>),
}

impl Constraint {
    pub fn check(&self, env: &HashMap<String, i64>) -> bool {
        match self {
            Constraint::Eq(l, r) => l.eval(env) == r.eval(env),
            Constraint::LessThan(l, r) => l.eval(env) < r.eval(env),
            Constraint::LessOrEq(l, r) => l.eval(env) <= r.eval(env),
            Constraint::GreaterThan(l, r) => l.eval(env) > r.eval(env),
            Constraint::And(l, r) => l.check(env) && r.check(env),
            Constraint::Not(c) => !c.check(env),
        }
    }
}

// =============================================================================
// 3. SMT SOLVER & PROOF ENGINE
// =============================================================================

#[derive(Debug, PartialEq, Eq)]
pub enum ProofResult {
    ProvenSafe,                             // UNSAT: Nemoguće je razbiti invarijantu!
    CounterExampleFound(HashMap<String, i64>), // SAT: Pronađen bag!
}

pub struct SmtSolverEngine;

impl SmtSolverEngine {
    /// Proverava da li je zadata invarijanta NEOBORIVA u definisanom prostoru pretrage
    pub fn verify_invariant(
        vars: &[&str],
        domain_min: i64,
        domain_max: i64,
        invariant: Constraint,
    ) -> ProofResult {
        // Tražimo negaciju (kontraprimer): Da li postoji ulaz gde je invarijanta LAŽNA?
        let violation_target = Constraint::Not(Box::new(invariant));

        // Bounded Model Checking / Brute-Force Symbolic Search simulator
        let mut env = HashMap::new();
        if Self::search_counterexample(vars, 0, domain_min, domain_max, &mut env, &violation_target) {
            ProofResult::CounterExampleFound(env)
        } else {
            ProofResult::ProvenSafe
        }
    }

    fn search_counterexample(
        vars: &[&str],
        index: usize,
        min: i64,
        max: i64,
        env: &mut HashMap<String, i64>,
        violation: &Constraint,
    ) -> bool {
        if index == vars.len() {
            return violation.check(env);
        }

        let var_name = vars[index].to_string();
        for val in min..=max {
            env.insert(var_name.clone(), val);
            if Self::search_counterexample(vars, index + 1, min, max, env, violation) {
                return true;
            }
        }
        false
    }
}