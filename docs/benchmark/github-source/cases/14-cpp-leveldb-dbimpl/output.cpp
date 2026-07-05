// Copyright (c) 2011 The LevelDB Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

#include "db/db_impl.h"

#include <algorithm>
#include <atomic>
#include <cstdint>
#include <cstdio>
#include <set>
#include <string>
#include <vector>

#include "db/builder.h"
#include "db/db_iter.h"
#include "db/dbformat.h"
#include "db/filename.h"
#include "db/log_reader.h"
#include "db/log_writer.h"
#include "db/memtable.h"
#include "db/table_cache.h"
#include "db/version_set.h"
#include "db/write_batch_internal.h"
#include "leveldb/db.h"
#include "leveldb/env.h"
#include "leveldb/status.h"
#include "leveldb/table.h"
#include "leveldb/table_builder.h"
#include "port/port.h"
#include "table/block.h"
#include "table/merger.h"
#include "table/two_level_iterator.h"
#include "util/coding.h"
#include "util/logging.h"
#include "util/mutexlock.h"

namespace leveldb {

const int kNumNonTableCacheFiles = 10;

// Information kept for every waiting writer
struct DBImpl::Writer {
    { … 9 line(s) … ⟦tj:0253f10d83ac3c89e9d2a51cd4862390⟧ }

struct DBImpl::CompactionState {
    { … 32 line(s) … ⟦tj:952a18fd6e146185ff0d8cc68352a382⟧ }

// Fix user-supplied options to be reasonable
template <class T, class V>
static void ClipToRange(T* ptr, V minvalue, V maxvalue) {
  if (static_cast<V>(*ptr) > maxvalue) *ptr = maxvalue;
  if (static_cast<V>(*ptr) < minvalue) *ptr = minvalue;
}
Options SanitizeOptions(const std::string& dbname,
                        const InternalKeyComparator* icmp,
                        const InternalFilterPolicy* ipolicy,
                        const Options& src) {
    { … 22 line(s) … ⟦tj:34eea91739870eca83d0bfa30385da54⟧ }

static int TableCacheSize(const Options& sanitized_options) {
  // Reserve ten files or so for other uses and give the rest to TableCache.
  return sanitized_options.max_open_files - kNumNonTableCacheFiles;
}

DBImpl::DBImpl(const Options& raw_options, const std::string& dbname)
    : env_(raw_options.env),
      internal_comparator_(raw_options.comparator),
      internal_filter_policy_(raw_options.filter_policy),
      options_(SanitizeOptions(dbname, &internal_comparator_,
                               &internal_filter_policy_, raw_options)),
      owns_info_log_(options_.info_log != raw_options.info_log),
      owns_cache_(options_.block_cache != raw_options.block_cache),
      dbname_(dbname),
      table_cache_(new TableCache(dbname_, options_, TableCacheSize(options_))),
      db_lock_(nullptr),
      shutting_down_(false),
      background_work_finished_signal_(&mutex_),
      mem_(nullptr),
      imm_(nullptr),
      has_imm_(false),
      logfile_(nullptr),
      logfile_number_(0),
      log_(nullptr),
      seed_(0),
      tmp_batch_(new WriteBatch),
      background_compaction_scheduled_(false),
      manual_compaction_(nullptr),
      versions_(new VersionSet(dbname_, &options_, table_cache_,
                               &internal_comparator_)) {}

DBImpl::~DBImpl() {
    { … 27 line(s) … ⟦tj:1b1abf91794758653f9f60ac9975576d⟧ }

Status DBImpl::NewDB() {
    { … 33 line(s) … ⟦tj:a1d1216647f8df9df244ec827476ae92⟧ }

void DBImpl::MaybeIgnoreError(Status* s) const {
  if (s->ok() || options_.paranoid_checks) {
    // No change needed
  } else {
    Log(options_.info_log, "Ignoring error %s", s->ToString().c_str());
    *s = Status::OK();
  }
}

void DBImpl::RemoveObsoleteFiles() {
  mutex_.AssertHeld();

  if (!bg_error_.ok()) {
    // After a background error, we don't know whether a new version may
    { … 9 line(s) … ⟦tj:2de16878a07a7b54f8d67c58aef05575⟧ }
  env_->GetChildren(dbname_, &filenames);  // Ignoring errors on purpose
    { … 51 line(s) … ⟦tj:8d4b0552a6635e54545e34aeae3a582a⟧ }

Status DBImpl::Recover(VersionEdit* edit, bool* save_manifest) {
  mutex_.AssertHeld();

  // Ignore error from CreateDir since the creation of the DB is
    { … 22 line(s) … ⟦tj:9cb36aeb444c6578265a7e28e7b48b3d⟧ }
    if (options_.error_if_exists) {
      return Status::InvalidArgument(dbname_,
                                     "exists (error_if_exists is true)");
    { … 63 line(s) … ⟦tj:f69255872bb7c6533586235b5d315fe9⟧ }

Status DBImpl::RecoverLogFile(uint64_t log_number, bool last_log,
                              bool* save_manifest, VersionEdit* edit,
                              SequenceNumber* max_sequence) {
    { … 7 line(s) … ⟦tj:8181a9c135f1f44629954434ee0f3cd5⟧ }
          (this->status == nullptr ? "(ignoring error) " : ""), fname,
    { … 66 line(s) … ⟦tj:ea06d69bca12d9515e906236f25c4961⟧ }
        // Reflect errors immediately so that conditions like full
    { … 41 line(s) … ⟦tj:57274d32209d2256ddafd348296340ed⟧ }

Status DBImpl::WriteLevel0Table(MemTable* mem, VersionEdit* edit,
                                Version* base) {
    { … 41 line(s) … ⟦tj:a972defd623e1771cd9514c6cac6fa52⟧ }

void DBImpl::CompactMemTable() {
    { … 31 line(s) … ⟦tj:b5237920dea59351aafa8bd5d8b18981⟧ }

void DBImpl::CompactRange(const Slice* begin, const Slice* end) {
    { … 10 line(s) … ⟦tj:372d3ae45b452cb031a62d66be62d8fb⟧ }
  TEST_CompactMemTable();  // TODO(sanjay): Skip if memtable does not overlap
    { … 4 line(s) … ⟦tj:fec247453d97dd130f1d839d71ac4d18⟧ }

void DBImpl::TEST_CompactRange(int level, const Slice* begin,
                               const Slice* end) {
    { … 23 line(s) … ⟦tj:db29c86cd65d928b3fe33330fd17719b⟧ }
         bg_error_.ok()) {
    { … 12 line(s) … ⟦tj:a5d960f59c3ff1be538a669e8f3c5d89⟧ }

Status DBImpl::TEST_CompactMemTable() {
    { … 5 line(s) … ⟦tj:4ef40db49766a99e79f84dbb17e6340b⟧ }
    while (imm_ != nullptr && bg_error_.ok()) {
      background_work_finished_signal_.Wait();
    }
    if (imm_ != nullptr) {
      s = bg_error_;
    { … 4 line(s) … ⟦tj:856d8aae657acae4a3992256a4a11afa⟧ }

void DBImpl::RecordBackgroundError(const Status& s) {
  mutex_.AssertHeld();
  if (bg_error_.ok()) {
    bg_error_ = s;
    background_work_finished_signal_.SignalAll();
  }
}

void DBImpl::MaybeScheduleCompaction() {
    { … 5 line(s) … ⟦tj:0c2798c1ec002727657e72d5402a90ab⟧ }
  } else if (!bg_error_.ok()) {
    // Already got an error; no more changes
    { … 8 line(s) … ⟦tj:0360848e2e7d7292dcc1087b23957474⟧ }

void DBImpl::BGWork(void* db) {
  reinterpret_cast<DBImpl*>(db)->BackgroundCall();
}

void DBImpl::BackgroundCall() {
    { … 4 line(s) … ⟦tj:2e4ad2fc3f753ffa355a214d9d33801a⟧ }
  } else if (!bg_error_.ok()) {
    // No more background work after a background error.
    { … 11 line(s) … ⟦tj:188ae7d9c1ff25c2e98dba63f6066d8b⟧ }

void DBImpl::BackgroundCompaction() {
    { … 60 line(s) … ⟦tj:ed97656ec598137d8e2b3abdc859d378⟧ }
    // Ignore compaction errors found during shutting down
  } else {
    Log(options_.info_log, "Compaction error: %s", status.ToString().c_str());
    { … 16 line(s) … ⟦tj:11cb6672deddbdb8f20bca84118c3bc0⟧ }

void DBImpl::CleanupCompaction(CompactionState* compact) {
    { … 15 line(s) … ⟦tj:be916ac5e263f74bd4ba64eca9cc1e1c⟧ }

Status DBImpl::OpenCompactionOutputFile(CompactionState* compact) {
    { … 23 line(s) … ⟦tj:4b38a547166fb58ade5749f6f1ff65cb⟧ }

Status DBImpl::FinishCompactionOutputFile(CompactionState* compact,
                                          Iterator* input) {
    { … 7 line(s) … ⟦tj:1527907b06eba0ebfef0edb02f54cfd0⟧ }
  // Check for iterator errors
    { … 13 line(s) … ⟦tj:9c020b07ede77941fb75f979aa038b2d⟧ }
  // Finish and check for file errors
    { … 24 line(s) … ⟦tj:1c33fdc49badd5e708a2f2d2b8a996d6⟧ }

Status DBImpl::InstallCompactionResults(CompactionState* compact) {
    { … 16 line(s) … ⟦tj:2681e708232a2c16b4a9349ab3bf51f9⟧ }

Status DBImpl::DoCompactionWork(CompactionState* compact) {
    { … 54 line(s) … ⟦tj:26a7affa0f41030502757c47b0de9a54⟧ }
      // Do not hide error keys
    { … 104 line(s) … ⟦tj:06322b63be38407892e3ee6b8265417d⟧ }

namespace {

struct IterState {
    { … 8 line(s) … ⟦tj:3a98a4c571b01c2dd08fba9bac44cc86⟧ }

static void CleanupIteratorState(void* arg1, void* arg2) {
    { … 8 line(s) … ⟦tj:255fd36bd435ea3e13be4c7265268cfa⟧ }

}  // anonymous namespace

Iterator* DBImpl::NewInternalIterator(const ReadOptions& options,
                                      SequenceNumber* latest_snapshot,
                                      uint32_t* seed) {
    { … 23 line(s) … ⟦tj:6631eee9dab663b3f2d996e42492f377⟧ }

Iterator* DBImpl::TEST_NewInternalIterator() {
    { … 4 line(s) … ⟦tj:f757924f96304fdc58b5f780c8129472⟧ }

int64_t DBImpl::TEST_MaxNextLevelOverlappingBytes() {
  MutexLock l(&mutex_);
  return versions_->MaxNextLevelOverlappingBytes();
}

Status DBImpl::Get(const ReadOptions& options, const Slice& key,
                   std::string* value) {
    { … 44 line(s) … ⟦tj:708c6b40f1ddc2db29e8d97e6da6b492⟧ }

Iterator* DBImpl::NewIterator(const ReadOptions& options) {
    { … 10 line(s) … ⟦tj:dbc07ac701fe2d225e3bda7d186d71a0⟧ }

void DBImpl::RecordReadSample(Slice key) {
    { … 5 line(s) … ⟦tj:52a88d6a02596f4b43768c92f8ec4977⟧ }

const Snapshot* DBImpl::GetSnapshot() {
  MutexLock l(&mutex_);
  return snapshots_.New(versions_->LastSequence());
}

void DBImpl::ReleaseSnapshot(const Snapshot* snapshot) {
  MutexLock l(&mutex_);
  snapshots_.Delete(static_cast<const SnapshotImpl*>(snapshot));
}

// Convenience methods
Status DBImpl::Put(const WriteOptions& o, const Slice& key, const Slice& val) {
  return DB::Put(o, key, val);
}

Status DBImpl::Delete(const WriteOptions& options, const Slice& key) {
  return DB::Delete(options, key);
}

Status DBImpl::Write(const WriteOptions& options, WriteBatch* updates) {
    { … 30 line(s) … ⟦tj:1a3f7f9b6fb26bea42c674921a0db515⟧ }
      bool sync_error = false;
      if (status.ok() && options.sync) {
        status = logfile_->Sync();
        if (!status.ok()) {
          sync_error = true;
    { … 6 line(s) … ⟦tj:a1eb856a631735b0c2d03863d1034171⟧ }
      if (sync_error) {
    { … 29 line(s) … ⟦tj:2269dac1758103ca8bffd66b3d7b2d0a⟧ }

// REQUIRES: Writer list must be non-empty
// REQUIRES: First writer must have a non-null batch
WriteBatch* DBImpl::BuildBatchGroup(Writer** last_writer) {
    { … 46 line(s) … ⟦tj:6f9b52e71ee2a30382f814542f47c985⟧ }

// REQUIRES: mutex_ is held
// REQUIRES: this thread is currently at the front of the writer queue
Status DBImpl::MakeRoomForWrite(bool force) {
    { … 5 line(s) … ⟦tj:97eb5f056904cfbbd151123cf4d18770⟧ }
    if (!bg_error_.ok()) {
      // Yield previous error
      s = bg_error_;
    { … 52 line(s) … ⟦tj:22b1ce3c5cc7c2134d089b1654f977f8⟧ }

bool DBImpl::GetProperty(const Slice& property, std::string* value) {
    { … 60 line(s) … ⟦tj:7b04fb3122151f885fd31ab79006a3ed⟧ }

void DBImpl::GetApproximateSizes(const Range* range, int n, uint64_t* sizes) {
  // TODO(opt): better implementation
    { … 15 line(s) … ⟦tj:8b016d1d4a1516d03e473b10daa70571⟧ }

// Default implementations of convenience methods that subclasses of DB
// can call if they wish
Status DB::Put(const WriteOptions& opt, const Slice& key, const Slice& value) {
    { … 4 line(s) … ⟦tj:3d2e5e549071fff1b463734b8befb62e⟧ }

Status DB::Delete(const WriteOptions& opt, const Slice& key) {
    { … 4 line(s) … ⟦tj:aac33809a1d4a2b2f15b90923731dbeb⟧ }

DB::~DB() = default;

Status DB::Open(const Options& options, const std::string& dbname, DB** dbptr) {
    { … 5 line(s) … ⟦tj:2d30d6f44cebd4318bb72e8af037298f⟧ }
  // Recover handles create_if_missing, error_if_exists
    { … 35 line(s) … ⟦tj:8134200d841802d9eca0e2784f27d6e0⟧ }

Snapshot::~Snapshot() = default;

Status DestroyDB(const std::string& dbname, const Options& options) {
    { … 4 line(s) … ⟦tj:47c763e1a761b4a38fd397832ff5946e⟧ }
    // Ignore error in case directory does not exist
    { … 18 line(s) … ⟦tj:a912c32ae4ccc73490f6813e2e578c3e⟧ }
    env->UnlockFile(lock);  // Ignore error since state is already gone
    env->RemoveFile(lockname);
    env->RemoveDir(dbname);  // Ignore error in case dir contains other files
  }
  return result;
}

}  // namespace leveldb
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (49072 bytes) is available by calling tinyjuice_retrieve with token "52453d1fb693010797cf299b76d204e5" (marker ⟦tj:52453d1fb693010797cf299b76d204e5⟧)]