use indexmap::IndexMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU64, Ordering};

/// クロージャを管理するIDの型
type Id = u64;
/// Subjectに購読するクロージャの型
type Closure<T> = Box<dyn FnMut(&T)>;

/// 購読したクロージャのID
static ID: AtomicU64 = AtomicU64::new(0);

/// Subjectの本体
struct SubjectInner<T> {
    subs: IndexMap<Id, Closure<T>>,
}

impl<T> SubjectInner<T> {
    fn new() -> Self {
        Self {
            subs: IndexMap::new(),
        }
    }

    /// クロージャを購読する
    fn subscribe(&mut self, f: Closure<T>) -> Id {
        let id = ID.fetch_add(1, Ordering::Relaxed);
        self.subs.insert(id, f);
        id
    }

    /// クロージャを購読解除する
    fn unsubscribe(&mut self, id: Id) {
        self.subs.swap_remove(&id);
    }
}

/// Subjectへの購読情報
/// drop時に自動解除される
pub struct Subscription<T> {
    id: Id,
    target: Weak<RefCell<SubjectInner<T>>>,
}

impl<T> Subscription<T> {
    fn new(id: Id, target: Weak<RefCell<SubjectInner<T>>>) -> Self {
        Self { id, target }
    }

    /// クロージャを購読解除する
    pub fn unsubscribe(&mut self) {
        if let Some(subject) = self.target.upgrade() {
            subject.borrow_mut().unsubscribe(self.id);
        }
    }
}

impl<T> Drop for Subscription<T> {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}

/// 購読可能な特性
pub trait Observable<T> {
    fn subscribe(&mut self, f: impl FnMut(&T) + 'static) -> Subscription<T>;
}

/// 購読されたクロージャへイベントを送信する型
pub struct Subject<T> {
    inner: Rc<RefCell<SubjectInner<T>>>,
}

impl<T> Observable<T> for Subject<T> {
    /// クロージャを購読する
    /// 戻り値のSubscriptionはdrop時に購読解除されるため束縛すること
    fn subscribe(&mut self, f: impl FnMut(&T) + 'static) -> Subscription<T> {
        let id = self.inner.borrow_mut().subscribe(Box::new(f));
        Subscription::new(id, Rc::downgrade(&self.inner))
    }
}

impl<T> Subject<T> {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(SubjectInner::new())),
        }
    }

    /// 購読しているクロージャへイベントを送信する
    pub fn emit(&mut self, v: &T) {
        for f in self.inner.borrow_mut().subs.values_mut() {
            f(v);
        }
    }
}
