/*
 * Copyright (C) 2008 Google Inc.
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

package com.google.gson;

import com.google.gson.annotations.JsonAdapter;
import com.google.gson.internal.ConstructorConstructor;
import com.google.gson.internal.Excluder;
import com.google.gson.internal.GsonBuildConfig;
import com.google.gson.internal.LazilyParsedNumber;
import com.google.gson.internal.Primitives;
import com.google.gson.internal.Streams;
import com.google.gson.internal.bind.ArrayTypeAdapter;
import com.google.gson.internal.bind.CollectionTypeAdapterFactory;
import com.google.gson.internal.bind.DefaultDateTypeAdapter;
import com.google.gson.internal.bind.JsonAdapterAnnotationTypeAdapterFactory;
import com.google.gson.internal.bind.JsonTreeReader;
import com.google.gson.internal.bind.JsonTreeWriter;
import com.google.gson.internal.bind.MapTypeAdapterFactory;
import com.google.gson.internal.bind.NumberTypeAdapter;
import com.google.gson.internal.bind.ObjectTypeAdapter;
import com.google.gson.internal.bind.ReflectiveTypeAdapterFactory;
import com.google.gson.internal.bind.SerializationDelegatingTypeAdapter;
import com.google.gson.internal.bind.TypeAdapters;
import com.google.gson.internal.sql.SqlTypesSupport;
import com.google.gson.reflect.TypeToken;
import com.google.gson.stream.JsonReader;
import com.google.gson.stream.JsonToken;
import com.google.gson.stream.JsonWriter;
import com.google.gson.stream.MalformedJsonException;
import java.io.EOFException;
import java.io.IOException;
import java.io.Reader;
import java.io.StringReader;
import java.io.StringWriter;
import java.io.Writer;
import java.lang.reflect.Type;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.text.DateFormat;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.atomic.AtomicLongArray;

/**
 * This is the main class for using Gson. Gson is typically used by first constructing a Gson
 * instance and then invoking {@link #toJson(Object)} or {@link #fromJson(String, Class)} methods on
 * it. Gson instances are Thread-safe so you can reuse them freely across multiple threads.
 *
 * <p>You can create a Gson instance by invoking {@code new Gson()} if the default configuration is
 * all you need. You can also use {@link GsonBuilder} to build a Gson instance with various
 * configuration options such as versioning support, pretty printing, custom newline, custom indent,
 * custom {@link JsonSerializer}s, {@link JsonDeserializer}s, and {@link InstanceCreator}s.
 *
 * <p>Here is an example of how Gson is used for a simple Class:
 *
 * <pre>
 * Gson gson = new Gson(); // Or use new GsonBuilder().create();
 * MyType target = new MyType();
 * String json = gson.toJson(target); // serializes target to JSON
 * MyType target2 = gson.fromJson(json, MyType.class); // deserializes json into target2
 * </pre>
 *
 * <p>If the type of the object that you are converting is a {@code ParameterizedType} (i.e. has at
 * least one type argument, for example {@code List<MyType>}) then for deserialization you must use
 * a {@code fromJson} method with {@link Type} or {@link TypeToken} parameter to specify the
 * parameterized type. For serialization specifying a {@code Type} or {@code TypeToken} is optional,
 * otherwise Gson will use the runtime type of the object. {@link TypeToken} is a class provided by
 * Gson which helps creating parameterized types. Here is an example showing how this can be done:
 *
 * <pre>
 * TypeToken&lt;List&lt;MyType&gt;&gt; listType = new TypeToken&lt;List&lt;MyType&gt;&gt;() {};
 * List&lt;MyType&gt; target = new LinkedList&lt;MyType&gt;();
 * target.add(new MyType(1, "abc"));
 *
 * Gson gson = new Gson();
 * // For serialization you normally do not have to specify the type, Gson will use
 * // the runtime type of the objects, however you can also specify it explicitly
 * String json = gson.toJson(target, listType.getType());
 *
 * // But for deserialization you have to specify the type
 * List&lt;MyType&gt; target2 = gson.fromJson(json, listType);
 * </pre>
 *
 * <p>See the <a href="https://github.com/google/gson/blob/main/UserGuide.md">Gson User Guide</a>
 * for a more complete set of examples.
 *
 * <h2 id="default-lenient">JSON Strictness handling</h2>
 *
 * For legacy reasons most of the {@code Gson} methods allow JSON data which does not comply with
 * the JSON specification when no explicit {@linkplain Strictness strictness} is set (the default).
 * To specify the strictness of a {@code Gson} instance, you should set it through {@link
 * GsonBuilder#setStrictness(Strictness)}.
 *
 * <p>For older Gson versions, which don't have the strictness mode API, the following workarounds
 * can be used:
 *
 * <h3>Serialization</h3>
 *
 * <ol>
 *   <li>Use {@link #getAdapter(Class)} to obtain the adapter for the type to be serialized
 *   <li>When using an existing {@code JsonWriter}, manually apply the writer settings of this
 *       {@code Gson} instance listed by {@link #newJsonWriter(Writer)}.<br>
 *       Otherwise, when not using an existing {@code JsonWriter}, use {@link
 *       #newJsonWriter(Writer)} to construct one.
 *   <li>Call {@link TypeAdapter#write(JsonWriter, Object)}
 * </ol>
 *
 * <h3>Deserialization</h3>
 *
 * <ol>
 *   <li>Use {@link #getAdapter(Class)} to obtain the adapter for the type to be deserialized
 *   <li>When using an existing {@code JsonReader}, manually apply the reader settings of this
 *       {@code Gson} instance listed by {@link #newJsonReader(Reader)}.<br>
 *       Otherwise, when not using an existing {@code JsonReader}, use {@link
 *       #newJsonReader(Reader)} to construct one.
 *   <li>Call {@link TypeAdapter#read(JsonReader)}
 *   <li>Call {@link JsonReader#peek()} and verify that the result is {@link JsonToken#END_DOCUMENT}
 *       to make sure there is no trailing data
 * </ol>
 *
 * Note that the {@code JsonReader} created this way is only 'legacy strict', it mostly adheres to
 * the JSON specification but allows small deviations. See {@link
 * JsonReader#setStrictness(Strictness)} for details.
 *
 * @see TypeToken
 * @author Inderjeet Singh
 * @author Joel Leitch
 * @author Jesse Wilson
 */
public final class Gson {
    { … 1222 line(s) … ⟦tj:2c2e6ce21f76ec66b2d89c9ebe5d852c⟧ }
      // TODO(inder): Figure out whether it is indeed right to rethrow this as JsonSyntaxException
    { … 161 line(s) … ⟦tj:7570e777b2984c185f125ad66ad5b535⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (68459 bytes) is available by calling tinyjuice_retrieve with token "85926a85e76809458c47cfd998fbcd1c" (marker ⟦tj:85926a85e76809458c47cfd998fbcd1c⟧)]