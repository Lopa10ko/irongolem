use std::collections::HashSet;
use std::sync::{Mutex, Once, OnceLock};

use crate::golem::dag::{default_dag_rules, GraphDelegate};

static NATIVE_FUNCS: OnceLock<Mutex<HashSet<usize>>> = OnceLock::new();

fn native_registry() -> &'static Mutex<HashSet<usize>> {
    NATIVE_FUNCS.get_or_init(|| Mutex::new(HashSet::new()))
}

fn ensure_native_dag_rules() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        for rule in default_dag_rules() {
            native_registry().lock().unwrap().insert(rule as usize);
        }
    });
}

#[derive(Debug, Clone, Default)]
pub struct AdaptRegistry;

impl AdaptRegistry {
    pub fn register_native(func_ptr: usize) {
        ensure_native_dag_rules();
        native_registry().lock().unwrap().insert(func_ptr);
    }

    pub fn unregister_native(func_ptr: usize) {
        native_registry().lock().unwrap().remove(&func_ptr);
    }

    pub fn is_native(func_ptr: usize) -> bool {
        ensure_native_dag_rules();
        native_registry().lock().unwrap().contains(&func_ptr)
    }
}

pub fn register_native<T: ?Sized>(func: &T) -> usize {
    let ptr = func as *const T as *const () as usize;
    AdaptRegistry::register_native(ptr);
    ptr
}

/// Native functions (DAG rules, optimiser operators) are returned unchanged so that
/// `id(rule) == id(adapted_rule)` as in Python `BaseOptimizationAdapter.adapt_func`.
pub fn adapt_native_fn<A, R>(fun: fn(&A) -> R) -> fn(&A) -> R {
    ensure_native_dag_rules();
    fun
}

#[derive(Debug, Clone, Default)]
pub struct DirectAdapter;

impl DirectAdapter {
    pub fn adapt(&self, graph: GraphDelegate) -> std::sync::Arc<GraphDelegate> {
        std::sync::Arc::new(graph)
    }

    pub fn adapt_many(&self, graphs: Vec<GraphDelegate>) -> Vec<std::sync::Arc<GraphDelegate>> {
        graphs.into_iter().map(|g| self.adapt(g)).collect()
    }

    pub fn restore(&self, graph: std::sync::Arc<GraphDelegate>) -> GraphDelegate {
        std::sync::Arc::try_unwrap(graph).unwrap_or_else(|arc| (*arc).clone())
    }

    pub fn adapt_func<A, R>(&self, fun: fn(&A) -> R) -> fn(&A) -> R {
        adapt_native_fn(fun)
    }

    pub fn restore_func<F>(&self, fun: F) -> F {
        fun
    }
}

pub trait OptimizationAdapter: Send + Sync {
    fn adapt_graph(&self, graph: GraphDelegate) -> std::sync::Arc<GraphDelegate>;
    fn adapt_many(&self, graphs: Vec<GraphDelegate>) -> Vec<std::sync::Arc<GraphDelegate>> {
        graphs.into_iter().map(|g| self.adapt_graph(g)).collect()
    }
    fn restore_graph(&self, graph: std::sync::Arc<GraphDelegate>) -> GraphDelegate;
}

impl OptimizationAdapter for DirectAdapter {
    fn adapt_graph(&self, graph: GraphDelegate) -> std::sync::Arc<GraphDelegate> {
        self.adapt(graph)
    }

    fn restore_graph(&self, graph: std::sync::Arc<GraphDelegate>) -> GraphDelegate {
        self.restore(graph)
    }
}

pub fn adapt_with<A: OptimizationAdapter>(_adapter: &A, fun: usize) -> usize {
    let _ = AdaptRegistry::is_native(fun);
    fun
}
