use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::any::{Any,TypeId};
use std::cell::UnsafeCell;
use std::sync::Mutex;


pub struct State<'r, T: Send + Sync + 'static>(&'r T);

impl<'r, T: Send + Sync + 'static> State<'r, T> {
  /// Retrieve a borrow to the underlying value with a lifetime of `'r`.
  /// Using this method is typically unnecessary as `State` implements
  /// [`std::ops::Deref`] with a [`std::ops::Deref::Target`] of `T`.
  #[inline(always)]
  pub fn inner(&self) -> &'r T {
    self.0
  }
}

impl<T: Send + Sync + 'static> std::ops::Deref for State<'_, T> {
  type Target = T;

  #[inline(always)]
  fn deref(&self) -> &T {
    self.0
  }
}

impl<T: Send + Sync + 'static> Clone for State<'_, T> {
  fn clone(&self) -> Self {
    State(self.0)
  }
}

impl<T: Send + Sync + 'static + PartialEq> PartialEq for State<'_, T> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<'r, T: Send + Sync + std::fmt::Debug> std::fmt::Debug for State<'r, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("State").field(&self.0).finish()
  }
}



#[derive(Default,Clone)]
struct IdentHash(u64);

impl std::hash::Hasher for IdentHash {
  fn finish(&self) -> u64 {
    self.0
  }

  fn write(&mut self, bytes: &[u8]) {
    for byte in bytes {
      self.write_u8(*byte);
    }
  }

  fn write_u8(&mut self, i: u8) {
    self.0 = (self.0 << 8) | (i as u64);
  }

  fn write_u64(&mut self, i: u64) {
    self.0 = i;
  }
}

type TypeIdMap = HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<IdentHash>>;

/// The Tauri state manager.
#[derive(Debug)]
pub struct StateManager {
  map: Mutex<UnsafeCell<TypeIdMap>>,
}
// SAFETY: data is accessed behind a lock
unsafe impl Sync for StateManager {}
unsafe impl Send for StateManager {}
impl StateManager {
    pub(crate) fn new() -> Self {
      Self {
        map: Default::default(),
      }
    }
  
    fn with_map_ref<'a, F: FnOnce(&'a TypeIdMap) -> R, R>(&'a self, f: F) -> R {
      let map = self.map.lock().unwrap();
      let map = map.get();
      // SAFETY: safe to access since we are holding a lock
      f(unsafe { &*map })
    }
  
    fn with_map_mut<F: FnOnce(&mut TypeIdMap) -> R, R>(&self, f: F) -> R {
      let mut map = self.map.lock().unwrap();
      let map = map.get_mut();
      f(map)
    }
  
    pub(crate) fn set<T: Send + Sync + 'static>(&self, state: T) -> bool {
      self.with_map_mut(|map| {
        let type_id = TypeId::of::<T>();
        let already_set = map.contains_key(&type_id);
        if !already_set {
          map.insert(type_id, Box::new(state) as Box<dyn Any>);
        }
        !already_set
      })
    }
  
    pub(crate) fn unmanage<T: Send + Sync + 'static>(&self) -> Option<T> {
      self.with_map_mut(|map| {
        let type_id = TypeId::of::<T>();
        map
          .remove(&type_id)
          .and_then(|ptr| ptr.downcast().ok().map(|b| *b))
      })
    }
  
    /// Gets the state associated with the specified type.
    pub fn get<T: Send + Sync + 'static>(&self) -> State<'_, T> {
      self
        .try_get()
        .expect("state: get() when given type is not managed")
    }
  
    /// Gets the state associated with the specified type.
    pub fn try_get<T: Send + Sync + 'static>(&self) -> Option<State<'_, T>> {
      self.with_map_ref(|map| {
        map
          .get(&TypeId::of::<T>())
          .and_then(|ptr| ptr.downcast_ref::<T>())
          .map(State)
      })
    }
  }

  