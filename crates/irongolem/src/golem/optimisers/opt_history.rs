use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use serde_json::{json, Map, Value};

use super::fitness::{Fitness, MultiObjFitness, SingleObjFitness};
use super::history::{Generation, Individual, ParentOperator};
use super::objective::ObjectiveInfo;
use crate::golem::dag::{Graph, GraphDelegate, LinkedGraphNode, NodeContent};

#[derive(Debug, Clone)]
pub struct OptHistory {
    pub generations: Vec<Generation>,
    pub archive_history: Vec<Vec<String>>,
    pub objective: Option<ObjectiveInfo>,
    pub default_save_dir: String,
    individuals_pool: HashMap<String, Individual>,
}

impl Default for OptHistory {
    fn default() -> Self {
        Self::new(None)
    }
}

impl OptHistory {
    pub fn new(objective: Option<ObjectiveInfo>) -> Self {
        Self {
            generations: Vec::new(),
            archive_history: Vec::new(),
            objective,
            default_save_dir: String::new(),
            individuals_pool: HashMap::new(),
        }
    }

    pub fn add_to_history(
        &mut self,
        individuals: Vec<Individual>,
        generation_label: Option<&str>,
        generation_metadata: Option<HashMap<String, serde_json::Value>>,
    ) {
        for ind in &individuals {
            self.individuals_pool.insert(ind.uid.clone(), ind.clone());
        }
        let generation = Generation::new(
            individuals,
            self.generations.len(),
            generation_label.map(String::from),
            generation_metadata,
        );
        self.generations.push(generation);
    }

    pub fn add_to_archive_history(&mut self, individuals: &[Individual]) {
        self.archive_history
            .push(individuals.iter().map(|i| i.uid.clone()).collect());
        for ind in individuals {
            self.individuals_pool.insert(ind.uid.clone(), ind.clone());
        }
    }

    pub fn generations_count(&self) -> usize {
        self.generations.len()
    }

    pub fn initial_assumptions(&self) -> Option<&Generation> {
        self.generations
            .iter()
            .find(|g| g.label.as_deref() == Some("initial_assumptions"))
    }

    pub fn final_choices(&self) -> Option<&Generation> {
        self.generations
            .iter()
            .rev()
            .find(|g| g.label.as_deref() == Some("final_choices"))
    }

    pub fn historical_fitness(&self) -> Vec<Vec<f64>> {
        self.generations
            .iter()
            .map(|gen| {
                gen.individuals
                    .iter()
                    .map(|ind| ind.fitness.values().first().copied().unwrap_or(0.0))
                    .collect()
            })
            .collect()
    }

    pub fn historical_fitness_multi(&self) -> Vec<Vec<Vec<f64>>> {
        if self.generations.is_empty() {
            return Vec::new();
        }
        let num_metrics = self.generations[0]
            .individuals
            .first()
            .map(|i| i.fitness.values().len())
            .unwrap_or(0);
        (0..num_metrics)
            .map(|metric_idx| {
                self.generations
                    .iter()
                    .map(|gen| {
                        gen.individuals
                            .iter()
                            .map(|ind| ind.fitness.values().get(metric_idx).copied().unwrap_or(0.0))
                            .collect()
                    })
                    .collect()
            })
            .collect()
    }

    pub fn all_historical_fitness(&self) -> Vec<f64> {
        if self
            .objective
            .as_ref()
            .map(|o| o.is_multi_objective)
            .unwrap_or(false)
        {
            self.historical_fitness_multi()
                .into_iter()
                .flat_map(|metric| metric.into_iter().flat_map(|gen| gen.into_iter()))
                .collect()
        } else {
            self.historical_fitness()
                .into_iter()
                .flat_map(|gen| gen.into_iter())
                .collect()
        }
    }

    pub fn all_historical_quality(&self, metric_position: usize) -> Vec<f64> {
        if self
            .objective
            .as_ref()
            .map(|o| o.is_multi_objective)
            .unwrap_or(false)
        {
            self.historical_fitness_multi()
                .get(metric_position)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .flat_map(|gen| gen.into_iter())
                .collect()
        } else {
            self.all_historical_fitness()
        }
    }

