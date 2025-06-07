use std::ops::{Deref, DerefMut};
use anymap::{any::{Any, IntoBox}, Map};
use tokio::sync::{Mutex, MutexGuard};

pub struct Extensions {
    map: Mutex<Map::<dyn Any + Send + Sync>>
}

impl Extensions {
    pub fn new() -> Self {
        Self { map: Mutex::new(Map::new()) }
    }

    pub async fn lock(&self) -> MutexGuard<'_, Map<(dyn Any + Send + Sync + 'static)>> {
        self.map.lock().await
    }

    pub async fn contains<T>(&self) -> bool where T: IntoBox<(dyn Any + Send + Sync)> {
        self.map.lock().await.contains::<T>()
    }

    /// Since it returns a guard, consider it like get_mut
    pub async fn get<T>(&self) -> Option<Extension<'_, T>> where T: IntoBox<(dyn Any + Send + Sync)> {
        let guard = self.map.lock().await;
        Self::guard_to_ext(guard)
    }

    fn guard_to_ext<T>(mut guard: ExtensionGuard<'_>) -> Option<Extension<'_, T>>
    where T: IntoBox<(dyn Any + Send + Sync)>
    {
        let value = guard.get_mut::<T>()?;
        // SAFETY: we can extend the lidetime of the value reference
        // since we're not using the guard or value independently.
        let value_ref = unsafe { &mut *(value as *mut T) };
        Some(Extension { guard, value: value_ref })
    }

    pub async fn insert<T>(&self, value: T) -> Option<T>
    where T: IntoBox<(dyn Any + Send + Sync)>
    {
        self.map.lock().await.insert(value)
    }
}

type ExtensionGuard<'a> = MutexGuard<'a, Map<dyn Any + Send + Sync>>;

/// dropping this struct means dropping the guard, unlocking the extensions mutex  
pub struct Extension<'a, T> {
    #[allow(unused)]
    guard: ExtensionGuard<'a>,
    value: &'a mut T
}

impl<'a, T> Deref for Extension<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T> DerefMut for Extension<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}