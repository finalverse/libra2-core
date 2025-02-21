// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_language_e2e_tests::data_store::FakeDataStore;
use libra2_types::{
    state_store::{
        state_key::StateKey, state_storage_usage::StateStorageUsage, state_value::StateValue,
        StateViewResult, TStateView,
    },
    transaction::Version,
};
use libra2_validator_interface::{Libra2ValidatorInterface, DebuggerStateView};
use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex},
};

pub struct DataStateView {
    debugger_view: DebuggerStateView,
    code_data: Option<FakeDataStore>,
    data_read_state_keys: Option<Arc<Mutex<HashMap<StateKey, StateValue>>>>,
}

impl DataStateView {
    pub fn new(
        db: Arc<dyn Libra2ValidatorInterface + Send>,
        version: Version,
        code_data: FakeDataStore,
    ) -> Self {
        Self {
            debugger_view: DebuggerStateView::new(db, version),
            code_data: Some(code_data),
            data_read_state_keys: None,
        }
    }

    pub fn new_with_data_reads(
        db: Arc<dyn Libra2ValidatorInterface + Send>,
        version: Version,
    ) -> Self {
        Self {
            debugger_view: DebuggerStateView::new(db, version),
            code_data: None,
            data_read_state_keys: Some(Arc::new(Mutex::new(HashMap::new()))),
        }
    }

    pub fn get_state_keys(self) -> Arc<Mutex<HashMap<StateKey, StateValue>>> {
        self.data_read_state_keys.unwrap()
    }
}

impl TStateView for DataStateView {
    type Key = StateKey;

    fn get_state_value(&self, state_key: &StateKey) -> StateViewResult<Option<StateValue>> {
        if let Some(code) = &self.code_data {
            if code.contains_key(state_key) {
                return code.get_state_value(state_key).map_err(Into::into);
            }
        }
        let ret = self.debugger_view.get_state_value(state_key);
        if let Some(reads) = &self.data_read_state_keys {
            if !state_key.is_aptos_code()
                && !reads.lock().unwrap().contains_key(state_key)
                && ret.is_ok()
            {
                let val = ret?;
                if val.is_some() {
                    reads
                        .lock()
                        .unwrap()
                        .deref_mut()
                        .insert(state_key.clone(), val.clone().unwrap());
                };
                return Ok(val);
            }
        }
        ret
    }

    fn get_usage(&self) -> StateViewResult<StateStorageUsage> {
        unreachable!()
    }
}