    pub fn get_leaderboard(&self, top_n: usize) -> String {
        let mut individuals_with_positions: Vec<_> = self
            .generations
            .iter()
            .enumerate()
            .flat_map(|(gen_num, gen)| {
                gen.individuals
                    .iter()
                    .enumerate()
                    .map(move |(ind_num, ind)| {
                        (ind.graph.as_ref().descriptive_id(), ind, gen_num, ind_num)
                    })
            })
            .collect();
        individuals_with_positions.sort_by(|a, b| {
            b.1.fitness
                .partial_cmp(&a.1.fitness)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        individuals_with_positions.truncate(top_n);

        let mut output = String::from("Position | Fitness | Generation | Graph\n");
        for (ind_num, (_, ind, gen_num, _)) in individuals_with_positions.iter().enumerate() {
            output.push_str(&format!(
                "{ind_num:>3} | {:>8?} | g{gen_num:>3} | {}\n",
                ind.fitness,
                ind.graph.as_ref().descriptive_id()
            ));
        }
        if let Some(first_gen) = self.generations.first() {
            for (i, individual) in first_gen.individuals.iter().enumerate() {
                output.push_str(&format!(
                    "I{i:>3} | {:>8?} |        - | {}\n",
                    individual.fitness,
                    individual.graph.as_ref().descriptive_id()
                ));
            }
        }
        output
    }

    pub fn save(&self, json_file_path: Option<&Path>, is_save_light: bool) -> String {
        let history = if is_save_light {
            self.lighten()
        } else {
            self.clone()
        };
        let value = history_to_json(&history);
        crate::golem::serializers::default_save(&value, json_file_path)
    }

    pub fn load(path_or_str: &str) -> Result<Self, serde_json::Error> {
        let content = if std::path::Path::new(path_or_str).exists() {
            std::fs::read_to_string(path_or_str).unwrap_or_else(|_| path_or_str.to_string())
        } else {
            path_or_str.to_string()
        };
        let mut value: Value = serde_json::from_str(&content)?;
        crate::golem::serializers::remap_legacy_paths(&mut value);
        Ok(history_from_json(&value))
    }

    fn lighten(&self) -> Self {
        let mut light = Self::new(self.objective.clone());
        light.archive_history = self.archive_history.clone();
        light.default_save_dir = self.default_save_dir.clone();
        for (i, archive_gen) in self.archive_history.iter().enumerate() {
            let individuals: Vec<Individual> = archive_gen
                .iter()
                .filter_map(|uid| self.individuals_pool.get(uid).cloned())
                .collect();
            light.add_to_history(individuals, None, None);
            if let Some(gen) = light.generations.last_mut() {
                gen.generation_num = i;
            }
        }
        light
    }
}

fn parse_objective_info(value: &Value) -> Option<ObjectiveInfo> {
    if let Some(obj) = value.get("_objective").or_else(|| value.get("objective")) {
        if let Ok(info) = serde_json::from_value::<ObjectiveInfo>(obj.clone()) {
            return Some(info);
        }
    }
    value
        .get("_is_multi_objective")
        .and_then(|v| v.as_bool())
        .map(|is_multi_objective| ObjectiveInfo {
            is_multi_objective,
            metric_names: Vec::new(),
        })
}

fn history_to_json(history: &OptHistory) -> Value {
    let pool = flatten_individuals(history);
    let generations: Vec<Value> = history
        .generations
        .iter()
        .map(generation_uids_to_json)
        .collect();
    json!({
        "individuals_pool": pool.iter().map(individual_to_json).collect::<Vec<_>>(),
        "_generations": generations,
        "archive_history": history.archive_history,
        "_objective": history.objective,
        "_default_save_dir": history.default_save_dir,
        "_class_path": "golem.core.optimisers.opt_history_objects.opt_history/OptHistory",
    })
}

fn history_from_json(value: &Value) -> OptHistory {
    let mut root = value.clone();
    if let Some(obj) = root.as_object_mut() {
        if obj.contains_key("_is_multi_objective") && !obj.contains_key("_objective") {
            if let Some(info) = parse_objective_info(value) {
                obj.insert(
                    "_objective".into(),
                    serde_json::to_value(info).unwrap_or(Value::Null),
                );
            }
            obj.remove("_is_multi_objective");
        }
        if obj.contains_key("individuals") && !obj.contains_key("_generations") {
            if let Some(individuals) = obj.remove("individuals") {
                obj.insert("_generations".into(), individuals);
            }
        }
    }

    let objective = parse_objective_info(&root);
    let default_save_dir = root
        .get("_default_save_dir")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let mut uid_map = parse_individuals_pool(&root);
    let generations = parse_generations(&root, &mut uid_map);
    let archive_history = parse_archive_history(&root);

    let mut history = OptHistory {
        generations,
        archive_history,
        objective,
        default_save_dir,
        individuals_pool: uid_map.clone(),
    };
    resolve_parent_operators(&mut history, &uid_map);
    history
}

fn flatten_individuals(history: &OptHistory) -> Vec<Individual> {
    let mut by_uid: HashMap<String, Individual> = HashMap::new();
    fn collect(ind: &Individual, by_uid: &mut HashMap<String, Individual>) {
        if by_uid.contains_key(&ind.uid) {
            return;
        }
        by_uid.insert(ind.uid.clone(), ind.clone());
        if let Some(op) = &ind.parent_operator {
            for parent in &op.parent_individuals {
                collect(parent, by_uid);
            }
        }
    }
    for gen in &history.generations {
        for ind in &gen.individuals {
            collect(ind, &mut by_uid);
        }
    }
    for ind in history.individuals_pool.values() {
        collect(ind, &mut by_uid);
    }
    by_uid.into_values().collect()
}

fn generation_uids_to_json(generation: &Generation) -> Value {
    json!({
        "data": generation.individuals.iter().map(|i| i.uid.clone()).collect::<Vec<_>>(),
        "generation_num": generation.generation_num,
        "label": generation.label,
        "metadata": generation.metadata,
        "_class_path": "golem.core.optimisers.opt_history_objects.generation/Generation",
    })
}

fn parse_individuals_pool(value: &Value) -> HashMap<String, Individual> {
    let mut map = HashMap::new();
    if let Some(pool) = value.get("individuals_pool").and_then(|v| v.as_array()) {
        for item in pool {
            if let Some(ind) = individual_from_json(item) {
                map.insert(ind.uid.clone(), ind);
            }
        }
    }
    map
}

fn parse_generations(value: &Value, uid_map: &mut HashMap<String, Individual>) -> Vec<Generation> {
    let raw = value
        .get("_generations")
        .or_else(|| value.get("individuals"));
    let Some(arr) = raw.and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    arr.iter()
        .enumerate()
        .map(|(idx, gen_val)| parse_one_generation(gen_val, idx, uid_map))
        .collect()
}

fn parse_one_generation(
    gen_val: &Value,
    idx: usize,
    uid_map: &mut HashMap<String, Individual>,
) -> Generation {
    if let Some(obj) = gen_val.as_object() {
        let uid_seq = obj
            .get("data")
            .or_else(|| obj.get("individuals"))
            .cloned()
            .unwrap_or(Value::Array(Vec::new()));
        let individuals = uids_to_individuals(&uid_seq, uid_map);
        let generation_num = obj
            .get("generation_num")
            .and_then(|v| v.as_u64())
            .unwrap_or(idx as u64) as usize;
        let label = obj
            .get("label")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(String::from);
        let metadata = obj
            .get("metadata")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        Generation::new(individuals, generation_num, label, Some(metadata))
    } else {
        let individuals = uids_to_individuals(gen_val, uid_map);
        Generation::new(individuals, idx, None, None)
    }
}

fn parse_archive_history(value: &Value) -> Vec<Vec<String>> {
    let Some(arr) = value.get("archive_history").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    arr.iter()
        .map(|gen| match gen {
            Value::Array(uids) => uids
                .iter()
                .filter_map(|u| {
                    u.as_str()
                        .map(String::from)
                        .or_else(|| u.get("uid").and_then(|v| v.as_str()).map(String::from))
                })
                .collect(),
            Value::Object(obj) => obj
                .get("data")
                .and_then(|v| v.as_array())
                .map(|uids| {
                    uids.iter()
                        .filter_map(|u| u.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            _ => Vec::new(),
        })
        .collect()
}

fn uids_to_individuals(seq: &Value, uid_map: &mut HashMap<String, Individual>) -> Vec<Individual> {
    let Some(arr) = seq.as_array() else {
        return Vec::new();
    };
    arr.iter()
        .map(|item| {
            if let Some(uid) = item.as_str() {
                uid_map
                    .get(uid)
                    .cloned()
                    .unwrap_or_else(|| missing_individual(uid))
            } else if let Some(ind) = individual_from_json(item) {
                uid_map.entry(ind.uid.clone()).or_insert(ind.clone());
                ind
            } else {
                missing_individual("unknown")
            }
        })
        .collect()
}

fn missing_individual(uid: &str) -> Individual {
    let mut ind = Individual::with_uid(uid, Arc::new(GraphDelegate::empty()));
    ind.metadata.insert(
        "MISSING_INDIVIDUAL".into(),
        json!("This individual could not be restored during OptHistory.load()"),
    );
    ind
}

fn resolve_parent_operators(history: &mut OptHistory, uid_map: &HashMap<String, Individual>) {
    for gen in &mut history.generations {
        for ind in &mut gen.individuals {
            resolve_parents_of(ind, uid_map);
        }
    }
}

fn resolve_parents_of(ind: &mut Individual, uid_map: &HashMap<String, Individual>) {
    if let Some(op) = ind.parent_operator.as_mut() {
        if op.parent_individuals.is_empty() && !op.parent_uids.is_empty() {
            op.parent_individuals = op
                .parent_uids
                .iter()
                .map(|uid| {
                    uid_map
                        .get(uid)
                        .cloned()
                        .unwrap_or_else(|| missing_individual(uid))
                })
                .collect();
        }
    }
}

fn individual_to_json(ind: &Individual) -> Value {
    json!({
        "uid": ind.uid,
        "graph": graph_to_json(ind.graph.as_ref()),
        "fitness": fitness_to_json(&ind.fitness),
        "parent_operator": ind.parent_operator.as_ref().map(parent_operator_to_json),
        "metadata": ind.metadata,
        "native_generation": ind.native_generation,
        "_class_path": "golem.core.optimisers.opt_history_objects.individual/Individual",
    })
}

fn individual_from_json(value: &Value) -> Option<Individual> {
    let uid = value.get("uid")?.as_str()?.to_string();
    let graph = value
        .get("graph")
        .map(graph_from_json)
        .unwrap_or_else(GraphDelegate::empty);
    let mut ind = Individual::with_uid(uid, Arc::new(graph));
    if let Some(fitness) = value.get("fitness") {
        ind.fitness = fitness_from_json(fitness);
    }
    if let Some(meta) = value.get("metadata").and_then(|v| v.as_object()) {
        ind.metadata = meta.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    }
    if let Some(gen) = value.get("native_generation").and_then(|v| v.as_u64()) {
        ind.native_generation = Some(gen as usize);
    }
    if let Some(op) = value
        .get("parent_operator")
        .and_then(parent_operator_from_json)
    {
        ind.parent_operator = Some(op);
    }
    Some(ind)
}

fn parent_operator_to_json(op: &ParentOperator) -> Value {
    json!({
        "type_": op.type_,
        "operators": op.operators,
        "parent_individuals": op.parent_uids,
        "_class_path": "golem.core.optimisers.opt_history_objects.parent_operator/ParentOperator",
    })
}

fn parent_operator_from_json(value: &Value) -> Option<ParentOperator> {
    let type_ = value.get("type_")?.as_str()?.to_string();
    let operators = match value.get("operators") {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(","),
        _ => String::new(),
    };
    let parent_uids: Vec<String> = value
        .get("parent_individuals")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    item.as_str()
                        .map(String::from)
                        .or_else(|| item.get("uid").and_then(|u| u.as_str()).map(String::from))
                })
                .collect()
        })
        .unwrap_or_default();
    Some(ParentOperator {
        type_,
        operators,
        parent_individuals: Vec::new(),
        parent_uids,
    })
}

fn fitness_to_json(fitness: &Fitness) -> Value {
    match fitness {
        Fitness::Invalid => json!({
            "_values": [Value::Null],
            "_class_path": "golem.core.optimisers.fitness.fitness/SingleObjFitness",
        }),
        Fitness::Single(s) => {
            let values: Vec<Value> = s
                .raw_values()
                .iter()
                .map(|v| match v {
                    Some(n) => json!(n),
                    None => Value::Null,
                })
                .collect();
            json!({
                "_values": values,
                "_class_path": "golem.core.optimisers.fitness.fitness/SingleObjFitness",
            })
        }
        Fitness::Multi(m) => json!({
            "wvalues": m.values(),
            "weights": m.weights(),
            "_class_path": "golem.core.optimisers.fitness.multi_objective_fitness/MultiObjFitness",
        }),
    }
}

fn fitness_from_json(value: &Value) -> Fitness {
    if let Some(obj) = value.as_object() {
        if obj.contains_key("Single") || obj.contains_key("Multi") || obj.contains_key("Invalid") {
            if let Ok(fitness) = serde_json::from_value::<Fitness>(value.clone()) {
                return fitness;
            }
        }
        let class_path = obj
            .get("_class_path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if class_path.contains("MultiObjFitness") || obj.contains_key("wvalues") {
            let wvalues: Vec<f64> = obj
                .get("wvalues")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default();
            let weights: Vec<f64> = obj
                .get("weights")
                .or_else(|| obj.get("_weights"))
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_else(|| vec![1.0; wvalues.len()]);
            if wvalues.is_empty() {
                return Fitness::Invalid;
            }
            return Fitness::Multi(MultiObjFitness::from_wvalues(wvalues, weights));
        }
        let raw = obj
            .get("_values")
            .or_else(|| obj.get("values"))
            .and_then(|v| v.as_array());
        if let Some(arr) = raw {
            let values: Vec<Option<f64>> = arr
                .iter()
                .map(|v| if v.is_null() { None } else { v.as_f64() })
                .collect();
            return Fitness::Single(SingleObjFitness::from_values(values));
        }
    }
    Fitness::default()
}

fn graph_to_json(graph: &GraphDelegate) -> Value {
    let nodes: Vec<Value> = graph
        .nodes()
        .into_iter()
        .map(|node| {
            let guard = node.read().unwrap();
            json!({
                "uid": guard.uid,
                "content": content_to_json(&guard.content),
                "_nodes_from": guard.nodes_from.iter().map(|p| p.read().unwrap().uid.clone()).collect::<Vec<_>>(),
                "_class_path": "golem.core.dag.linked_graph_node/LinkedGraphNode",
            })
        })
        .collect();
    json!({
        "operator": {
            "_nodes": nodes,
            "_class_path": "golem.core.dag.linked_graph/LinkedGraph",
        },
        "_class_path": "golem.core.dag.graph_delegate/GraphDelegate",
    })
}

fn graph_from_json(value: &Value) -> GraphDelegate {
    let nodes_val = value
        .pointer("/operator/_nodes")
        .or_else(|| value.pointer("/_nodes"))
        .or_else(|| value.get("nodes"));
    let Some(arr) = nodes_val.and_then(|v| v.as_array()) else {
        return GraphDelegate::empty();
    };
    if arr.is_empty() {
        return GraphDelegate::empty();
    }

    let mut by_uid: HashMap<String, Arc<RwLock<LinkedGraphNode>>> = HashMap::new();
    let mut parent_uids: HashMap<String, Vec<String>> = HashMap::new();
    let mut order: Vec<String> = Vec::new();

    for node_val in arr {
        let uid = node_val
            .get("uid")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if uid.is_empty() {
            continue;
        }
        let content = node_val
            .get("content")
            .map(content_from_json)
            .unwrap_or_else(|| NodeContent::new(""));
        let parents: Vec<String> = node_val
            .get("_nodes_from")
            .or_else(|| node_val.get("nodes_from"))
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|p| p.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        by_uid.insert(uid.clone(), LinkedGraphNode::with_uid(uid.clone(), content));
        parent_uids.insert(uid.clone(), parents);
        order.push(uid);
    }

    for (uid, parents) in &parent_uids {
        let Some(node) = by_uid.get(uid).cloned() else {
            continue;
        };
        let parent_arcs: Vec<_> = parents
            .iter()
            .filter_map(|p| by_uid.get(p).cloned())
            .collect();
        node.write().unwrap().nodes_from = parent_arcs;
    }

    let arcs: Vec<_> = order
        .iter()
        .filter_map(|uid| by_uid.get(uid).cloned())
        .collect();
    if arcs.is_empty() {
        GraphDelegate::empty()
    } else {
        GraphDelegate::with_roots(arcs)
    }
}

fn content_to_json(content: &NodeContent) -> Value {
    let mut map = Map::new();
    map.insert("name".into(), json!(content.name));
    if !content.params.is_empty() {
        map.insert(
            "params".into(),
            Value::Object(
                content
                    .params
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            ),
        );
    }
    for (k, v) in &content.extra {
        map.insert(k.clone(), v.clone());
    }
    Value::Object(map)
}

fn content_from_json(value: &Value) -> NodeContent {
    let Some(obj) = value.as_object() else {
        return NodeContent::new(value.as_str().unwrap_or(""));
    };
    let name = obj
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let mut params = std::collections::BTreeMap::new();
    if let Some(p) = obj.get("params").and_then(|v| v.as_object()) {
        for (k, v) in p {
            params.insert(k.clone(), v.clone());
        }
    }
    let mut extra = std::collections::BTreeMap::new();
    for (k, v) in obj {
        if k != "name" && k != "params" && k != "_class_path" {
            extra.insert(k.clone(), v.clone());
        }
    }
    NodeContent {
        name,
        params,
        extra,
    }
}
