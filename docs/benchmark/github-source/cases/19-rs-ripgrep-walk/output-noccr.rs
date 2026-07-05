use std::{
    cmp::Ordering,
    ffi::OsStr,
    fs::{self, FileType, Metadata},
    io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering as AtomicOrdering},
    sync::Arc,
};

use {
    crossbeam_deque::{Stealer, Worker as Deque},
    same_file::Handle,
    walkdir::{self, WalkDir},
};

use crate::{
    dir::{Ignore, IgnoreBuilder},
    gitignore::GitignoreBuilder,
    overrides::Override,
    types::Types,
    Error, PartialErrorBuilder,
};

/// A directory entry with a possible error attached.
///
/// The error typically refers to a problem parsing ignore files in a
/// particular directory.
#[derive(Clone, Debug)]
pub struct DirEntry {
    dent: DirEntryInner,
    err: Option<Error>,
}

impl DirEntry {
    /// The full path that this entry represents.
    pub fn path(&self) -> &Path {
        self.dent.path()
    }

    /// The full path that this entry represents.
    /// Analogous to [`DirEntry::path`], but moves ownership of the path.
    pub fn into_path(self) -> PathBuf {
        self.dent.into_path()
    }

    /// Whether this entry corresponds to a symbolic link or not.
    pub fn path_is_symlink(&self) -> bool {
        self.dent.path_is_symlink()
    }

    /// Returns true if and only if this entry corresponds to stdin.
    ///
    /// i.e., The entry has depth 0 and its file name is `-`.
    pub fn is_stdin(&self) -> bool {
        self.dent.is_stdin()
    }

    /// Return the metadata for the file that this entry points to.
    pub fn metadata(&self) -> Result<Metadata, Error> {
        self.dent.metadata()
    }

    /// Return the file type for the file that this entry points to.
    ///
    /// This entry doesn't have a file type if it corresponds to stdin.
    pub fn file_type(&self) -> Option<FileType> {
        self.dent.file_type()
    }

    /// Return the file name of this entry.
    ///
    /// If this entry has no file name (e.g., `/`), then the full path is
    /// returned.
    pub fn file_name(&self) -> &OsStr {
        self.dent.file_name()
    }

    /// Returns the depth at which this entry was created relative to the root.
    pub fn depth(&self) -> usize {
        self.dent.depth()
    }

    /// Returns the underlying inode number if one exists.
    ///
    /// If this entry doesn't have an inode number, then `None` is returned.
    #[cfg(unix)]
    pub fn ino(&self) -> Option<u64> {
        self.dent.ino()
    }

    /// Returns an error, if one exists, associated with processing this entry.
    ///
    /// An example of an error is one that occurred while parsing an ignore
    /// file. Errors related to traversing a directory tree itself are reported
    /// as part of yielding the directory entry, and not with this method.
    pub fn error(&self) -> Option<&Error> {
        self.err.as_ref()
    }

    /// Returns true if and only if this entry points to a directory.
    pub(crate) fn is_dir(&self) -> bool {
        self.dent.is_dir()
    }

    fn new_stdin() -> DirEntry {
        DirEntry { dent: DirEntryInner::Stdin, err: None }
    }

    fn new_walkdir(dent: walkdir::DirEntry, err: Option<Error>) -> DirEntry {
        DirEntry { dent: DirEntryInner::Walkdir(dent), err }
    }

    fn new_raw(dent: DirEntryRaw, err: Option<Error>) -> DirEntry {
        DirEntry { dent: DirEntryInner::Raw(dent), err }
    }
}

/// DirEntryInner is the implementation of DirEntry.
///
/// It specifically represents three distinct sources of directory entries:
///
/// 1. From the walkdir crate.
/// 2. Special entries that represent things like stdin.
/// 3. From a path.
///
/// Specifically, (3) has to essentially re-create the DirEntry implementation
/// from WalkDir.
#[derive(Clone, Debug)]
enum DirEntryInner {
    Stdin,
    Walkdir(walkdir::DirEntry),
    Raw(DirEntryRaw),
}

impl DirEntryInner {
    fn path(&self) -> &Path {
        use self::DirEntryInner::*;
        match *self {
            Stdin => Path::new("<stdin>"),
            Walkdir(ref x) => x.path(),
            Raw(ref x) => x.path(),
        }
    }

    fn into_path(self) -> PathBuf {
        use self::DirEntryInner::*;
        match self {
            Stdin => PathBuf::from("<stdin>"),
            Walkdir(x) => x.into_path(),
            Raw(x) => x.into_path(),
        }
    }

    fn path_is_symlink(&self) -> bool {
        use self::DirEntryInner::*;
        match *self {
            Stdin => false,
            Walkdir(ref x) => x.path_is_symlink(),
            Raw(ref x) => x.path_is_symlink(),
        }
    }

    fn is_stdin(&self) -> bool {
        match *self {
            DirEntryInner::Stdin => true,
            _ => false,
        }
    }

    fn metadata(&self) -> Result<Metadata, Error> {
        use self::DirEntryInner::*;
        match *self {
        { … 11 line(s) … }
        }
}

    fn file_type(&self) -> Option<FileType> {
        use self::DirEntryInner::*;
        match *self {
            Stdin => None,
            Walkdir(ref x) => Some(x.file_type()),
            Raw(ref x) => Some(x.file_type()),
        }
    }

    fn file_name(&self) -> &OsStr {
        use self::DirEntryInner::*;
        match *self {
            Stdin => OsStr::new("<stdin>"),
            Walkdir(ref x) => x.file_name(),
            Raw(ref x) => x.file_name(),
        }
    }

    fn depth(&self) -> usize {
        use self::DirEntryInner::*;
        match *self {
            Stdin => 0,
            Walkdir(ref x) => x.depth(),
            Raw(ref x) => x.depth(),
        }
    }

    #[cfg(unix)]
    fn ino(&self) -> Option<u64> {
        use self::DirEntryInner::*;
        use walkdir::DirEntryExt;
        match *self {
            Stdin => None,
            Walkdir(ref x) => Some(x.ino()),
            Raw(ref x) => Some(x.ino()),
        }
    }

