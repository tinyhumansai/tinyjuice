/*
 * Copyright (C) 2007 The Guava Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.google.common.collect;

import static com.google.common.base.Preconditions.checkNotNull;
import static com.google.common.collect.CollectPreconditions.checkNonnegative;

import com.google.common.annotations.GwtCompatible;
import com.google.common.annotations.J2ktIncompatible;
import com.google.common.annotations.VisibleForTesting;
import com.google.common.base.Function;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.Map.Entry;
import java.util.NoSuchElementException;
import java.util.SortedMap;
import java.util.SortedSet;
import java.util.TreeSet;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicInteger;
import javax.annotation.CheckForNull;
import org.checkerframework.checker.nullness.qual.NonNull;
import org.checkerframework.checker.nullness.qual.Nullable;

/**
 * A comparator, with additional methods to support common operations. This is an "enriched" version
 * of {@code Comparator} for pre-Java-8 users, in the same sense that {@link FluentIterable} is an
 * enriched {@link Iterable} for pre-Java-8 users.
 *
 * <h3>Three types of methods</h3>
 *
 * Like other fluent types, there are three types of methods present: methods for <i>acquiring</i>,
 * <i>chaining</i>, and <i>using</i>.
 *
 * <h4>Acquiring</h4>
 *
 * <p>The common ways to get an instance of {@code Ordering} are:
 *
 * <ul>
 *   <li>Subclass it and implement {@link #compare} instead of implementing {@link Comparator}
    { … 103 line(s) … ⟦tj:921ef22f4565e2fc45deb54b5fd0f821⟧ }
  // TODO(kevinb): right way to explain this??
    { … 59 line(s) … ⟦tj:473136e81df0660490f92ff99371ccf0⟧ }
  // TODO(kevinb): provide replacement
    { … 25 line(s) … ⟦tj:fd4cc04a734ef122baac5b9b6e8981c3⟧ }
  // TODO(kevinb): provide replacement
    { … 68 line(s) … ⟦tj:a5691107441f408b78d93c6c9782d0a0⟧ }
  // TODO(kevinb): copy to Comparators, etc.
    { … 430 line(s) … ⟦tj:7d45dff970a07302daf4b75b9234e411⟧ }
  @SuppressWarnings("nullness") // TODO: b/316358623 - Remove after checker fix.
    { … 75 line(s) … ⟦tj:2b6f56f8a732db88b0ff63fb3eefb820⟧ }
    // TODO(kevinb): see if delegation is hurting performance noticeably
    // TODO(kevinb): if we change this implementation, add full unit tests.
    { … 38 line(s) … ⟦tj:3d835f23906e600fc40d3f20e079d8ee⟧ }
  // TODO(kevinb): rerun benchmarks including new options
  @SuppressWarnings("nullness") // TODO: b/316358623 - Remove after checker fix.
    { … 22 line(s) … ⟦tj:96e9f5cae0d25fa02b9483cca292e237⟧ }
  // TODO(kevinb): rerun benchmarks including new options
    { … 87 line(s) … ⟦tj:5d79e228ed537e83086ebff4fc0630e9⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (40352 bytes) is available by calling tinyjuice_retrieve with token "1b3ee3fd819be6c455f9cae2ef8b6f87" (marker ⟦tj:1b3ee3fd819be6c455f9cae2ef8b6f87⟧)]