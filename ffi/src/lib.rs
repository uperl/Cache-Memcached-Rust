use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CStr;
use memcache::Url;
use memcache::Client;
use memcache::ConnectionManager;
use r2d2::ManageConnection;

struct Mc {
  manager: ConnectionManager,
  client: Option<Client>,
}

thread_local!(
  static COUNTER: Cell<u64> = Cell::new(1);
  static STORE: RefCell<HashMap<u64, Mc>> = RefCell::new(HashMap::new())
);

#[no_mangle]
pub extern "C" fn mc_new(url: *const i8) -> u64 {
  let url = unsafe { CStr::from_ptr(url) };
  let url = match url.to_str() {
    Ok(url)  => url,
    Err(_)   => return 0
  };
  let url = match Url::parse(url) {
    Ok(url) => url,
    Err(_)  => return 0
  };
  let manager = ConnectionManager::new(url);

  let mc = Mc {
    manager: manager,
    client: None,
  };

  let index = COUNTER.with(|it| {
    let index = it.get();
    it.set(index + 1);
    index
  });

  STORE.with(|it| {
    let mut it = it.borrow_mut();
    it.insert(index, mc);
  });

  index
}

#[no_mangle]
pub extern "C" fn mc_DESTROY(index: u64) {
  STORE.with(|it| {
    let mut it = it.borrow_mut();
    it.remove(&index);
  })
}