    /// Returns true if and only if this entry points to a directory.
    fn is_dir(&self) -> bool {
        self.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
    }
}

/// DirEntryRaw is essentially copied from the walkdir crate so that we can
/// build `DirEntry`s from whole cloth in the parallel iterator.
#[derive(Clone)]
struct DirEntryRaw {
    /// The path as reported by the `fs::ReadDir` iterator (even if it's a
    /// symbolic link).
    path: PathBuf,
    /// The file type. Necessary for recursive iteration, so store it.
    ty: FileType,
    /// Is set when this entry was created from a symbolic link and the user
    /// expects the iterator to follow symbolic links.
    follow_link: bool,
    /// The depth at which this entry was generated relative to the root.
    depth: usize,
    /// The underlying inode number (Unix only).
    #[cfg(unix)]
    ino: u64,
    /// The underlying metadata (Windows only). We store this on Windows
    /// because this comes for free while reading a directory.
    #[cfg(windows)]
    metadata: fs::Metadata,
}

impl std::fmt::Debug for DirEntryRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { … 10 line(s) … }
}

impl DirEntryRaw {
    fn path(&self) -> &Path {
        &self.path
    }

    fn into_path(self) -> PathBuf {
        self.path
    }

    fn path_is_symlink(&self) -> bool {
        self.ty.is_symlink() || self.follow_link
    }

    fn metadata(&self) -> Result<Metadata, Error> {
        self.metadata_internal()
    }

    #[cfg(windows)]
    fn metadata_internal(&self) -> Result<fs::Metadata, Error> {
        if self.follow_link {
            fs::metadata(&self.path)
        } else {
            Ok(self.metadata.clone())
        }
        .map_err(|err| Error::Io(io::Error::from(err)).with_path(&self.path))
    }

    #[cfg(not(windows))]
    fn metadata_internal(&self) -> Result<fs::Metadata, Error> {
        if self.follow_link {
            fs::metadata(&self.path)
        } else {
            fs::symlink_metadata(&self.path)
        }
        .map_err(|err| Error::Io(io::Error::from(err)).with_path(&self.path))
    }

    fn file_type(&self) -> FileType {
        self.ty
    }

    fn file_name(&self) -> &OsStr {
        self.path.file_name().unwrap_or_else(|| self.path.as_os_str())
    }

    fn depth(&self) -> usize {
        self.depth
    }

    #[cfg(unix)]
    fn ino(&self) -> u64 {
        self.ino
    }

    fn from_entry(
        depth: usize,
        ent: &fs::DirEntry,
    ) -> Result<DirEntryRaw, Error> {
        let ty = ent.file_type().map_err(|err| {
            let err = Error::Io(io::Error::from(err)).with_path(ent.path());
            Error::WithDepth { depth, err: Box::new(err) }
        })?;
        DirEntryRaw::from_entry_os(depth, ent, ty)
    }

    #[cfg(windows)]
    fn from_entry_os(
        depth: usize,
        ent: &fs::DirEntry,
        ty: fs::FileType,
    ) -> Result<DirEntryRaw, Error> { … 13 line(s) … }

    #[cfg(unix)]
    fn from_entry_os(
        depth: usize,
        ent: &fs::DirEntry,
        ty: fs::FileType,
    ) -> Result<DirEntryRaw, Error> { … 11 line(s) … }

    // Placeholder implementation to allow compiling on non-standard platforms
    // (e.g. wasm32).
    #[cfg(not(any(windows, unix)))]
    fn from_entry_os(
        depth: usize,
        ent: &fs::DirEntry,
        ty: fs::FileType,
    ) -> Result<DirEntryRaw, Error> {
        Err(Error::Io(io::Error::new(
            io::ErrorKind::Other,
            "unsupported platform",
        )))
    }

    #[cfg(windows)]
    fn from_path(
        depth: usize,
        pb: PathBuf,
        link: bool,
    ) -> Result<DirEntryRaw, Error> { … 11 line(s) … }

    #[cfg(unix)]
    fn from_path(
        depth: usize,
        pb: PathBuf,
        link: bool,
    ) -> Result<DirEntryRaw, Error> { … 13 line(s) … }

    // Placeholder implementation to allow compiling on non-standard platforms
    // (e.g. wasm32).
    #[cfg(not(any(windows, unix)))]
    fn from_path(
        depth: usize,
        pb: PathBuf,
        link: bool,
    ) -> Result<DirEntryRaw, Error> {
        Err(Error::Io(io::Error::new(
            io::ErrorKind::Other,
            "unsupported platform",
        )))
    }
}

