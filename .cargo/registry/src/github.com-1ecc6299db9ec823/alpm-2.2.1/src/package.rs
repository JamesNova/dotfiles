use crate::utils::*;
use crate::{
    Alpm, AlpmList, AlpmListMut, Backup, ChangeLog, Db, Dep, FileList, PackageFrom, PackageReason,
    PackageValidation, Result, Signature, Ver,
};

#[cfg(feature = "mtree")]
use crate::MTree;

use std::mem::transmute;
use std::ops::Deref;
use std::ptr::NonNull;
use std::{fmt, ptr};

use alpm_sys::*;

pub trait AsPkg {
    fn as_pkg(&self) -> Pkg;
}

impl<'h> AsPkg for Package<'h> {
    fn as_pkg(&self) -> Pkg {
        self.pkg
    }
}

impl<'h> AsPkg for Pkg<'h> {
    fn as_pkg(&self) -> Pkg {
        *self
    }
}

#[derive(Copy, Clone)]
pub struct Package<'h> {
    pub(crate) pkg: Pkg<'h>,
}

#[derive(Copy, Clone)]
pub struct Pkg<'h> {
    pub(crate) handle: &'h Alpm,
    pkg: NonNull<alpm_pkg_t>,
}

impl<'h> fmt::Debug for Pkg<'h> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pkg")
            .field("name", &self.name())
            .field("version", &self.version())
            .finish()
    }
}

impl<'h> fmt::Debug for Package<'h> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Package")
            .field("name", &self.name())
            .field("version", &self.version())
            .finish()
    }
}

impl<'h> Deref for Package<'h> {
    type Target = Pkg<'h>;
    fn deref(&self) -> &Self::Target {
        &self.pkg
    }
}

impl<'h> Package<'h> {
    pub(crate) unsafe fn new(handle: &Alpm, pkg: *mut alpm_pkg_t) -> Package {
        Package {
            pkg: Pkg::new(handle, pkg),
        }
    }
}

impl<'h> Pkg<'h> {
    pub(crate) unsafe fn new(handle: &Alpm, pkg: *mut alpm_pkg_t) -> Pkg {
        Pkg {
            handle,
            pkg: NonNull::new_unchecked(pkg),
        }
    }

    pub(crate) fn as_ptr(self) -> *mut alpm_pkg_t {
        self.pkg.as_ptr()
    }

