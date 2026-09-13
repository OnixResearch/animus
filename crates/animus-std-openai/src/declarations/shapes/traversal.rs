// The JSON byte bound also bounds schema nodes. Each node consumes input bytes.
const MAX_VISITED_NODES: usize = crate::declarations::MAX_SCHEMA_BYTES;
// serde_json retains its default recursion limit for the initial JSON decode.
const MAX_SCHEMA_DEPTH: u32 = 128;

struct PendingNode<'a> {
    value: &'a serde_json::Value,
    depth: u32,
}

// This owner admits every push and charges every visit before it removes work.
struct SchemaWork<'a> {
    pending: Vec<PendingNode<'a>>,
    visited: usize,
}

impl<'a> SchemaWork<'a> {
    fn new(schema: &'a serde_json::Value) -> Self {
        Self {
            pending: vec![PendingNode {
                value: schema,
                depth: 0,
            }],
            visited: 0,
        }
    }

    fn pop(&mut self) -> Result<Option<PendingNode<'a>>, crate::HostError> {
        let Some(node) = self.pending.last() else {
            return Ok(None);
        };
        let visited = self.visited.checked_add(1).ok_or_else(shape_error)?;
        if visited > MAX_VISITED_NODES || node.depth > MAX_SCHEMA_DEPTH {
            return Err(shape_error());
        }
        debug_assert!(visited <= MAX_VISITED_NODES);
        debug_assert!(node.depth <= MAX_SCHEMA_DEPTH);
        self.visited = visited;
        Ok(Vec::pop(&mut self.pending))
    }

    fn enqueue(
        &mut self,
        value: &'a serde_json::Value,
        depth: u32,
    ) -> Result<(), crate::HostError> {
        if self.pending.len() >= MAX_VISITED_NODES || depth > MAX_SCHEMA_DEPTH {
            return Err(shape_error());
        }
        Vec::push(&mut self.pending, PendingNode { value, depth });
        debug_assert!(self.pending.len() <= MAX_VISITED_NODES);
        debug_assert!(depth <= MAX_SCHEMA_DEPTH);
        Ok(())
    }
}

pub(super) fn validate(schema: &serde_json::Value) -> Result<(), crate::HostError> {
    if schema.get("type").and_then(serde_json::Value::as_str) != Some("object") {
        return Err(shape_error());
    }
    let mut work = SchemaWork::new(schema);
    while let Some(node) = work.pop()? {
        let object = node.value.as_object().ok_or_else(shape_error)?;
        let child_depth = node.depth.checked_add(1).ok_or_else(shape_error)?;
        visit_object(object, &mut work, child_depth)?;
    }
    Ok(())
}

fn visit_object<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    work: &mut SchemaWork<'a>,
    child_depth: u32,
) -> Result<(), crate::HostError> {
    debug_assert!(child_depth > 0);
    debug_assert!(work.pending.len() <= MAX_VISITED_NODES);
    let kind = object.get("type").and_then(serde_json::Value::as_str);
    if kind == Some("object") {
        let properties = validate_properties(object)?;
        for property in properties.values() {
            work.enqueue(property, child_depth)?;
        }
    }
    if kind == Some("array") {
        work.enqueue(object.get("items").ok_or_else(shape_error)?, child_depth)?;
    }
    visit_unions(object, work, child_depth)?;
    if let Some(definitions) = object.get("$defs") {
        let definitions = definitions.as_object().ok_or_else(shape_error)?;
        for definition in definitions.values() {
            work.enqueue(definition, child_depth)?;
        }
    }
    Ok(())
}

fn validate_properties(
    object: &serde_json::Map<String, serde_json::Value>,
) -> Result<&serde_json::Map<String, serde_json::Value>, crate::HostError> {
    let properties = object
        .get("properties")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(shape_error)?;
    let required = object
        .get("required")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(shape_error)?;
    if object
        .get("additionalProperties")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
    {
        return Err(shape_error());
    }
    let required_names = required
        .iter()
        .map(serde_json::Value::as_str)
        .collect::<Option<std::collections::BTreeSet<_>>>()
        .ok_or_else(shape_error)?;
    let property_names = properties
        .keys()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    if required_names != property_names {
        return Err(shape_error());
    }
    debug_assert_eq!(required_names, property_names);
    debug_assert_eq!(
        object
            .get("additionalProperties")
            .and_then(serde_json::Value::as_bool),
        Some(false)
    );
    Ok(properties)
}

fn visit_unions<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    work: &mut SchemaWork<'a>,
    child_depth: u32,
) -> Result<(), crate::HostError> {
    for union_key in ["anyOf", "oneOf", "allOf"] {
        if let Some(union) = object.get(union_key) {
            let choices = union.as_array().ok_or_else(shape_error)?;
            if choices.is_empty() {
                return Err(shape_error());
            }
            for choice in choices {
                work.enqueue(choice, child_depth)?;
            }
        }
    }
    Ok(())
}

const fn shape_error() -> crate::HostError {
    crate::HostError::InvalidToolSchema {
        reason: crate::declarations::SCHEMA_SHAPE_REASON,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pending_capacity_refusal_preserves_admitted_work() {
        let schema = serde_json::json!({});
        let mut work = super::SchemaWork::new(&schema);
        for _node in 1..super::MAX_VISITED_NODES {
            work.enqueue(&schema, 0)
                .expect("capacity must admit this node");
        }
        assert_eq!(work.pending.len(), super::MAX_VISITED_NODES);
        assert_eq!(work.enqueue(&schema, 0), Err(super::shape_error()));
        assert_eq!(work.pending.len(), super::MAX_VISITED_NODES);
        assert_eq!(work.visited, 0);
        assert!(work.pop().expect("admitted node must remain").is_some());
    }

    #[test]
    fn depth_refusal_preserves_lifo_order() {
        let schema = serde_json::json!({});
        let child = serde_json::json!({"type": "string"});
        let mut work = super::SchemaWork::new(&schema);
        work.enqueue(&child, super::MAX_SCHEMA_DEPTH)
            .expect("exact depth must pass");
        let refused_depth = super::MAX_SCHEMA_DEPTH
            .checked_add(1)
            .expect("depth must fit");
        assert_eq!(
            work.enqueue(&schema, refused_depth),
            Err(super::shape_error())
        );
        let last = work
            .pop()
            .expect("last admitted node must remain")
            .expect("child must exist");
        assert_eq!(last.value, &child);
        assert_eq!(last.depth, super::MAX_SCHEMA_DEPTH);
        let root = work
            .pop()
            .expect("root must remain")
            .expect("root must exist");
        assert_eq!(root.value, &schema);
        assert_eq!(root.depth, 0);
        assert!(work.pop().expect("empty work must pass").is_none());
    }

    #[test]
    fn visit_limit_refusal_retains_pending_node() {
        let schema = serde_json::json!({});
        let mut work = super::SchemaWork::new(&schema);
        for _node in 0..super::MAX_VISITED_NODES {
            assert!(
                work.pop()
                    .expect("visit budget must admit this node")
                    .is_some()
            );
            work.enqueue(&schema, 0).expect("one pending node must fit");
        }
        assert_eq!(work.visited, super::MAX_VISITED_NODES);
        assert!(matches!(work.pop(), Err(error) if error == super::shape_error()));
        assert_eq!(work.visited, super::MAX_VISITED_NODES);
        assert_eq!(work.pending.len(), 1);
    }
}