/// WalkBuilder builds a recursive directory iterator.
///
/// The builder supports a large number of configurable options. This includes
/// specific glob overrides, file type matching, toggling whether hidden
/// files are ignored or not, and of course, support for respecting gitignore
/// files.
///
/// By default, all ignore files found are respected. This includes `.ignore`,
/// `.gitignore`, `.git/info/exclude` and even your global gitignore
/// globs, usually found in `$XDG_CONFIG_HOME/git/ignore`.
///
/// Some standard recursive directory options are also supported, such as
/// limiting the recursive depth or whether to follow symbolic links (disabled
/// by default).
///
/// # Ignore rules
///
/// There are many rules that influence whether a particular file or directory
/// is skipped by this iterator. Those rules are documented here. Note that
/// the rules assume a default configuration.
///
/// * First, glob overrides are checked. If a path matches a glob override,
/// then matching stops. The path is then only skipped if the glob that matched
/// the path is an ignore glob. (An override glob is a whitelist glob unless it
/// starts with a `!`, in which case it is an ignore glob.)
/// * Second, ignore files are checked. Ignore files currently only come from
/// git ignore files (`.gitignore`, `.git/info/exclude` and the configured
/// global gitignore file), plain `.ignore` files, which have the same format
/// as gitignore files, or explicitly added ignore files. The precedence order
/// is: `.ignore`, `.gitignore`, `.git/info/exclude`, global gitignore and
/// finally explicitly added ignore files. Note that precedence between
/// different types of ignore files is not impacted by the directory hierarchy;
/// any `.ignore` file overrides all `.gitignore` files. Within each precedence
/// level, more nested ignore files have a higher precedence than less nested
/// ignore files.
/// * Third, if the previous step yields an ignore match, then all matching
/// is stopped and the path is skipped. If it yields a whitelist match, then
/// matching continues. A whitelist match can be overridden by a later matcher.
/// * Fourth, unless the path is a directory, the file type matcher is run on
/// the path. As above, if it yields an ignore match, then all matching is
/// stopped and the path is skipped. If it yields a whitelist match, then
/// matching continues.
/// * Fifth, if the path hasn't been whitelisted and it is hidden, then the
/// path is skipped.
/// * Sixth, unless the path is a directory, the size of the file is compared
/// against the max filesize limit. If it exceeds the limit, it is skipped.
/// * Seventh, if the path has made it this far then it is yielded in the
/// iterator.
#[derive(Clone)]
pub struct WalkBuilder {
    paths: Vec<PathBuf>,
    ig_builder: IgnoreBuilder,
    max_depth: Option<usize>,
    max_filesize: Option<u64>,
    follow_links: bool,
    same_file_system: bool,
    sorter: Option<Sorter>,
    threads: usize,
    skip: Option<Arc<Handle>>,
    filter: Option<Filter>,
}

