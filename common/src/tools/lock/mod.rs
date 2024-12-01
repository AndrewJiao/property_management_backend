use crate::data_result::AppResult;
use anyhow::anyhow;
use lazy_static::lazy_static;
use log::info;
use parking_lot::{ArcMutexGuard, Mutex, RawMutex};
use std::collections::HashMap;
use std::sync::Arc;

pub struct PatternLockManager<> {
    lock_name: &'static str,
    locker: Mutex<HashMap<String, Arc<Mutex<()>>>>,
}

impl PatternLockManager {
    pub fn try_lock(&self, key: &str) -> AppResult<ArcMutexGuard<RawMutex, ()>> {
        info!("业务锁：{} 获取锁 key = {} begin",self.lock_name,key);
        let mut lock_guard = self.locker.lock();
        let key_str = key.to_string();

        let inner_lock = lock_guard.entry(key_str).or_insert(Arc::new(Mutex::new(())));
        let inner_guard = inner_lock.try_lock_arc().ok_or(anyhow!("其它业务正在处理中"))?;
        info!("业务锁：{} 获取锁 key = {} success",self.lock_name,key);
        Ok(inner_guard)
    }


    pub fn new(name: &'static str) -> Self {
        Self {
            lock_name: name,
            locker: Mutex::new(Default::default()),
        }
    }
}

lazy_static!(
    pub static ref LOCK_OWNER_FEE :PatternLockManager = PatternLockManager::new("owner_fee_system");
);
