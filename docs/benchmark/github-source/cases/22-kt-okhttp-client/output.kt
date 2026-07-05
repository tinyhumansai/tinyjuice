/*
 * Copyright (C) 2012 Square, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package okhttp3

import java.net.Proxy
import java.net.ProxySelector
import java.net.Socket
import java.time.Duration
import java.util.Collections
import java.util.Random
import java.util.concurrent.ExecutorService
import java.util.concurrent.TimeUnit
import java.util.concurrent.TimeUnit.MILLISECONDS
import javax.net.SocketFactory
import javax.net.ssl.HostnameVerifier
import javax.net.ssl.SSLSocketFactory
import javax.net.ssl.X509TrustManager
import okhttp3.Protocol.HTTP_1_1
import okhttp3.Protocol.HTTP_2
import okhttp3.internal.asFactory
import okhttp3.internal.checkDuration
import okhttp3.internal.concurrent.TaskRunner
import okhttp3.internal.connection.RealCall
import okhttp3.internal.connection.RouteDatabase
import okhttp3.internal.immutableListOf
import okhttp3.internal.platform.Platform
import okhttp3.internal.proxy.NullProxySelector
import okhttp3.internal.tls.CertificateChainCleaner
import okhttp3.internal.tls.OkHostnameVerifier
import okhttp3.internal.toImmutableList
import okhttp3.internal.ws.RealWebSocket
import okio.Sink
import okio.Source
import org.codehaus.mojo.animal_sniffer.IgnoreJRERequirement

/**
 * Factory for [calls][Call], which can be used to send HTTP requests and read their responses.
 *
 * ## OkHttpClients Should Be Shared
 *
 * OkHttp performs best when you create a single `OkHttpClient` instance and reuse it for all of
 * your HTTP calls. This is because each client holds its own connection pool and thread pools.
 * Reusing connections and threads reduces latency and saves memory. Conversely, creating a client
 * for each request wastes resources on idle pools.
 *
 * Use `new OkHttpClient()` to create a shared instance with the default settings:
 *
 * ```
 * // The singleton HTTP client.
 * public final OkHttpClient client = new OkHttpClient();
 * ```
 *
 * Or use `new OkHttpClient.Builder()` to create a shared instance with custom settings:
 *
 * ```
 * // The singleton HTTP client.
 * public final OkHttpClient client = new OkHttpClient.Builder()
 *     .addInterceptor(new HttpLoggingInterceptor())
 *     .cache(new Cache(cacheDir, cacheSize))
 *     .build();
 * ```
 *
 * ## Customize Your Client With newBuilder()
 *
 * You can customize a shared OkHttpClient instance with [newBuilder]. This builds a client that
 * shares the same connection pool, thread pools, and configuration. Use the builder methods to
 * configure the derived client for a specific purpose.
 *
 * This example shows a call with a short 500 millisecond timeout:
 *
 * ```
 * OkHttpClient eagerClient = client.newBuilder()
 *     .readTimeout(500, TimeUnit.MILLISECONDS)
 *     .build();
 * Response response = eagerClient.newCall(request).execute();
 * ```
 *
 * ## Shutdown Isn't Necessary
 *
 * The threads and connections that are held will be released automatically if they remain idle. But
 * if you are writing a application that needs to aggressively release unused resources you may do
 * so.
 *
 * Shutdown the dispatcher's executor service with [shutdown()][ExecutorService.shutdown]. This will
 * also cause future calls to the client to be rejected.
 *
 * ```
 * client.dispatcher().executorService().shutdown();
 * ```
 *
 * Clear the connection pool with [evictAll()][ConnectionPool.evictAll]. Note that the connection
 * pool's daemon thread may not exit immediately.
 *
 * ```
 * client.connectionPool().evictAll();
 * ```
 *
 * If your client has a cache, call [close()][Cache.close]. Note that it is an error to create calls
 * against a cache that is closed, and doing so will cause the call to crash.
 *
 * ```
 * client.cache().close();
 * ```
 *
 * OkHttp also uses daemon threads for HTTP/2 connections. These will exit automatically if they
 * remain idle.
 */
open class OkHttpClient internal constructor(
  builder: Builder
) : Cloneable, Call.Factory, WebSocket.Factory {
    { … 15 line(s) … ⟦tj:71d79bb7ece09934be6c3b91a0a51451⟧ }
   * These interceptors must call [Interceptor.Chain.proceed] exactly once: it is an error for
    { … 427 line(s) … ⟦tj:7cc6c9836694a09e9e2bc0634715c213⟧ }
     * These interceptors must call [Interceptor.Chain.proceed] exactly once: it is an error for a
    { … 511 line(s) … ⟦tj:78e5b8e200d4b95670ba6e1e34861671⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (44011 bytes) is available by calling tinyjuice_retrieve with token "7c57469526804a4708f841ee010533f8" (marker ⟦tj:7c57469526804a4708f841ee010533f8⟧)]