#[derive(Clone)]
enum Sorter {
    ByName(Arc<dyn Fn(&OsStr, &OsStr) -> Ordering + Send + Sync + 'static>),
    ByPath(Arc<dyn Fn(&Path, &Path) -> Ordering + Send + Sync + 'static>),
}

#[derive(Clone)]
struct Filter(Arc<dyn Fn(&DirEntry) -> bool + Send + Sync + 'static>);

impl std::fmt::Debug for WalkBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { … 11 line(s) … }
}

impl WalkBuilder {
    /// Create a new builder for a recursive directory iterator for the
    /// directory given.
    ///
    /// Note that if you want to traverse multiple different directories, it
    /// is better to call `add` on this builder than to create multiple
    /// `Walk` values.
    pub fn new<P: AsRef<Path>>(path: P) -> WalkBuilder {
        WalkBuilder {
            paths: vec![path.as_ref().to_path_buf()],
            ig_builder: IgnoreBuilder::new(),
            max_depth: None,
            max_filesize: None,
            follow_links: false,
            same_file_system: false,
            sorter: None,
            threads: 0,
            skip: None,
            filter: None,
        }
    }

    /// Build a new `Walk` iterator.
    pub fn build(&self) -> Walk {
        let follow_links = self.follow_links;
        let max_depth = self.max_depth;
        { … 42 line(s) … }
        }
}

    /// Build a new `WalkParallel` iterator.
    ///
    /// Note that this *doesn't* return something that implements `Iterator`.
    /// Instead, the returned value must be run with a closure. e.g.,
    /// `builder.build_parallel().run(|| |path| println!("{:?}", path))`.
    pub fn build_parallel(&self) -> WalkParallel { … 13 line(s) … }

    /// Add a file path to the iterator.
    ///
    /// Each additional file path added is traversed recursively. This should
    /// be preferred over building multiple `Walk` iterators since this
    /// enables reusing resources across iteration.
    pub fn add<P: AsRef<Path>>(&mut self, path: P) -> &mut WalkBuilder {
        self.paths.push(path.as_ref().to_path_buf());
        self
    }

    /// The maximum depth to recurse.
    ///
    /// The default, `None`, imposes no depth restriction.
    pub fn max_depth(&mut self, depth: Option<usize>) -> &mut WalkBuilder {
        self.max_depth = depth;
        self
    }

    /// Whether to follow symbolic links or not.
    pub fn follow_links(&mut self, yes: bool) -> &mut WalkBuilder {
        self.follow_links = yes;
        self
    }

    /// Whether to ignore files above the specified limit.
    pub fn max_filesize(&mut self, filesize: Option<u64>) -> &mut WalkBuilder {
        self.max_filesize = filesize;
        self
    }

    /// The number of threads to use for traversal.
    ///
    /// Note that this only has an effect when using `build_parallel`.
    ///
    /// The default setting is `0`, which chooses the number of threads
    /// automatically using heuristics.
    pub fn threads(&mut self, n: usize) -> &mut WalkBuilder {
        self.threads = n;
        self
    }

    /// Add a global ignore file to the matcher.
    ///
    /// This has lower precedence than all other sources of ignore rules.
    ///
    /// If there was a problem adding the ignore file, then an error is
    /// returned. Note that the error may indicate *partial* failure. For
    /// example, if an ignore file contains an invalid glob, all other globs
    /// are still applied.
    pub fn add_ignore<P: AsRef<Path>>(&mut self, path: P) -> Option<Error> {
        let mut builder = GitignoreBuilder::new("");
        let mut errs = PartialErrorBuilder::default();
        { … 9 line(s) … }
        errs.into_error_option()
}

    /// Add a custom ignore file name
    ///
    /// These ignore files have higher precedence than all other ignore files.
    ///
    /// When specifying multiple names, earlier names have lower precedence than
    /// later names.
    pub fn add_custom_ignore_filename<S: AsRef<OsStr>>(
        &mut self,
        file_name: S,
    ) -> &mut WalkBuilder {
        self.ig_builder.add_custom_ignore_filename(file_name);
        self
    }

    /// Add an override matcher.
    ///
    /// By default, no override matcher is used.
    ///
    /// This overrides any previous setting.
    pub fn overrides(&mut self, overrides: Override) -> &mut WalkBuilder {
        self.ig_builder.overrides(overrides);
        self
    }

    /// Add a file type matcher.
    ///
    /// By default, no file type matcher is used.
    ///
    /// This overrides any previous setting.
    pub fn types(&mut self, types: Types) -> &mut WalkBuilder {
        self.ig_builder.types(types);
        self
    }

    /// Enables all the standard ignore filters.
    ///
    /// This toggles, as a group, all the filters that are enabled by default:
    ///
    /// - [hidden()](#method.hidden)
    /// - [parents()](#method.parents)
    /// - [ignore()](#method.ignore)
    /// - [git_ignore()](#method.git_ignore)
    /// - [git_global()](#method.git_global)
    /// - [git_exclude()](#method.git_exclude)
    ///
    /// They may still be toggled individually after calling this function.
    ///
    /// This is (by definition) enabled by default.
    pub fn standard_filters(&mut self, yes: bool) -> &mut WalkBuilder {
        self.hidden(yes)
            .parents(yes)
            .ignore(yes)
            .git_ignore(yes)
            .git_global(yes)
            .git_exclude(yes)
    }

    /// Enables ignoring hidden files.
    ///
    /// This is enabled by default.
    pub fn hidden(&mut self, yes: bool) -> &mut WalkBuilder {
        self.ig_builder.hidden(yes);
        self
    }

    /// Enables reading ignore files from parent directories.
    ///
    /// If this is enabled, then .gitignore files in parent directories of each
    /// file path given are respected. Otherwise, they are ignored.
    ///
    /// This is enabled by default.
    pub fn parents(&mut self, yes: bool) -> &mut WalkBuilder {
        self.ig_builder.parents(yes);
        self
    }

    /// Enables reading `.ignore` files.
    ///
    /// `.ignore` files have the same semantics as `gitignore` files and are
    /// supported by search tools such as ripgrep and The Silver Searcher.
    ///
    /// This is enabled by default.
    pub fn ignore(&mut self, yes: bool) -> &mut WalkBuilder {
        self.ig_builder.ignore(yes);
        self
    }

    /// Enables reading a global gitignore file, whose path is specified in
    /// git's `core.excludesFile` config option.
    ///
    /// Git's config file location is `$HOME/.gitconfig`. If `$HOME/.gitconfig`
    /// does not exist or does not specify `core.excludesFile`, then
    /// `$XDG_CONFIG_HOME/git/ignore` is read. If `$XDG_CONFIG_HOME` is not
    /// set or is empty, then `$HOME/.config/git/ignore` is used instead.
    ///
    /// This is enabled by default.
    pub fn git_global(&mut self, yes: bool) -> &mut WalkBuilder {
        self.ig_builder.git_global(yes);
        self
    }

    /// Enables reading `.gitignore` files.
    ///
    /// `.gitignore` files have match semantics as described in the `gitignore`
    /// man page.
    ///
    /// This is enabled by default.
    pub fn git_ignore(&mut self, yes: bool) -> &mut WalkBuilder {
        self.ig_builder.git_ignore(yes);
        self
    }

    /// Enables reading `.git/info/exclude` files.
    ///
    /// `.git/info/exclude` files have match semantics as described in the
    /// `gitignore` man page.
    ///
    /// This is enabled by default.
    pub fn git_exclude(&mut self, yes: bool) -> &mut WalkBuilder {
        self.ig_builder.git_exclude(yes);
        self
    }

    /// Whether a git repository is required to apply git-related ignore
    /// rules (global rules, .gitignore and local exclude rules).
    ///
    /// When disabled, git-related ignore rules are applied even when searching
    /// outside a git repository.
    pub fn require_git(&mut self, yes: bool) -> &mut WalkBuilder {
        self.ig_builder.require_git(yes);
        self
    }

    /// Process ignore files case insensitively
    ///
    /// This is disabled by default.
    pub fn ignore_case_insensitive(&mut self, yes: bool) -> &mut WalkBuilder {
        self.ig_builder.ignore_case_insensitive(yes);
        self
    }

    /// Set a function for sorting directory entries by their path.
    ///
    /// If a compare function is set, the resulting iterator will return all
    /// paths in sorted order. The compare function will be called to compare
    /// entries from the same directory.
    ///
    /// This is like `sort_by_file_name`, except the comparator accepts
    /// a `&Path` instead of the base file name, which permits it to sort by
    /// more criteria.
    ///
    /// This method will override any previous sorter set by this method or
    /// by `sort_by_file_name`.
    ///
    /// Note that this is not used in the parallel iterator.
    pub fn sort_by_file_path<F>(&mut self, cmp: F) -> &mut WalkBuilder
    where
        F: Fn(&Path, &Path) -> Ordering + Send + Sync + 'static,
    {
        self.sorter = Some(Sorter::ByPath(Arc::new(cmp)));
        self
    }

    /// Set a function for sorting directory entries by file name.
    ///
    /// If a compare function is set, the resulting iterator will return all
    /// paths in sorted order. The compare function will be called to compare
    /// names from entries from the same directory using only the name of the
    /// entry.
    ///
    /// This method will override any previous sorter set by this method or
    /// by `sort_by_file_path`.
    ///
    /// Note that this is not used in the parallel iterator.
    pub fn sort_by_file_name<F>(&mut self, cmp: F) -> &mut WalkBuilder
    where
        F: Fn(&OsStr, &OsStr) -> Ordering + Send + Sync + 'static,
    {
        self.sorter = Some(Sorter::ByName(Arc::new(cmp)));
        self
    }

    /// Do not cross file system boundaries.
    ///
    /// When this option is enabled, directory traversal will not descend into
    /// directories that are on a different file system from the root path.
    ///
    /// Currently, this option is only supported on Unix and Windows. If this
    /// option is used on an unsupported platform, then directory traversal
    /// will immediately return an error and will not yield any entries.
    pub fn same_file_system(&mut self, yes: bool) -> &mut WalkBuilder {
        self.same_file_system = yes;
        self
    }

    /// Do not yield directory entries that are believed to correspond to
    /// stdout.
    ///
    /// This is useful when a command is invoked via shell redirection to a
    /// file that is also being read. For example, `grep -r foo ./ > results`
    /// might end up trying to search `results` even though it is also writing
    /// to it, which could cause an unbounded feedback loop. Setting this
    /// option prevents this from happening by skipping over the `results`
    /// file.
    ///
    /// This is disabled by default.
    pub fn skip_stdout(&mut self, yes: bool) -> &mut WalkBuilder {
        if yes {
            self.skip = stdout_handle().map(Arc::new);
        } else {
            self.skip = None;
        }
        self
    }

    /// Yields only entries which satisfy the given predicate and skips
    /// descending into directories that do not satisfy the given predicate.
    ///
    /// The predicate is applied to all entries. If the predicate is
    /// true, iteration carries on as normal. If the predicate is false, the
    /// entry is ignored and if it is a directory, it is not descended into.
    ///
    /// Note that the errors for reading entries that may not satisfy the
    /// predicate will still be yielded.
    pub fn filter_entry<P>(&mut self, filter: P) -> &mut WalkBuilder
    where
        P: Fn(&DirEntry) -> bool + Send + Sync + 'static,
    {
        self.filter = Some(Filter(Arc::new(filter)));
        self
    }
}

/// Walk is a recursive directory iterator over file paths in one or more
/// directories.
///
/// Only file and directory paths matching the rules are returned. By default,
/// ignore files like `.gitignore` are respected. The precise matching rules
/// and precedence is explained in the documentation for `WalkBuilder`.
pub struct Walk {
    its: std::vec::IntoIter<(PathBuf, Option<WalkEventIter>)>,
    it: Option<WalkEventIter>,
    ig_root: Ignore,
    ig: Ignore,
    max_filesize: Option<u64>,
    skip: Option<Arc<Handle>>,
    filter: Option<Filter>,
}

impl Walk {
    /// Creates a new recursive directory iterator for the file path given.
    ///
    /// Note that this uses default settings, which include respecting
    /// `.gitignore` files. To configure the iterator, use `WalkBuilder`
    /// instead.
    pub fn new<P: AsRef<Path>>(path: P) -> Walk {
        WalkBuilder::new(path).build()
    }

