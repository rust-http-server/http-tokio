use std::ops::{Deref, DerefMut};
use anymap::{any::{Any, IntoBox}, Map};
use tokio::sync::{Mutex, MutexGuard};

pub type OldExtensions = Map::<dyn Any + Send + Sync>;

pub struct Extensions {
    map: Mutex<Map::<dyn Any + Send + Sync>>
}

impl Extensions {
    pub fn new() -> Self {
        Self { map: Mutex::new(Map::new()) }
    }

    /// NON SO SE SIA IL CASO
    pub fn get_sync_unsafe<T>(&self) -> Option<Ext<'_, T>> where T: IntoBox<(dyn Any + Send + Sync)>
    {
        let guard = self.map.try_lock().expect("i am sorry for this... but i just dont like to await"); 
        Self::guard_to_ext(guard)
    }

    /// Since it returns a guard, consider it like get_mut
    pub async fn get<T>(&self) -> Option<Ext<'_, T>> where T: IntoBox<(dyn Any + Send + Sync)>
    {
        let guard = self.map.lock().await;
        Self::guard_to_ext(guard)
    }

    fn guard_to_ext<T>(mut guard: ExtGuard<'_>) -> Option<Ext<'_, T>>
    where T: IntoBox<(dyn Any + Send + Sync)>
    {
        let value = guard.get_mut::<T>()?;
        // SAFETY: we can extend the lidetime of the value reference
        // since we're not using the guard or value independently.
        let value_ref = unsafe { &mut *(value as *mut T) };
        Some(Ext { guard, value: value_ref })
    }

    pub async fn insert<T>(&self, value: T) -> Option<T>
    where T: IntoBox<(dyn Any + Send + Sync)>
    {
        self.map.lock().await.insert(value)
    }
}

type ExtGuard<'a> = MutexGuard<'a, Map<dyn Any + Send + Sync>>;

/// dropping this struct means dropping the guard, unlocking the extensions mutex  
pub struct Ext<'a, T> {
    #[allow(unused)]
    guard: ExtGuard<'a>,
    value: &'a mut T
}

impl<'a, T> Deref for Ext<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T> DerefMut for Ext<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}