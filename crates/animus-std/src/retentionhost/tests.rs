const ZERO: u32 = 0;
const ONE: u32 = 1;
const TWO: u32 = 2;
const THREE: u32 = 3;
const TWO_SLOTS: usize = 2;
const THRESHOLD_LRU: u8 = 0;
const THRESHOLD_FIFO: u8 = 100;

fn key(label: &str) -> animus_core::retention::Digest {
    animus_core::retention::project_key(animus_core::retention::Projection {
        tool_identity: label,
        normalized_arguments: "{}",
        capability_context: None,
    })
    .expect("valid key")
}

fn config(capacity: Option<u32>, threshold: u8) -> super::Config {
    let bound_entries = capacity.unwrap_or(ZERO);
    super::Config::new(capacity, bound_entries, bound_entries, threshold)
}

#[test]
fn missing_capacity_and_reversed_watermarks_fail_closed() {
    let missing = super::Store::<std::string::String>::new(config(None, THRESHOLD_LRU));
    let reversed = super::Store::<std::string::String>::new(super::Config::new(
        Some(TWO),
        TWO,
        ONE,
        THRESHOLD_LRU,
    ));

    assert!(matches!(missing, Err(super::StoreError::MissingCapacity)));
    assert!(matches!(
        reversed,
        Err(super::StoreError::Decision(
            animus_core::retention::DecisionError::ReversedWatermarks
        ))
    ));
}

#[test]
fn explicit_zero_capacity_is_pass_through() {
    let cache = super::Store::new(config(Some(ZERO), THRESHOLD_LRU)).expect("zero cache");
    let outcome = cache
        .insert(key("zero"), std::string::String::from("value"))
        .expect("insert");

    assert_eq!(cache.capacity(), ZERO);
    assert_eq!(outcome, super::InsertOutcome::NotRetained);
    assert_eq!(cache.len(), Ok(ZERO));
    assert_eq!(cache.get(&key("zero")), Ok(None));
}

#[test]
fn single_slot_evicts_and_never_exceeds_capacity() {
    let cache = super::Store::new(config(Some(ONE), THRESHOLD_LRU)).expect("cache");
    let first_key = key("first");
    let second_key = key("second");

    assert_eq!(
        cache.insert(first_key, std::string::String::from("first")),
        Ok(super::InsertOutcome::Inserted {
            evicted_keys: std::vec::Vec::new()
        })
    );
    assert_eq!(
        cache.insert(second_key, std::string::String::from("second")),
        Ok(super::InsertOutcome::Inserted {
            evicted_keys: std::vec![first_key]
        })
    );
    let retained = cache
        .get(&second_key)
        .expect("lookup")
        .expect("retained value");

    assert_eq!(cache.len(), Ok(ONE));
    assert_eq!(cache.get(&first_key), Ok(None));
    assert_eq!(retained.as_str(), "second");
}

#[test]
fn strict_lru_promotes_but_fifo_preserves_insertion_order() {
    let first_key = key("first");
    let second_key = key("second");
    let third_key = key("third");
    let lru = super::Store::new(config(Some(TWO), THRESHOLD_LRU)).expect("lru");
    lru.insert(first_key, "first").expect("first");
    lru.insert(second_key, "second").expect("second");
    assert!(lru.get(&first_key).expect("hit").is_some());
    let lru_outcome = lru.insert(third_key, "third").expect("third");

    let fifo = super::Store::new(config(Some(TWO), THRESHOLD_FIFO)).expect("fifo");
    fifo.insert(first_key, "first").expect("first");
    fifo.insert(second_key, "second").expect("second");
    assert!(fifo.get(&first_key).expect("hit").is_some());
    let fifo_outcome = fifo.insert(third_key, "third").expect("third");

    assert_eq!(
        lru_outcome,
        super::InsertOutcome::Inserted {
            evicted_keys: std::vec![second_key]
        }
    );
    assert_eq!(
        fifo_outcome,
        super::InsertOutcome::Inserted {
            evicted_keys: std::vec![first_key]
        }
    );
}

#[test]
fn watermark_trim_releases_to_low_bound() {
    let config = super::Config::new(Some(THREE), ONE, TWO, THRESHOLD_LRU);
    let cache = super::Store::new(config).expect("watermark cache");
    cache.insert(key("first"), "first").expect("first");
    cache.insert(key("second"), "second").expect("second");
    let outcome = cache.insert(key("third"), "third").expect("third");

    assert_eq!(cache.len(), Ok(ONE));
    assert!(matches!(
        outcome,
        super::InsertOutcome::Inserted { ref evicted_keys } if evicted_keys.len() == TWO_SLOTS
    ));
    assert!(cache.get(&key("third")).expect("third lookup").is_some());
}

#[test]
fn miss_path_calls_adapter_once_and_hit_skips_it() {
    let cache = super::Store::new(config(Some(ONE), THRESHOLD_LRU)).expect("cache");
    let request_key = key("adapter");
    let calls = std::sync::atomic::AtomicU32::new(ZERO);
    let first = cache
        .get_or_try_insert_with(request_key, || {
            calls.fetch_add(ONE, std::sync::atomic::Ordering::SeqCst);
            Ok::<_, &'static str>(std::string::String::from("tool result"))
        })
        .expect("miss");
    let second = cache
        .get_or_try_insert_with(request_key, || {
            calls.fetch_add(ONE, std::sync::atomic::Ordering::SeqCst);
            Err::<std::string::String, _>("must not run")
        })
        .expect("hit");

    assert!(!first.is_hit());
    assert!(second.is_hit());
    assert_eq!(first.value().as_str(), "tool result");
    assert_eq!(second.value().as_str(), "tool result");
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), ONE);
}

struct DropProbe {
    state: std::sync::Weak<std::sync::Mutex<super::storage::State<DropProbe>>>,
    observed_unlocked: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Drop for DropProbe {
    fn drop(&mut self) {
        let is_unlocked = self
            .state
            .upgrade()
            .is_some_and(|state| state.try_lock().is_ok());
        self.observed_unlocked
            .store(is_unlocked, std::sync::atomic::Ordering::SeqCst);
    }
}

#[test]
fn evicted_value_drops_after_guard_release() {
    let cache = super::Store::new(config(Some(ONE), THRESHOLD_LRU)).expect("cache");
    let state = std::sync::Arc::downgrade(&cache.state);
    let observed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    cache
        .insert(
            key("first"),
            DropProbe {
                state: std::sync::Weak::clone(&state),
                observed_unlocked: std::sync::Arc::clone(&observed),
            },
        )
        .expect("first");
    cache
        .insert(
            key("second"),
            DropProbe {
                state,
                observed_unlocked: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            },
        )
        .expect("second");

    assert!(observed.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(cache.len(), Ok(ONE));
}