    fn skip_entry(&self, ent: &DirEntry) -> Result<bool, Error> {
        if ent.depth() == 0 {
            return Ok(false);
        { … 30 line(s) … }
        Ok(false)
}
}

impl Iterator for Walk {
    type Item = Result<DirEntry, Error>;

    #[inline(always)]
    fn next(&mut self) -> Option<Result<DirEntry, Error>> {
        loop {
            let ev = match self.it.as_mut().and_then(|it| it.next()) {
        { … 62 line(s) … }
        }
}
}

impl std::iter::FusedIterator for Walk {}

/// WalkEventIter transforms a WalkDir iterator into an iterator that more
/// accurately describes the directory tree. Namely, it emits events that are
/// one of three types: directory, file or "exit." An "exit" event means that
/// the entire contents of a directory have been enumerated.
struct WalkEventIter {
    depth: usize,
    it: walkdir::IntoIter,
    next: Option<Result<walkdir::DirEntry, walkdir::Error>>,
}

#[derive(Debug)]
enum WalkEvent {
    Dir(walkdir::DirEntry),
    File(walkdir::DirEntry),
    Exit,
}

impl From<WalkDir> for WalkEventIter {
    fn from(it: WalkDir) -> WalkEventIter {
        WalkEventIter { depth: 0, it: it.into_iter(), next: None }
    }
}

impl Iterator for WalkEventIter {
    type Item = walkdir::Result<WalkEvent>;

    #[inline(always)]
    fn next(&mut self) -> Option<walkdir::Result<WalkEvent>> {
        let dent = self.next.take().or_else(|| self.it.next());
        let depth = match dent {
        { … 21 line(s) … }
        }
}
}

/// WalkState is used in the parallel recursive directory iterator to indicate
/// whether walking should continue as normal, skip descending into a
/// particular directory or quit the walk entirely.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WalkState {
    /// Continue walking as normal.
    Continue,
    /// If the directory entry given is a directory, don't descend into it.
    /// In all other cases, this has no effect.
    Skip,
    /// Quit the entire iterator as soon as possible.
    ///
    /// Note that this is an inherently asynchronous action. It is possible
    /// for more entries to be yielded even after instructing the iterator
    /// to quit.
    Quit,
}

impl WalkState {
    fn is_continue(&self) -> bool {
        *self == WalkState::Continue
    }

    fn is_quit(&self) -> bool {
        *self == WalkState::Quit
    }
}

/// A builder for constructing a visitor when using [`WalkParallel::visit`].
/// The builder will be called for each thread started by `WalkParallel`. The
/// visitor returned from each builder is then called for every directory
/// entry.
pub trait ParallelVisitorBuilder<'s> {
    /// Create per-thread `ParallelVisitor`s for `WalkParallel`.
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's>;
}

impl<'a, 's, P: ParallelVisitorBuilder<'s>> ParallelVisitorBuilder<'s>
    for &'a mut P
{
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        (**self).build()
    }
}

/// Receives files and directories for the current thread.
///
/// Setup for the traversal can be implemented as part of
/// [`ParallelVisitorBuilder::build`]. Teardown when traversal finishes can be
/// implemented by implementing the `Drop` trait on your traversal type.
pub trait ParallelVisitor: Send {
    /// Receives files and directories for the current thread. This is called
    /// once for every directory entry visited by traversal.
    fn visit(&mut self, entry: Result<DirEntry, Error>) -> WalkState;
}

struct FnBuilder<F> {
    builder: F,
}

impl<'s, F: FnMut() -> FnVisitor<'s>> ParallelVisitorBuilder<'s>
    for FnBuilder<F>
{
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        let visitor = (self.builder)();
        Box::new(FnVisitorImp { visitor })
    }
}

type FnVisitor<'s> =
    Box<dyn FnMut(Result<DirEntry, Error>) -> WalkState + Send + 's>;

struct FnVisitorImp<'s> {
    visitor: FnVisitor<'s>,
}

impl<'s> ParallelVisitor for FnVisitorImp<'s> {
    fn visit(&mut self, entry: Result<DirEntry, Error>) -> WalkState {
        (self.visitor)(entry)
    }
}