    pub fn name(&self) -> &'h str {
        let name = unsafe { alpm_pkg_get_name(self.as_ptr()) };
        unsafe { from_cstr(name) }
    }

    pub fn check_md5sum(&self) -> Result<()> {
        self.handle
            .check_ret(unsafe { alpm_pkg_checkmd5sum(self.as_ptr()) })
    }

    pub fn should_ignore(&self) -> bool {
        let ret = unsafe { alpm_pkg_should_ignore(self.handle.as_ptr(), self.as_ptr()) };
        ret != 0
    }

    pub fn filename(&self) -> &'h str {
        let name = unsafe { alpm_pkg_get_filename(self.as_ptr()) };
        unsafe { from_cstr_optional2(name) }
    }

    pub fn base(&self) -> Option<&'h str> {
        let base = unsafe { alpm_pkg_get_base(self.as_ptr()) };
        unsafe { from_cstr_optional(base) }
    }

    pub fn version(&self) -> &'h Ver {
        let version = unsafe { alpm_pkg_get_version(self.as_ptr()) };
        unsafe { Ver::from_ptr(version) }
    }

    pub fn origin(&self) -> PackageFrom {
        let origin = unsafe { alpm_pkg_get_origin(self.as_ptr()) };
        unsafe { transmute::<_alpm_pkgfrom_t, PackageFrom>(origin) }
    }

    pub fn desc(&self) -> Option<&'h str> {
        let desc = unsafe { alpm_pkg_get_desc(self.as_ptr()) };
        unsafe { from_cstr_optional(desc) }
    }

    pub fn url(&self) -> Option<&'h str> {
        let url = unsafe { alpm_pkg_get_url(self.as_ptr()) };
        unsafe { from_cstr_optional(url) }
    }

    pub fn build_date(&self) -> i64 {
        let date = unsafe { alpm_pkg_get_builddate(self.as_ptr()) };
        date as i64
    }

    pub fn install_date(&self) -> Option<i64> {
        let date = unsafe { alpm_pkg_get_installdate(self.as_ptr()) };
        if date == 0 {
            None
        } else {
            Some(date as i64)
        }
    }

    pub fn packager(&self) -> Option<&'h str> {
        let packager = unsafe { alpm_pkg_get_packager(self.as_ptr()) };
        unsafe { from_cstr_optional(packager) }
    }

    pub fn md5sum(&self) -> Option<&'h str> {
        let md5sum = unsafe { alpm_pkg_get_md5sum(self.as_ptr()) };
        unsafe { from_cstr_optional(md5sum) }
    }

    pub fn sha256sum(&self) -> Option<&'h str> {
        let sha256sum = unsafe { alpm_pkg_get_sha256sum(self.as_ptr()) };
        unsafe { from_cstr_optional(sha256sum) }
    }

    pub fn arch(&self) -> Option<&'h str> {
        let arch = unsafe { alpm_pkg_get_arch(self.as_ptr()) };
        unsafe { from_cstr_optional(arch) }
    }

    pub fn size(&self) -> i64 {
        let size = unsafe { alpm_pkg_get_size(self.as_ptr()) };
        size as i64
    }

    pub fn isize(&self) -> i64 {
        let size = unsafe { alpm_pkg_get_isize(self.as_ptr()) };
        size as i64
    }

    pub fn reason(&self) -> PackageReason {
        let reason = unsafe { alpm_pkg_get_reason(self.as_ptr()) };
        unsafe { transmute::<_alpm_pkgreason_t, PackageReason>(reason) }
    }

    pub fn validation(&self) -> PackageValidation {
        let validation = unsafe { alpm_pkg_get_validation(self.as_ptr()) };
        PackageValidation::from_bits(validation as u32).unwrap()
    }

    pub fn licenses(&self) -> AlpmList<'h, &'h str> {
        let list = unsafe { alpm_pkg_get_licenses(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn groups(&self) -> AlpmList<'h, &'h str> {
        let list = unsafe { alpm_pkg_get_groups(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn depends(&self) -> AlpmList<'h, Dep<'h>> {
        let list = unsafe { alpm_pkg_get_depends(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn optdepends(&self) -> AlpmList<'h, Dep<'h>> {
        let list = unsafe { alpm_pkg_get_optdepends(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn checkdepends(&self) -> AlpmList<'h, Dep<'h>> {
        let list = unsafe { alpm_pkg_get_checkdepends(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn makedepends(&self) -> AlpmList<'h, Dep<'h>> {
        let list = unsafe { alpm_pkg_get_makedepends(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn conflicts(&self) -> AlpmList<'h, Dep<'h>> {
        let list = unsafe { alpm_pkg_get_conflicts(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn provides(&self) -> AlpmList<'h, Dep<'h>> {
        let list = unsafe { alpm_pkg_get_provides(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn replaces(&self) -> AlpmList<'h, Dep<'h>> {
        let list = unsafe { alpm_pkg_get_replaces(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn files(&self) -> FileList {
        let files = unsafe { *alpm_pkg_get_files(self.as_ptr()) };
        unsafe { FileList::new(files) }
    }

    pub fn backup(&self) -> AlpmList<'h, Backup> {
        let list = unsafe { alpm_pkg_get_backup(self.as_ptr()) };
        unsafe { AlpmList::from_parts(self.handle, list) }
    }

    pub fn db(&self) -> Option<Db<'h>> {
        let db = unsafe { alpm_pkg_get_db(self.as_ptr()) };
        self.handle.check_null(db).ok()?;
        unsafe { Some(Db::new(self.handle, db)) }
    }

    pub fn changelog(&self) -> Result<ChangeLog> {
        let changelog = unsafe { alpm_pkg_changelog_open(self.as_ptr()) };
        self.handle.check_null(changelog)?;
        let changelog = unsafe { ChangeLog::new(*self, changelog) };
        Ok(changelog)
    }

    #[cfg(feature = "mtree")]
    pub fn mtree(&self) -> Result<MTree> {
        let archive = unsafe { alpm_pkg_mtree_open(self.as_ptr()) };
        self.handle.check_null(archive)?;

        let archive = unsafe { MTree::new(*self, archive) };

        Ok(archive)
    }

    pub fn required_by(&self) -> AlpmListMut<'h, String> {
        let list = unsafe { alpm_pkg_compute_requiredby(self.as_ptr()) };
        unsafe { AlpmListMut::from_parts(self.handle, list) }
    }

    pub fn optional_for(&self) -> AlpmListMut<'h, String> {
        let list = unsafe { alpm_pkg_compute_optionalfor(self.as_ptr()) };
        unsafe { AlpmListMut::from_parts(self.handle, list) }
    }

    pub fn base64_sig(&self) -> Option<&'h str> {
        let base64_sig = unsafe { alpm_pkg_get_base64_sig(self.as_ptr()) };
        unsafe { from_cstr_optional(base64_sig) }
    }

    pub fn has_scriptlet(&self) -> bool {
        unsafe { alpm_pkg_has_scriptlet(self.as_ptr()) != 0 }
    }

    pub fn sig(&self) -> Result<Signature> {
        let mut sig = ptr::null_mut();
        let mut len = 0;
        let ret = unsafe { alpm_pkg_get_sig(self.as_ptr(), &mut sig, &mut len) };
        self.handle.check_ret(ret)?;
        let sig = unsafe { Signature::new(sig, len) };
        Ok(sig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SigLevel;
    use std::io::Read;
    use std::mem::size_of;

    #[test]
    fn test_depends() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.register_syncdb("core", SigLevel::NONE).unwrap();
        let pkg = db.pkg("linux").unwrap();
        let depends = pkg
            .depends()
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            &depends,
            &["coreutils", "linux-firmware", "kmod", "mkinitcpio"]
        )
    }

    #[test]
    fn test_files() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.localdb();
        let pkg = db.pkg("filesystem").unwrap();
        let files = pkg.files();

        for file in files.files() {
            println!("{}", file.name());
        }

        assert!(files.contains("etc/").unwrap().is_some());
        assert_eq!(pkg.filename(), "");
    }

    #[test]
    fn test_files_null() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.register_syncdb("core", SigLevel::NONE).unwrap();
        let pkg = db.pkg("filesystem").unwrap();
        let files = pkg.files();

        assert_eq!(files.files().len(), 0);
    }

    #[test]
    fn test_groups() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.register_syncdb("core", SigLevel::NONE).unwrap();
        let pkg = db.pkg("linux").unwrap();
        let groups = pkg.groups();

        assert_eq!(&groups.iter().collect::<Vec<_>>(), &["base"],)
    }

    #[test]
    fn test_backup() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.localdb();
        let pkg = db.pkg("pacman").unwrap();
        let backup = pkg.backup();
        assert_eq!(backup.first().unwrap().name(), "etc/pacman.conf");
    }

    #[test]
    fn test_rquired_by() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.register_syncdb("extra", SigLevel::NONE).unwrap();
        let pkg = db.pkg("ostree").unwrap();
        let optional = pkg
            .required_by()
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>();
        assert_eq!(&optional, &["flatpak"]);
    }

    #[test]
    fn test_changelog() {
        let handle = Alpm::new("/", "tests/db").unwrap();
        let db = handle.localdb();
        let pkg = db.pkg("vifm").unwrap();
        let mut s = String::new();
        let mut changelog = pkg.changelog().unwrap();
        changelog.read_to_string(&mut s).unwrap();
        assert!(s.contains("2010-02-15 Jaroslav Lichtblau <svetlemodry@archlinux.org>"));
    }

    #[test]
    fn test_pkg_optimization() {
        assert!(size_of::<Pkg>() == size_of::<Option<Pkg>>());
    }
}
