use std::cell::RefCell;

use ic_stable_structures::{DefaultMemoryImpl, memory_manager::{VirtualMemory, MemoryManager, MemoryId}};

mod types;

type MemoryType = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static STABLE_CELL: RefCell<ic_stable_structures::Cell<types::Snapshot, MemoryType>> = RefCell::new(ic_stable_structures::Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), types::Snapshot::default()).unwrap());
    static STABLE_VEC: RefCell<ic_stable_structures::Vec<types::Snapshot, MemoryType>> = RefCell::new(ic_stable_structures::Vec::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))).unwrap());
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn dummy() -> types::Snapshot {
    types::dummy(ic_cdk::api::time() / (1000 * 1000000))
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn default() -> types::Snapshot {
    types::Snapshot::default()
}

// stable: cell
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_cell() -> types::Snapshot {
    STABLE_CELL.with(|p| p.borrow().get().clone())
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn update_from_dummy() {
    let datum = dummy();
    update_cell(datum);
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn update_from_default() {
    let datum = default();
    update_cell(datum);
}
fn update_cell(data: types::Snapshot) {
    STABLE_CELL.with(|mem| {
        mem.borrow_mut().set(data.clone()).unwrap();
    });
}

// stable: vec
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_last_data() -> types::Snapshot {
    let datum = STABLE_VEC.with(|mem| {
        let borrowed_mem = mem.borrow();
        let len = borrowed_mem.len();
        let res = borrowed_mem.get(len - 1);
        res.unwrap().clone()
    });
    datum
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_data(n: u64) -> types::Snapshot {
    let datum = STABLE_VEC.with(|mem| {
        let borrowed_mem = mem.borrow();
        let res = borrowed_mem.get(n as u64);
        res.unwrap().clone()
    });
    datum
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn add_from_dummy() {
    let datum = dummy();
    add_to_vec(datum);
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn add_from_default() {
    let datum = default();
    add_to_vec(datum);
}
fn add_to_vec(datum: types::Snapshot) {
    STABLE_VEC.with(|mem| {
        mem.borrow_mut().push(&datum).unwrap();
    });
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use candid::{Encode, Decode};

    use super::*;
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }

    #[test]
    fn test_default_bytes() {
        let bytes = Encode!(&default()).unwrap();
        let data = Decode!(bytes.as_ref(), types::Snapshot).unwrap();
        let re_bytes = Encode!(&data).unwrap();
        assert_eq!(bytes, re_bytes);
    }

    #[test]
    fn test_dummy_bytes() {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap().as_secs();
        let bytes = Encode!(&types::dummy(time)).unwrap();
        let data = Decode!(bytes.as_ref(), types::Snapshot).unwrap();
        let re_bytes = Encode!(&data).unwrap();
        assert_eq!(bytes, re_bytes);
    }
}