/// WalkParallel is a parallel recursive directory iterator over files paths
/// in one or more directories.
///
/// Only file and directory paths matching the rules are returned. By default,
/// ignore files like `.gitignore` are respected. The precise matching rules
/// and precedence is explained in the documentation for `WalkBuilder`.
///
/// Unlike `Walk`, this uses multiple threads for traversing a directory.
pub struct WalkParallel {
    paths: std::vec::IntoIter<PathBuf>,
    ig_root: Ignore,
    max_filesize: Option<u64>,
    max_depth: Option<usize>,
    follow_links: bool,
    same_file_system: bool,
    threads: usize,
    skip: Option<Arc<Handle>>,
    filter: Option<Filter>,
}

impl WalkParallel {
    /// Execute the parallel recursive directory iterator. `mkf` is called
    /// for each thread used for iteration. The function produced by `mkf`
    /// is then in turn called for each visited file path.
    pub fn run<'s, F>(self, mkf: F)
    where
        F: FnMut() -> FnVisitor<'s>,
    {
        self.visit(&mut FnBuilder { builder: mkf })
    }

    /// Execute the parallel recursive directory iterator using a custom
    /// visitor.
    ///
    /// The builder given is used to construct a visitor for every thread
    /// used by this traversal. The visitor returned from each builder is then
    /// called for every directory entry seen by that thread.
    ///
    /// Typically, creating a custom visitor is useful if you need to perform
    /// some kind of cleanup once traversal is finished. This can be achieved
    /// by implementing `Drop` for your builder (or for your visitor, if you
    /// want to execute cleanup for every thread that is launched).
    ///
    /// For example, each visitor might build up a data structure of results
    /// corresponding to the directory entries seen for each thread. Since each
    /// visitor runs on only one thread, this build-up can be done without
    /// synchronization. Then, once traversal is complete, all of the results
    /// can be merged together into a single data structure.
    pub fn visit(mut self, builder: &mut dyn ParallelVisitorBuilder<'_>) {
        let threads = self.threads();
        let mut stack = vec![];
        { … 71 line(s) … }
        });
}

    fn threads(&self) -> usize {
        if self.threads == 0 {
            2
        } else {
            self.threads
        }
    }
}

/// Message is the set of instructions that a worker knows how to process.
enum Message {
    /// A work item corresponds to a directory that should be descended into.
    /// Work items for entries that should be skipped or ignored should not
    /// be produced.
    Work(Work),
    /// This instruction indicates that the worker should quit.
    Quit,
}

/// A unit of work for each worker to process.
///
/// Each unit of work corresponds to a directory that should be descended
/// into.
struct Work {
    /// The directory entry.
    dent: DirEntry,
    /// Any ignore matchers that have been built for this directory's parents.
    ignore: Ignore,
    /// The root device number. When present, only files with the same device
    /// number should be considered.
    root_device: Option<u64>,
}

impl Work {
    /// Returns true if and only if this work item is a directory.
    fn is_dir(&self) -> bool {
        self.dent.is_dir()
    }

    /// Returns true if and only if this work item is a symlink.
    fn is_symlink(&self) -> bool {
        self.dent.file_type().map_or(false, |ft| ft.is_symlink())
    }

    /// Adds ignore rules for parent directories.
    ///
    /// Note that this only applies to entries at depth 0. On all other
    /// entries, this is a no-op.
    fn add_parents(&mut self) -> Option<Error> { … 10 line(s) … }

    /// Reads the directory contents of this work item and adds ignore
    /// rules for this directory.
    ///
    /// If there was a problem with reading the directory contents, then
    /// an error is returned. If there was a problem reading the ignore
    /// rules for this directory, then the error is attached to this
    /// work item's directory entry.
    fn read_dir(&mut self) -> Result<fs::ReadDir, Error> {
        let readdir = match fs::read_dir(self.dent.path()) {
            Ok(readdir) => readdir,
        { … 10 line(s) … }
        Ok(readdir)
}
}

/// A work-stealing stack.
#[derive(Debug)]
struct Stack {
    /// This thread's index.
    index: usize,
    /// The thread-local stack.
    deque: Deque<Message>,
    /// The work stealers.
    stealers: Arc<[Stealer<Message>]>,
}

impl Stack {
    /// Create a work-stealing stack for each thread. The given messages
    /// correspond to the initial paths to start the search at. They will
    /// be distributed automatically to each stack in a round-robin fashion.
    fn new_for_each_thread(threads: usize, init: Vec<Message>) -> Vec<Stack> {
        // Using new_lifo() ensures each worker operates depth-first, not
        // breadth-first. We do depth-first because a breadth first traversal
        { … 20 line(s) … }
        stacks
}

    /// Push a message.
    fn push(&self, msg: Message) {
        self.deque.push(msg);
    }

    /// Pop a message.
    fn pop(&self) -> Option<Message> {
        self.deque.pop().or_else(|| self.steal())
    }

    /// Steal a message from another queue.
    fn steal(&self) -> Option<Message> { … 13 line(s) … }
}

/// A worker is responsible for descending into directories, updating the
/// ignore matchers, producing new work and invoking the caller's callback.
///
/// Note that a worker is *both* a producer and a consumer.
struct Worker<'s> {
    /// The caller's callback.
    visitor: Box<dyn ParallelVisitor + 's>,
    /// A work-stealing stack of work to do.
    ///
    /// We use a stack instead of a channel because a stack lets us visit
    /// directories in depth first order. This can substantially reduce peak
    /// memory usage by keeping both the number of file paths and gitignore
    /// matchers in memory lower.
    stack: Stack,
    /// Whether all workers should terminate at the next opportunity. Note
    /// that we need this because we don't want other `Work` to be done after
    /// we quit. We wouldn't need this if have a priority channel.
    quit_now: Arc<AtomicBool>,
    /// The number of currently active workers.
    active_workers: Arc<AtomicUsize>,
    /// The maximum depth of directories to descend. A value of `0` means no
    /// descension at all.
    max_depth: Option<usize>,
    /// The maximum size a searched file can be (in bytes). If a file exceeds
    /// this size it will be skipped.
    max_filesize: Option<u64>,
    /// Whether to follow symbolic links or not. When this is enabled, loop
    /// detection is performed.
    follow_links: bool,
    /// A file handle to skip, currently is either `None` or stdout, if it's
    /// a file and it has been requested to skip files identical to stdout.
    skip: Option<Arc<Handle>>,
    /// A predicate applied to dir entries. If true, the entry and all
    /// children will be skipped.
    filter: Option<Filter>,
}

