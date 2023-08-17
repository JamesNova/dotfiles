use crate::utils::*;
use crate::{
    Alpm, AlpmList, AlpmListMut, Group, IntoRawAlpmList, Package, Result, SigLevel, Usage,
};

use std::ffi::CString;
use std::fmt;
use std::ops::Deref;
use std::ptr::NonNull;

use alpm_sys::*;

#[derive(Copy, Clone)]
#[doc(alias("repo", "repository"))]
pub struct Db<'h> {
    db: NonNull<alpm_db_t>,
    pub(crate) handle: &'h Alpm,
}

impl<'h> fmt::Debug for Db<'h> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Db").field("name", &self.name()).finish()
    }
}

pub struct DbMut<'h> {
    pub(crate) inner: Db<'h>,
}

impl<'h> fmt::Debug for DbMut<'h> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl<'h> Deref for DbMut<'h> {
    type Target = Db<'h>;

    fn deref(&self) -> &Db<'h> {
        &self.inner
    }
}

impl Alpm {
    pub fn register_syncdb<S: Into<Vec<u8>>>(&self, name: S, sig_level: SigLevel) -> Result<Db> {
        let name = CString::new(name).unwrap();

        let db =
            unsafe { alpm_register_syncdb(self.as_ptr(), name.as_ptr(), sig_level.bits() as i32) };

        self.check_null(db)?;
        unsafe { Ok(Db::new(self, db)) }
    }

    pub fn register_syncdb_mut<S: Into<Vec<u8>>>(
        &mut self,
        name: S,
        sig_level: SigLevel,
    ) -> Result<DbMut> {
        let db = self.register_syncdb(name, sig_level)?;
        Ok(DbMut { inner: db })
    }

    pub fn unregister_all_syncdbs(&mut self) -> Result<()> {
        self.check_ret(unsafe { alpm_unregister_all_syncdbs(self.as_ptr()) })
    }
}

impl<'h> DbMut<'h> {
    pub fn unregister(self) {
        unsafe { alpm_db_unregister(self.as_ptr()) };
    }

    pub fn add_server<S: Into<Vec<u8>>>(&self, server: S) -> Result<()> {
        let server = CString::new(server).unwrap();
        let ret = unsafe { alpm_db_add_server(self.as_ptr(), server.as_ptr()) };
        self.handle.check_ret(ret)
    }

    pub fn set_servers<'a, L: IntoRawAlpmList<'a, String>>(&self, list: L) -> Result<()> {
        let list = unsafe { list.into_raw_alpm_list() };
        let ret = unsafe { alpm_db_set_servers(self.as_ptr(), list.list()) };
        self.handle.check_ret(ret)
    }

    pub fn remove_server<S: Into<Vec<u8>>>(&self, server: S) -> Result<()> {
        let server = CString::new(server).unwrap();
        let ret = unsafe { alpm_db_remove_server(self.as_ptr(), server.as_ptr()) };
        self.handle.check_ret(ret)
    }
}

impl<'h> Db<'h> {
    pub(crate) unsafe fn new(handle: &Alpm, db: *mut alpm_db_t) -> Db {
        Db {
            handle,
            db: NonNull::new_unchecked(db),
        }
    }

    pub fn as_ptr(self) -> *mut alpm_db_t {
        self.db.as_ptr()
    }

