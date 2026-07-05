// Copyright (c) 2011 The LevelDB Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

#include "db/version_set.h"

#include <algorithm>
#include <cstdio>

#include "db/filename.h"
#include "db/log_reader.h"
#include "db/log_writer.h"
#include "db/memtable.h"
#include "db/table_cache.h"
#include "leveldb/env.h"
#include "leveldb/table_builder.h"
#include "table/merger.h"
#include "table/two_level_iterator.h"
#include "util/coding.h"
#include "util/logging.h"

namespace leveldb {

static size_t TargetFileSize(const Options* options) {
  return options->max_file_size;
}

// Maximum bytes of overlaps in grandparent (i.e., level+2) before we
// stop building a single file in a level->level+1 compaction.
static int64_t MaxGrandParentOverlapBytes(const Options* options) {
  return 10 * TargetFileSize(options);
}

// Maximum number of bytes in all compacted files.  We avoid expanding
// the lower level file set of a compaction if it would make the
// total compaction cover more than this many bytes.
static int64_t ExpandedCompactionByteSizeLimit(const Options* options) {
  return 25 * TargetFileSize(options);
}

static double MaxBytesForLevel(const Options* options, int level) {
    { … 11 line(s) … ⟦tj:873bcdf928379dc21a51cd4e31ae0e12⟧ }

static uint64_t MaxFileSizeForLevel(const Options* options, int level) {
  // We could vary per level to reduce number of files?
  return TargetFileSize(options);
}

static int64_t TotalFileSize(const std::vector<FileMetaData*>& files) {
    { … 6 line(s) … ⟦tj:59507a17ad45472cc8e8f4f55ec6e431⟧ }

Version::~Version() {
    { … 18 line(s) … ⟦tj:58546aac973ed10e392a4b255775f2ea⟧ }

int FindFile(const InternalKeyComparator& icmp,
             const std::vector<FileMetaData*>& files, const Slice& key) {
    { … 17 line(s) … ⟦tj:75f99b6e34275995e61cc57845ccb1d7⟧ }

static bool AfterFile(const Comparator* ucmp, const Slice* user_key,
                      const FileMetaData* f) {
    { … 4 line(s) … ⟦tj:71bf29c41db7f44950f0f97d81d42a90⟧ }

static bool BeforeFile(const Comparator* ucmp, const Slice* user_key,
                       const FileMetaData* f) {
    { … 4 line(s) … ⟦tj:df50d836b778b5eafa58be802fad1bf0⟧ }

bool SomeFileOverlapsRange(const InternalKeyComparator& icmp,
                           bool disjoint_sorted_files,
                           const std::vector<FileMetaData*>& files,
                           const Slice* smallest_user_key,
                           const Slice* largest_user_key) {
    { … 31 line(s) … ⟦tj:cd8d3ca1029d91c56b17d94d91b17206⟧ }

// An internal iterator.  For a given version/level pair, yields
// information about the files in the level.  For a given entry, key()
// is the largest key that occurs in the file, and value() is an
// 16-byte value containing the file number and file size, both
// encoded using EncodeFixed64.
class Version::LevelFileNumIterator : public Iterator {
    { … 45 line(s) … ⟦tj:efbbcdc3cbcb2d9587ae9eda19fd16bd⟧ }

static Iterator* GetFileIterator(void* arg, const ReadOptions& options,
                                 const Slice& file_value) {
    { … 9 line(s) … ⟦tj:51a3da8d36555472831159c02f2c0dde⟧ }

Iterator* Version::NewConcatenatingIterator(const ReadOptions& options,
                                            int level) const {
    { … 4 line(s) … ⟦tj:e7d291061b149aea10e443ea05d405ba⟧ }

void Version::AddIterators(const ReadOptions& options,
                           std::vector<Iterator*>* iters) {
    { … 15 line(s) … ⟦tj:d242980092d383cfe83c6306f57f8ca1⟧ }

// Callback from TableCache::Get()
namespace {
enum SaverState {
    { … 5 line(s) … ⟦tj:b734d1501cec593bc816533009d5a043⟧ }
struct Saver {
    { … 5 line(s) … ⟦tj:70ccf6071a9381cd7123c45767fc68ed⟧ }
}  // namespace
static void SaveValue(void* arg, const Slice& ikey, const Slice& v) {
    { … 13 line(s) … ⟦tj:68ac32b0f577ee2668a9b631da57b0e4⟧ }

static bool NewestFirst(FileMetaData* a, FileMetaData* b) {
  return a->number > b->number;
}

void Version::ForEachOverlapping(Slice user_key, Slice internal_key, void* arg,
                                 bool (*func)(void*, int, FileMetaData*)) {
    { … 40 line(s) … ⟦tj:deaa8530f22db8e11e83e8e6248293d5⟧ }

Status Version::Get(const ReadOptions& options, const LookupKey& k,
                    std::string* value, GetStats* stats) {
    { … 75 line(s) … ⟦tj:62b56b033d8ef3e6dd9e0852e82b53b9⟧ }

bool Version::UpdateStats(const GetStats& stats) {
    { … 11 line(s) … ⟦tj:47cce51bf31b98403ddd767a8449a31a⟧ }

bool Version::RecordReadSample(Slice internal_key) {
    { … 36 line(s) … ⟦tj:dd6649ea833204c9e035b69ce99729e9⟧ }

void Version::Ref() { ++refs_; }

void Version::Unref() {
    { … 7 line(s) … ⟦tj:42bfb401f6bc65a3229f465b9fec01ee⟧ }

bool Version::OverlapInLevel(int level, const Slice* smallest_user_key,
                             const Slice* largest_user_key) {
  return SomeFileOverlapsRange(vset_->icmp_, (level > 0), files_[level],
                               smallest_user_key, largest_user_key);
}

int Version::PickLevelForMemTableOutput(const Slice& smallest_user_key,
                                        const Slice& largest_user_key) {
    { … 24 line(s) … ⟦tj:335944fcc243083c94ae5d0a57443028⟧ }

// Store in "*inputs" all files in "level" that overlap [begin,end]
void Version::GetOverlappingInputs(int level, const InternalKey* begin,
                                   const InternalKey* end,
                                   std::vector<FileMetaData*>* inputs) {
    { … 38 line(s) … ⟦tj:56489f8e56a14eb2bbf98e6028f94668⟧ }

std::string Version::DebugString() const {
    { … 24 line(s) … ⟦tj:2d7df3c0f6de759aebda2ed478e099a5⟧ }

// A helper class so we can efficiently apply a whole sequence
// of edits to a particular state without creating intermediate
// Versions that contain full copies of the intermediate state.
class VersionSet::Builder {
    { … 162 line(s) … ⟦tj:4a4c1dc325d015775248cedfc59f5562⟧ }

VersionSet::VersionSet(const std::string& dbname, const Options* options,
                       TableCache* table_cache,
                       const InternalKeyComparator* cmp)
    : env_(options->env),
      dbname_(dbname),
      options_(options),
      table_cache_(table_cache),
      icmp_(*cmp),
      next_file_number_(2),
      manifest_file_number_(0),  // Filled by Recover()
      last_sequence_(0),
      log_number_(0),
      prev_log_number_(0),
      descriptor_file_(nullptr),
      descriptor_log_(nullptr),
      dummy_versions_(this),
      current_(nullptr) {
  AppendVersion(new Version(this));
}

VersionSet::~VersionSet() {
    { … 5 line(s) … ⟦tj:af71061b2f91b1309e6f0278d8cb8e2d⟧ }

void VersionSet::AppendVersion(Version* v) {
    { … 15 line(s) … ⟦tj:50fd330679d58e5597d0e81029eb2e27⟧ }

Status VersionSet::LogAndApply(VersionEdit* edit, port::Mutex* mu) {
    { … 83 line(s) … ⟦tj:dcc2c0ae7e4f35ffeeb6b5c15f21ba23⟧ }

Status VersionSet::Recover(bool* save_manifest) {
    { … 124 line(s) … ⟦tj:2ec9cb62cb02cdd92911259eb2fa8c96⟧ }
    std::string error = s.ToString();
    Log(options_->info_log, "Error recovering version set with %d records: %s",
        read_records, error.c_str());
    { … 4 line(s) … ⟦tj:51af1783b1a3046d08e2ee22f39373b3⟧ }

bool VersionSet::ReuseManifest(const std::string& dscname,
                               const std::string& dscbase) {
    { … 28 line(s) … ⟦tj:0d8250b3ddac0a4062c19d8e882c5461⟧ }

void VersionSet::MarkFileNumberUsed(uint64_t number) {
    { … 4 line(s) … ⟦tj:50c1cf2208adaee3244127cc78d0c895⟧ }

void VersionSet::Finalize(Version* v) {
    { … 36 line(s) … ⟦tj:6ec7437f61291f70204fb4d33afee7fb⟧ }

Status VersionSet::WriteSnapshot(log::Writer* log) {
  // TODO: Break up into multiple records to reduce memory usage on recovery?
    { … 27 line(s) … ⟦tj:54e82cca3ba88efc0b6854ba318e8731⟧ }

int VersionSet::NumLevelFiles(int level) const {
    { … 4 line(s) … ⟦tj:3b0b7c19522d4defd5f859c476cb7d5d⟧ }

const char* VersionSet::LevelSummary(LevelSummaryStorage* scratch) const {
    { … 10 line(s) … ⟦tj:8c54311fedbd804f7b50283129d23f64⟧ }

uint64_t VersionSet::ApproximateOffsetOf(Version* v, const InternalKey& ikey) {
    { … 30 line(s) … ⟦tj:5b3a14e0661ff52709c146bbc1f79674⟧ }

void VersionSet::AddLiveFiles(std::set<uint64_t>* live) {
    { … 10 line(s) … ⟦tj:97af51576cb99e22317df1f2c30af11d⟧ }

int64_t VersionSet::NumLevelBytes(int level) const {
    { … 4 line(s) … ⟦tj:a93f35c89da2751e5d8bf86768f89ef7⟧ }

int64_t VersionSet::MaxNextLevelOverlappingBytes() {
    { … 15 line(s) … ⟦tj:384b6cf33b9d6e2fe77c4882d86574bc⟧ }

// Stores the minimal range that covers all entries in inputs in
// *smallest, *largest.
// REQUIRES: inputs is not empty
void VersionSet::GetRange(const std::vector<FileMetaData*>& inputs,
                          InternalKey* smallest, InternalKey* largest) {
    { … 18 line(s) … ⟦tj:d0f46d3f312d0caefece894bc665cee9⟧ }

// Stores the minimal range that covers all entries in inputs1 and inputs2
// in *smallest, *largest.
// REQUIRES: inputs is not empty
void VersionSet::GetRange2(const std::vector<FileMetaData*>& inputs1,
                           const std::vector<FileMetaData*>& inputs2,
                           InternalKey* smallest, InternalKey* largest) {
    { … 4 line(s) … ⟦tj:91ac792699b660346821edf3ea70f6ad⟧ }

Iterator* VersionSet::MakeInputIterator(Compaction* c) {
    { … 6 line(s) … ⟦tj:dc2e77fb1cefe03506220a2b78f52316⟧ }
  // TODO(opt): use concatenating iterator for level-0 if there is no overlap
    { … 24 line(s) … ⟦tj:f3080a1dea265b41775c3bc54c6bd6c1⟧ }

Compaction* VersionSet::PickCompaction() {
    { … 52 line(s) … ⟦tj:bf87e53501fb1a34f56aef521d100275⟧ }

// Finds the largest key in a vector of files. Returns true if files it not
// empty.
bool FindLargestKey(const InternalKeyComparator& icmp,
                    const std::vector<FileMetaData*>& files,
                    InternalKey* largest_key) {
    { … 12 line(s) … ⟦tj:04c8574778d96a8d511b2129f050a36c⟧ }

// Finds minimum file b2=(l2, u2) in level file for which l2 > u1 and
// user_key(l2) = user_key(u1)
FileMetaData* FindSmallestBoundaryFile(
    const InternalKeyComparator& icmp,
    const std::vector<FileMetaData*>& level_files,
    const InternalKey& largest_key) {
    { … 15 line(s) … ⟦tj:6f4ab249f64df1f0b9911f52dba1435b⟧ }

// Extracts the largest file b1 from |compaction_files| and then searches for a
// b2 in |level_files| for which user_key(u1) = user_key(l2). If it finds such a
// file b2 (known as a boundary file) it adds it to |compaction_files| and then
// searches again using this new upper bound.
//
// If there are two blocks, b1=(l1, u1) and b2=(l2, u2) and
// user_key(u1) = user_key(l2), and if we compact b1 but not b2 then a
// subsequent get operation will yield an incorrect result because it will
// return the record from b2 in level i rather than from b1 because it searches
// level by level for records matching the supplied user key.
//
// parameters:
//   in     level_files:      List of files to search for boundary files.
//   in/out compaction_files: List of files to extend by adding boundary files.
void AddBoundaryInputs(const InternalKeyComparator& icmp,
                       const std::vector<FileMetaData*>& level_files,
                       std::vector<FileMetaData*>* compaction_files) {
    { … 21 line(s) … ⟦tj:15d690fc0ddd700ba6cf16bc137a2043⟧ }

void VersionSet::SetupOtherInputs(Compaction* c) {
    { … 59 line(s) … ⟦tj:7f97af5cf1bdec9a9dd7983c7585e510⟧ }

Compaction* VersionSet::CompactRange(int level, const InternalKey* begin,
                                     const InternalKey* end) {
    { … 30 line(s) … ⟦tj:8ae5402c9049900602860356848d4437⟧ }

Compaction::Compaction(const Options* options, int level)
    : level_(level),
      max_output_file_size_(MaxFileSizeForLevel(options, level)),
      input_version_(nullptr),
      grandparent_index_(0),
      seen_key_(false),
      overlapped_bytes_(0) {
    { … 4 line(s) … ⟦tj:74d7b586cb693366e8fb6f9dcdbe1458⟧ }

Compaction::~Compaction() {
    { … 4 line(s) … ⟦tj:1eeb4ee6423260426eb5a2f4f4c0cda9⟧ }

bool Compaction::IsTrivialMove() const {
    { … 8 line(s) … ⟦tj:dcc71e4fb4403414a55f43d366575ad0⟧ }

void Compaction::AddInputDeletions(VersionEdit* edit) {
    { … 6 line(s) … ⟦tj:97a5adc9dacd33dec6be8e07eaa16b43⟧ }

bool Compaction::IsBaseLevelForKey(const Slice& user_key) {
    { … 19 line(s) … ⟦tj:70d5493406cbec2d1981f0155d1872b9⟧ }

bool Compaction::ShouldStopBefore(const Slice& internal_key) {
    { … 22 line(s) … ⟦tj:f76d0c65dca9ffe28d547ba40d095c6f⟧ }

void Compaction::ReleaseInputs() {
    { … 5 line(s) … ⟦tj:e8c3185afe1111935c9b6c7cccfc3e9d⟧ }

}  // namespace leveldb
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (51353 bytes) is available by calling tinyjuice_retrieve with token "4a3029cca815fe176d716fd86461aa65" (marker ⟦tj:4a3029cca815fe176d716fd86461aa65⟧)]