impl<'s> Worker<'s> {
    /// Runs this worker until there is no more work left to do.
    ///
    /// The worker will call the caller's callback for all entries that aren't
    /// skipped by the ignore matcher.
    fn run(mut self) {
        while let Some(work) = self.get_work() {
            if let WalkState::Quit = self.run_one(work) {
                self.quit_now();
            }
        }
    }

    fn run_one(&mut self, mut work: Work) -> WalkState {
        // If the work is not a directory, then we can just execute the
        // caller's callback immediately and move on.
        { … 63 line(s) … }
        WalkState::Continue
}

    /// Decides whether to submit the given directory entry as a file to
    /// search.
    ///
    /// If the entry is a path that should be ignored, then this is a no-op.
    /// Otherwise, the entry is pushed on to the queue. (The actual execution
    /// of the callback happens in `run_one`.)
    ///
    /// If an error occurs while reading the entry, then it is sent to the
    /// caller's callback.
    ///
    /// `ig` is the `Ignore` matcher for the parent directory. `depth` should
    /// be the depth of this entry. `result` should be the item yielded by
    /// a directory iterator.
    fn generate_work(
        &mut self,
        ig: &Ignore,
        depth: usize,
        root_device: Option<u64>,
        result: Result<fs::DirEntry, io::Error>,
    ) -> WalkState {
        let fs_dent = match result {
            Ok(fs_dent) => fs_dent,
        { … 60 line(s) … }
        WalkState::Continue
}

    /// Returns the next directory to descend into.
    ///
    /// If all work has been exhausted, then this returns None. The worker
    /// should then subsequently quit.
    fn get_work(&mut self) -> Option<Work> {
        let mut value = self.recv();
        loop {
        { … 43 line(s) … }
        }
}

    /// Indicates that all workers should quit immediately.
    fn quit_now(&self) {
        self.quit_now.store(true, AtomicOrdering::SeqCst);
    }

    /// Returns true if this worker should quit immediately.
    fn is_quit_now(&self) -> bool {
        self.quit_now.load(AtomicOrdering::SeqCst)
    }

    /// Send work.
    fn send(&self, work: Work) {
        self.stack.push(Message::Work(work));
    }

    /// Send a quit message.
    fn send_quit(&self) {
        self.stack.push(Message::Quit);
    }

    /// Receive work.
    fn recv(&self) -> Option<Message> {
        self.stack.pop()
    }

    /// Deactivates a worker and returns the number of currently active workers.
    fn deactivate_worker(&self) -> usize {
        self.active_workers.fetch_sub(1, AtomicOrdering::Acquire) - 1
    }

    /// Reactivates a worker.
    fn activate_worker(&self) {
        self.active_workers.fetch_add(1, AtomicOrdering::Release);
    }
}

fn check_symlink_loop(
    ig_parent: &Ignore,
    child_path: &Path,
    child_depth: usize,
) -> Result<(), Error> {
    let hchild = Handle::from_path(child_path).map_err(|err| {
        Error::from(err).with_path(child_path).with_depth(child_depth)
    { … 13 line(s) … }
    Ok(())
}

// Before calling this function, make sure that you ensure that is really
// necessary as the arguments imply a file stat.
fn skip_filesize(
    max_filesize: u64,
    path: &Path,
    ent: &Option<Metadata>,
) -> bool {
    let filesize = match *ent {
        Some(ref md) => Some(md.len()),
    { … 12 line(s) … }
    }
}

fn should_skip_entry(ig: &Ignore, dent: &DirEntry) -> bool { … 12 line(s) … }

/// Returns a handle to stdout for filtering search.
///
/// A handle is returned if and only if stdout is being redirected to a file.
/// The handle returned corresponds to that file.
///
/// This can be used to ensure that we do not attempt to search a file that we
/// may also be writing to.
fn stdout_handle() -> Option<Handle> {
    let h = match Handle::stdout() {
        Err(_) => return None,
    { … 9 line(s) … }
    Some(h)
}

/// Returns true if and only if the given directory entry is believed to be
/// equivalent to the given handle. If there was a problem querying the path
/// for information to determine equality, then that error is returned.
fn path_equals(dent: &DirEntry, handle: &Handle) -> Result<bool, Error> {
    #[cfg(unix)]
    fn never_equal(dent: &DirEntry, handle: &Handle) -> bool {
    { … 15 line(s) … }
        .map_err(|err| Error::Io(err).with_path(dent.path()))
}

/// Returns true if the given walkdir entry corresponds to a directory.
///
/// This is normally just `dent.file_type().is_dir()`, but when we aren't
/// following symlinks, the root directory entry may be a symlink to a
/// directory that we *do* follow---by virtue of it being specified by the user
/// explicitly. In that case, we need to follow the symlink and query whether
/// it's a directory or not. But we only do this for root entries to avoid an
/// additional stat check in most cases.
fn walkdir_is_dir(dent: &walkdir::DirEntry) -> bool {
    if dent.file_type().is_dir() {
        return true;
    }
    if !dent.file_type().is_symlink() || dent.depth() > 0 {
        return false;
    }
    dent.path().metadata().ok().map_or(false, |md| md.file_type().is_dir())
}

/// Returns true if and only if the given path is on the same device as the
/// given root device.
fn is_same_file_system(root_device: u64, path: &Path) -> Result<bool, Error> {
    let dent_device =
        device_num(path).map_err(|err| Error::Io(err).with_path(path))?;
    Ok(root_device == dent_device)
}

#[cfg(unix)]
fn device_num<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    use std::os::unix::fs::MetadataExt;

    path.as_ref().metadata().map(|md| md.dev())
}

#[cfg(windows)]
fn device_num<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    use winapi_util::{file, Handle};

    let h = Handle::from_path_any(path)?;
    file::information(h).map(|info| info.volume_serial_number())
}

#[cfg(not(any(unix, windows)))]
fn device_num<P: AsRef<Path>>(_: P) -> io::Result<u64> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "walkdir: same_file_system option not supported on this platform",
    ))
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use std::sync::{Arc, Mutex};