    pub fn name(&self) -> &'h str {
        let name = unsafe { alpm_db_get_name(self.as_ptr()) };
        unsafe { from_cstr(name) }
    }

    pub fn servers(&self) -> AlpmList<'h, &'h str> {
        let list = unsafe { alpm_db_get_servers(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn pkg<S: Into<Vec<u8>>>(&self, name: S) -> Result<Package<'h>> {
        let name = CString::new(name).unwrap();
        let pkg = unsafe { alpm_db_get_pkg(self.as_ptr(), name.as_ptr()) };
        self.handle.check_null(pkg)?;
        unsafe { Ok(Package::new(self.handle, pkg)) }
    }

    #[doc(alias = "pkgcache")]
    pub fn pkgs(&self) -> AlpmList<'h, Package<'h>> {
        let pkgs = unsafe { alpm_db_get_pkgcache(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, pkgs) }
    }

    pub fn group<S: Into<Vec<u8>>>(&self, name: S) -> Result<Group<'h>> {
        let name = CString::new(name).unwrap();
        let group = unsafe { alpm_db_get_group(self.as_ptr(), name.as_ptr()) };
        self.handle.check_null(group)?;
        unsafe { Ok(Group::new(self.handle, group)) }
    }

    pub fn set_usage(&self, usage: Usage) -> Result<()> {
        let ret = unsafe { alpm_db_set_usage(self.as_ptr(), usage.bits() as i32) };
        self.handle.check_ret(ret)
    }

    pub fn search<L>(&self, list: L) -> Result<AlpmListMut<'h, Package<'h>>>
    where
        L: IntoRawAlpmList<'h, String>,
    {
        let mut ret = std::ptr::null_mut();
        let list = unsafe { list.into_raw_alpm_list() };
        let ok = unsafe { alpm_db_search(self.as_ptr(), list.list(), &mut ret) };
        self.handle.check_ret(ok)?;
        unsafe { Ok(AlpmListMut::from_parts(self.handle, ret)) }
    }

    #[doc(alias = "groupcache")]
    pub fn groups(&self) -> Result<AlpmListMut<'h, Group<'h>>> {
        let groups = unsafe { alpm_db_get_groupcache(self.as_ptr()) };
        self.handle.check_null(groups)?;
        unsafe { Ok(AlpmListMut::from_parts(self.handle, groups)) }
    }

    pub fn siglevel(&self) -> SigLevel {
        let siglevel = unsafe { alpm_db_get_siglevel(self.as_ptr()) };
        SigLevel::from_bits(siglevel as u32).unwrap()
    }

    pub fn is_valid(&self) -> Result<()> {
        let ret = unsafe { alpm_db_get_valid(self.as_ptr()) };
        self.handle.check_ret(ret)
    }

    pub fn usage(&self) -> Result<Usage> {
        let mut usage = 0;

        let ret = unsafe { alpm_db_get_usage(self.as_ptr(), &mut usage) };
        self.handle.check_ret(ret)?;

        let usage = Usage::from_bits(usage as u32).unwrap();
        Ok(usage)
    }
}

#[cfg(test)]
mod tests {
    use crate::SigLevel;
    use crate::{Alpm, AlpmListMut};

    #[test]
    fn test_register() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.register_syncdb("foo", SigLevel::NONE).unwrap();

        assert_eq!(db.name(), "foo");
    }

    #[test]
    fn test_servers() {
        let mut handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.register_syncdb_mut("foo", SigLevel::NONE).unwrap();
        assert_eq!(db.name(), "foo");
        let servers = vec!["a", "bb", "ccc"];

        for server in &servers {
            db.add_server(*server).unwrap();
        }

        let servers2 = db
            .servers()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        db.set_servers(servers2.iter()).unwrap();
        let servers2 = db
            .servers()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        db.set_servers(servers2.iter()).unwrap();

        assert_eq!(servers, db.servers().iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_set_servers() {
        let mut handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.register_syncdb_mut("foo", SigLevel::NONE).unwrap();
        assert_eq!(db.name(), "foo");
        let servers = vec!["a", "bb", "ccc"];

        db.set_servers(servers.iter().cloned()).unwrap();

        assert_eq!(servers, db.servers().iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_mut() {
        let mut handle = Alpm::new("/", "tests/db").unwrap();
        handle.register_syncdb_mut("foo", SigLevel::NONE).unwrap();
        handle.register_syncdb_mut("bar", SigLevel::NONE).unwrap();

        for db in handle.syncdbs_mut() {
            db.add_server("foo").unwrap();
        }

        for db in handle.syncdbs_mut() {
            db.add_server("bar").unwrap();
        }

        for db in handle.syncdbs() {
            assert_eq!(db.servers().iter().collect::<Vec<_>>(), vec!["foo", "bar"]);
        }

        for db in handle.syncdbs_mut() {
            db.unregister();
        }

        assert!(handle.syncdbs().is_empty());
    }

    #[test]
    fn test_pkg() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.register_syncdb("core", SigLevel::NONE).unwrap();
        let pkg = db.pkg("linux").unwrap();
        assert!(pkg.version().as_str() == "5.1.8.arch1-1");
    }

    #[test]
    fn test_search() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.register_syncdb("core", SigLevel::NONE).unwrap();
        let res = db
            .search(["^mkinitcpio-nfs-utils$"].iter().cloned())
            .unwrap();
        let res = res.iter().collect::<Vec<_>>();

        for _ in &res {}
        for _ in &res {}

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name(), "mkinitcpio-nfs-utils");

        let mut list: AlpmListMut<String> = AlpmListMut::new(&handle);
        list.push("pacman".to_string());

        let pkgs = db.search(&list).unwrap();
        assert!(!pkgs.is_empty());

        db.search(["pacman"].iter().cloned()).unwrap();
        db.search(vec!["pacman".to_string()].into_iter()).unwrap();
    }

    #[test]
    fn test_group() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.register_syncdb("core", SigLevel::NONE).unwrap();
        let base = db.group("base").unwrap();
        assert_eq!(base.name(), "base");
        assert!(base.packages().len() > 10);
        assert!(base.packages().len() < 100);
    }
}
