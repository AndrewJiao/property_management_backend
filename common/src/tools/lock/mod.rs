use crate::data_result::AppResult;
use anyhow::anyhow;
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct PatternLockManager<> {
    lock_name: &'static str,
    locker: Arc<Mutex<HashMap<String, Arc<Mutex<()>>>>>,
}

impl PatternLockManager {
    pub fn lock(&self, key: &str) -> AppResult<Arc<Mutex<()>>> {
        info!("业务锁：{} 获取锁 key = {} begin",self.lock_name,key);
        let mut lock_guard = self.locker.lock()
            .map_err(|e| anyhow!(e.to_string()))?;
        let key_str = key.to_string();
        let entry = lock_guard.entry(key_str.clone());
        let inner_lock = lock_guard.entry(key_str).or_insert(Arc::new(Mutex::new(())));
        // let guard = inner_lock.try_lock().map_err(|e| anyhow!(e.to_string()))?;
        info!("业务锁：{} 获取锁 key = {} success",self.lock_name,key);
        Ok(Arc::clone(inner_lock))
    }


    pub fn new(name: &'static str) -> Self {
        Self {
            lock_name: name,
            locker: Arc::new(Mutex::new(Default::default())),
        }
    }
}

lazy_static!(
    pub static ref LOCK_OWNER_FEE :PatternLockManager = PatternLockManager::new("owner_fee_system");
);