    use super::{DirEntry, WalkBuilder, WalkState};
    use crate::tests::TempDir;

    fn wfile<P: AsRef<Path>>(path: P, contents: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    fn wfile_size<P: AsRef<Path>>(path: P, size: u64) {
        let file = File::create(path).unwrap();
        file.set_len(size).unwrap();
    }

    #[cfg(unix)]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) {
        use std::os::unix::fs::symlink;
        symlink(src, dst).unwrap();
    }

    fn mkdirp<P: AsRef<Path>>(path: P) {
        fs::create_dir_all(path).unwrap();
    }

    fn normal_path(unix: &str) -> String {
        if cfg!(windows) {
            unix.replace("\\", "/")
        } else {
            unix.to_string()
        }
    }

    fn walk_collect(prefix: &Path, builder: &WalkBuilder) -> Vec<String> {
        let mut paths = vec![];
        for result in builder.build() {
        { … 11 line(s) … }
        paths
}

    fn walk_collect_parallel(
        prefix: &Path,
        builder: &WalkBuilder,
    ) -> Vec<String> { … 12 line(s) … }

    fn walk_collect_entries_parallel(builder: &WalkBuilder) -> Vec<DirEntry> {
        let dents = Arc::new(Mutex::new(vec![]));
        builder.build_parallel().run(|| {
        { … 10 line(s) … }
        dents.to_vec()
}

    fn mkpaths(paths: &[&str]) -> Vec<String> {
        let mut paths: Vec<_> = paths.iter().map(|s| s.to_string()).collect();
        paths.sort();
        paths
    }

    fn tmpdir() -> TempDir {
        TempDir::new().unwrap()
    }

    fn assert_paths(prefix: &Path, builder: &WalkBuilder, expected: &[&str]) {
        let got = walk_collect(prefix, builder);
        assert_eq!(got, mkpaths(expected), "single threaded");
        let got = walk_collect_parallel(prefix, builder);
        assert_eq!(got, mkpaths(expected), "parallel");
    }

    #[test]
    fn no_ignores() { … 13 line(s) … }

    #[test]
    fn custom_ignore() {
        let td = tmpdir();
        let custom_ignore = ".customignore";
        { … 9 line(s) … }
        assert_paths(td.path(), &builder, &["bar", "a", "a/bar"]);
}

    #[test]
    fn custom_ignore_exclusive_use() {
        let td = tmpdir();
        let custom_ignore = ".customignore";
        { … 13 line(s) … }
        assert_paths(td.path(), &builder, &["bar", "a", "a/bar"]);
}

    #[test]
    fn gitignore() {
        let td = tmpdir();
        mkdirp(td.path().join(".git"));
        { … 11 line(s) … }
        );
}

    #[test]
    fn explicit_ignore() {
        let td = tmpdir();
        let igpath = td.path().join(".not-an-ignore");
        { … 9 line(s) … }
        assert_paths(td.path(), &builder, &["bar", "a", "a/bar"]);
}

    #[test]
    fn explicit_ignore_exclusive_use() {
        let td = tmpdir();
        let igpath = td.path().join(".not-an-ignore");
        { … 14 line(s) … }
        );
}

    #[test]
    fn gitignore_parent() { … 11 line(s) … }

    #[test]
    fn max_depth() {
        let td = tmpdir();
        mkdirp(td.path().join("a/b/c"));
        { … 17 line(s) … }
        );
}

    #[test]
    fn max_filesize() {
        let td = tmpdir();
        mkdirp(td.path().join("a/b"));
        { … 27 line(s) … }
        );
}

    #[cfg(unix)] // because symlinks on windows are weird
    #[test]
    fn symlinks() {
        let td = tmpdir();
        mkdirp(td.path().join("a/b"));
        { … 9 line(s) … }
        );
}

    #[cfg(unix)] // because symlinks on windows are weird
    #[test]
    fn first_path_not_symlink() {
        let td = tmpdir();
        mkdirp(td.path().join("foo"));
        { … 13 line(s) … }
        assert!(!dents[0].path_is_symlink());
}

    #[cfg(unix)] // because symlinks on windows are weird
    #[test]
    fn symlink_loop() {
        let td = tmpdir();
        mkdirp(td.path().join("a/b"));
        symlink(td.path().join("a"), td.path().join("a/b/c"));

        let mut builder = WalkBuilder::new(td.path());
        assert_paths(td.path(), &builder, &["a", "a/b", "a/b/c"]);
        assert_paths(td.path(), &builder.follow_links(true), &["a", "a/b"]);
    }

    // It's a little tricky to test the 'same_file_system' option since
    // we need an environment with more than one file system. We adopt a
    // heuristic where /sys is typically a distinct volume on Linux and roll
    // with that.
    #[test]
    #[cfg(target_os = "linux")]
    fn same_file_system() {
        use super::device_num;

        { … 22 line(s) … }
        assert_paths(td.path(), &builder, &["same_file", "same_file/alink"]);
}

    #[cfg(target_os = "linux")]
    #[test]
    fn no_read_permissions() {
        let dir_path = Path::new("/root");

        { … 11 line(s) … }
        assert_paths(dir_path.parent().unwrap(), &builder, &["root"]);
}

    #[test]
    fn filter() {
        let td = tmpdir();
        mkdirp(td.path().join("a/b/c"));
        { … 15 line(s) … }
        );
}
}