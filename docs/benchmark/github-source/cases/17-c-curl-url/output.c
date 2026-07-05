/***************************************************************************
 *                                  _   _ ____  _
 *  Project                     ___| | | |  _ \| |
 *                             / __| | | | |_) | |
 *                            | (__| |_| |  _ <| |___
 *                             \___|\___/|_| \_\_____|
 *
 * Copyright (C) Daniel Stenberg, <daniel@haxx.se>, et al.
 *
 * This software is licensed as described in the file COPYING, which
 * you should have received as part of this distribution. The terms
 * are also available at https://curl.se/docs/copyright.html.
 *
 * You may opt to use, copy, modify, merge, publish, distribute and/or sell
 * copies of the Software, and permit persons to whom the Software is
 * furnished to do so, under the terms of the COPYING file.
 *
 * This software is distributed on an "AS IS" basis, WITHOUT WARRANTY OF ANY
 * KIND, either express or implied.
 *
 * SPDX-License-Identifier: curl
 *
 ***************************************************************************/

#include "curl_setup.h"

#ifdef HAVE_NETINET_IN_H
#include <netinet/in.h>
#endif
#ifdef HAVE_NETDB_H
#include <netdb.h>
#endif
#ifdef HAVE_ARPA_INET_H
#include <arpa/inet.h>
#endif
#ifdef HAVE_NET_IF_H
#include <net/if.h>
#endif
#ifdef HAVE_IPHLPAPI_H
#include <Iphlpapi.h>
#endif
#ifdef HAVE_SYS_IOCTL_H
#include <sys/ioctl.h>
#endif
#ifdef HAVE_SYS_PARAM_H
#include <sys/param.h>
#endif

#ifdef __VMS
#include <in.h>
#include <inet.h>
#endif

#ifdef HAVE_SYS_UN_H
#include <sys/un.h>
#endif

#ifndef HAVE_SOCKET
#error "We can't compile without socket() support!"
#endif

#include <limits.h>

#include "doh.h"
#include "urldata.h"
#include "netrc.h"
#include "formdata.h"
#include "mime.h"
#include "vtls/vtls.h"
#include "hostip.h"
#include "transfer.h"
#include "sendf.h"
#include "progress.h"
#include "cookie.h"
#include "strcase.h"
#include "strerror.h"
#include "escape.h"
#include "strtok.h"
#include "share.h"
#include "content_encoding.h"
#include "http_digest.h"
#include "http_negotiate.h"
#include "select.h"
#include "multiif.h"
#include "easyif.h"
#include "speedcheck.h"
#include "warnless.h"
#include "getinfo.h"
#include "urlapi-int.h"
#include "system_win32.h"
#include "hsts.h"
#include "noproxy.h"
#include "cfilters.h"
#include "idn.h"

/* And now for the protocols */
#include "ftp.h"
#include "dict.h"
#include "telnet.h"
#include "tftp.h"
#include "http.h"
#include "http2.h"
#include "file.h"
#include "curl_ldap.h"
#include "vssh/ssh.h"
#include "imap.h"
#include "url.h"
#include "connect.h"
#include "inet_ntop.h"
#include "http_ntlm.h"
#include "curl_rtmp.h"
#include "gopher.h"
#include "mqtt.h"
#include "http_proxy.h"
#include "conncache.h"
#include "multihandle.h"
#include "strdup.h"
#include "setopt.h"
#include "altsvc.h"
#include "dynbuf.h"
#include "headers.h"

/* The last 3 #include files should be in this order */
#include "curl_printf.h"
#include "curl_memory.h"
#include "memdebug.h"

#ifndef ARRAYSIZE
#define ARRAYSIZE(A) (sizeof(A)/sizeof((A)[0]))
#endif

#ifdef USE_NGHTTP2
static void data_priority_cleanup(struct Curl_easy *data);
#else
#define data_priority_cleanup(x)
#endif

/* Some parts of the code (e.g. chunked encoding) assume this buffer has at
 * more than just a few bytes to play with. Don't let it become too small or
 * bad things will happen.
 */
#if READBUFFER_SIZE < READBUFFER_MIN
# error READBUFFER_SIZE is too small
#endif

#ifdef USE_UNIX_SOCKETS
#define UNIX_SOCKET_PREFIX "localhost"
#endif

/* Reject URLs exceeding this length */
#define MAX_URL_LEN 0xffff

/*
* get_protocol_family()
*
* This is used to return the protocol family for a given protocol.
*
* Parameters:
*
* 'h'  [in]  - struct Curl_handler pointer.
*
* Returns the family as a single bit protocol identifier.
*/
static curl_prot_t get_protocol_family(const struct Curl_handler *h)
{
    { … 4 line(s) … ⟦tj:4c8a3ca42d1bd4bd5ba3cc3bbd7f95ba⟧ }

void Curl_freeset(struct Curl_easy *data)
{
    { … 30 line(s) … ⟦tj:cbd4bfd45ef353de0d95aa55c0fcfa2a⟧ }

/* free the URL pieces */
static void up_free(struct Curl_easy *data)
{
    { … 12 line(s) … ⟦tj:6b3ebdb4fbaf851ef3d32406341df4e4⟧ }

/*
 * This is the internal function curl_easy_cleanup() calls. This should
 * cleanup and free all resources associated with this sessionhandle.
 *
 * We ignore SIGPIPE when this is called from curl_easy_cleanup.
 */

CURLcode Curl_close(struct Curl_easy **datap)
{
    { … 116 line(s) … ⟦tj:51a2f9e711b023f7d7391d632fd33672⟧ }

/*
 * Initialize the UserDefined fields within a Curl_easy.
 * This may be safely called on a new or existing Curl_easy.
 */
CURLcode Curl_init_userdefined(struct Curl_easy *data)
{
    { … 143 line(s) … ⟦tj:00270ef4191b30dbbda5bf0e3d86bbc3⟧ }

/**
 * Curl_open()
 *
 * @param curl is a pointer to a sessionhandle pointer that gets set by this
 * function.
 * @return CURLcode
 */

CURLcode Curl_open(struct Curl_easy **curl)
{
    { … 6 line(s) … ⟦tj:1f31e8276f909076713217ce3462f8bb⟧ }
    /* this is a very serious error */
    { … 44 line(s) … ⟦tj:63e0605c09efe1c8fb9f92022c39480e⟧ }

static void conn_shutdown(struct Curl_easy *data)
{
    { … 9 line(s) … ⟦tj:0e61fdefa95a5f80e7f5bd0644cc5873⟧ }

static void conn_free(struct Curl_easy *data, struct connectdata *conn)
{
    { … 38 line(s) … ⟦tj:42de9f57c0e34054b2f6cc4a61e8ec8f⟧ }

/*
 * Disconnects the given connection. Note the connection may not be the
 * primary connection, like when freeing room in the connection cache or
 * killing of a dead old connection.
 *
 * A connection needs an easy handle when closing down. We support this passed
 * in separately since the connection to get closed here is often already
 * disassociated from an easy handle.
 *
 * This function MUST NOT reset state in the Curl_easy struct if that
 * isn't strictly bound to the life-time of *this* particular connection.
 *
 */

void Curl_disconnect(struct Curl_easy *data,
                     struct connectdata *conn, bool dead_connection)
{
    { … 54 line(s) … ⟦tj:683944397d02ed617f0b4d1858cea604⟧ }

/*
 * IsMultiplexingPossible()
 *
 * Return a bitmask with the available multiplexing options for the given
 * requested connection.
 */
static int IsMultiplexingPossible(const struct Curl_easy *handle,
                                  const struct connectdata *conn)
{
    { … 13 line(s) … ⟦tj:8c38a51a903eec09f048d3e494007ca1⟧ }

#ifndef CURL_DISABLE_PROXY
static bool
proxy_info_matches(const struct proxy_info *data,
                   const struct proxy_info *needle)
{
    { … 7 line(s) … ⟦tj:b13088cad948d144f1c6ab410e611408⟧ }

static bool
socks_proxy_info_matches(const struct proxy_info *data,
                         const struct proxy_info *needle)
{
    { … 14 line(s) … ⟦tj:596405f3cb4fde4e889c5b89ca2e1478⟧ }
#else
/* disabled, won't get called */
#define proxy_info_matches(x,y) FALSE
#define socks_proxy_info_matches(x,y) FALSE
#endif

/* A connection has to have been idle for a shorter time than 'maxage_conn'
   (the success rate is just too low after this), or created less than
   'maxlifetime_conn' ago, to be subject for reuse. */

static bool conn_maxage(struct Curl_easy *data,
                        struct connectdata *conn,
                        struct curltime now)
{
    { … 24 line(s) … ⟦tj:63250e6d4a5d6d9c7b0d6b1b2c1362ef⟧ }

/*
 * This function checks if the given connection is dead and prunes it from
 * the connection cache if so.
 *
 * When this is called as a Curl_conncache_foreach() callback, the connection
 * cache lock is held!
 *
 * Returns TRUE if the connection was dead and pruned.
 */
static bool prune_if_dead(struct connectdata *conn,
                          struct Curl_easy *data)
{
    { … 53 line(s) … ⟦tj:41ea444a2b2d9173474510832d1cfa37⟧ }

/*
 * Wrapper to use prune_if_dead() function in Curl_conncache_foreach()
 *
 */
static int call_prune_if_dead(struct Curl_easy *data,
                              struct connectdata *conn, void *param)
{
    { … 8 line(s) … ⟦tj:98d5b48dfabb15ae27c67b6f402bfe23⟧ }

/*
 * This function scans the connection cache for half-open/dead connections,
 * closes and removes them. The cleanup is done at most once per second.
 *
 * When called, this transfer has no connection attached.
 */
static void prune_dead_connections(struct Curl_easy *data)
{
    { … 25 line(s) … ⟦tj:b5e6a3981da94da486cef3eb4cd55ca7⟧ }

#ifdef USE_SSH
static bool ssh_config_matches(struct connectdata *one,
                               struct connectdata *two)
{
  return (Curl_safecmp(one->proto.sshc.rsa, two->proto.sshc.rsa) &&
          Curl_safecmp(one->proto.sshc.rsa_pub, two->proto.sshc.rsa_pub));
}
#else
#define ssh_config_matches(x,y) FALSE
#endif

/*
 * Given one filled in connection struct (named needle), this function should
 * detect if there already is one that has all the significant details
 * exactly the same and thus should be used instead.
 *
 * If there is a match, this function returns TRUE - and has marked the
 * connection as 'in-use'. It must later be called with ConnectionDone() to
 * return back to 'idle' (unused) state.
 *
 * The force_reuse flag is set if the connection must be used.
 */
static bool
ConnectionExists(struct Curl_easy *data,
                 struct connectdata *needle,
                 struct connectdata **usethis,
                 bool *force_reuse,
                 bool *waitpipe)
{
    { … 402 line(s) … ⟦tj:9f51634611668049a47fe7239c3ce32c⟧ }

/*
 * verboseconnect() displays verbose information after a connect
 */
#ifndef CURL_DISABLE_VERBOSE_STRINGS
void Curl_verboseconnect(struct Curl_easy *data,
                         struct connectdata *conn, int sockindex)
{
    { … 8 line(s) … ⟦tj:a9c747afbe20ac3ce87268ceaf2aa0a8⟧ }
#endif

/*
 * Allocate and initialize a new connectdata object.
 */
static struct connectdata *allocate_conn(struct Curl_easy *data)
{
    { … 70 line(s) … ⟦tj:b4bff9604a59d92395e15893b99bf34e⟧ }
      goto error;
    { … 14 line(s) … ⟦tj:13f3f7502c2dac46008ab9feaf0c96dc⟧ }
error:
    { … 5 line(s) … ⟦tj:37550c102aaf3c0ab9a966408e9dede8⟧ }

const struct Curl_handler *Curl_get_scheme_handler(const char *scheme)
{
  return Curl_getn_scheme_handler(scheme, strlen(scheme));
}

/* returns the handler if the given scheme is built-in */
const struct Curl_handler *Curl_getn_scheme_handler(const char *scheme,
                                                    size_t len)
{
    { … 210 line(s) … ⟦tj:906eae7b753269cc53a01a77fcc2e88e⟧ }

static CURLcode findprotocol(struct Curl_easy *data,
                             struct connectdata *conn,
                             const char *protostr)
{
    { … 28 line(s) … ⟦tj:c240c3233453082115476ee614a91a31⟧ }


CURLcode Curl_uc_to_curlcode(CURLUcode uc)
{
    { … 11 line(s) … ⟦tj:edbb7f665574cf6aaf22261ed7d3a896⟧ }

#ifdef USE_IPV6
/*
 * If the URL was set with an IPv6 numerical address with a zone id part, set
 * the scope_id based on that!
 */

static void zonefrom_url(CURLU *uh, struct Curl_easy *data,
                         struct connectdata *conn)
{
    { … 30 line(s) … ⟦tj:cf3ba6cc15ef7e3593a21f0bea1d5e19⟧ }
              Curl_strerror(errno, buffer, sizeof(buffer)));
    { … 60 line(s) … ⟦tj:4a354384634f9282f403ef59fc762c7c⟧ }
      failf(data, "URL rejected: %s", curl_url_strerror(uc));
    { … 359 line(s) … ⟦tj:b60a30c656f4475c79e0f341ada437d1⟧ }
    goto error;
    { … 11 line(s) … ⟦tj:67ee15e536d0b23bc5a1805851db2810⟧ }
      goto error;
    { … 23 line(s) … ⟦tj:ef2e04c6d9ae2e1f074d3dcbc0925e90⟧ }
      goto error;
    { … 4 line(s) … ⟦tj:406dae90a99278004efe2cbfe213586f⟧ }
          curl_url_strerror(uc));
    result = CURLE_COULDNT_RESOLVE_PROXY;
    goto error;
    { … 9 line(s) … ⟦tj:bfa2ce5b9c42d91f82812c4cd144bf4e⟧ }
      goto error;
    { … 14 line(s) … ⟦tj:107a4135ab188323dfff506ea6a01a67⟧ }
    goto error;
  uc = curl_url_get(uhp, CURLUPART_PASSWORD, &proxypasswd, CURLU_URLDECODE);
  if(uc && (uc != CURLUE_NO_PASSWORD))
    goto error;
    { … 7 line(s) … ⟦tj:c3bd5157258290b650a56cc48565a668⟧ }
      goto error;
    { … 5 line(s) … ⟦tj:90822c9e393ab82ad988fb30db6b9779⟧ }
        goto error;
    { … 6 line(s) … ⟦tj:971feb1957b9b5b03eb9b3e9b916a102⟧ }
      goto error;
    { … 32 line(s) … ⟦tj:dfaaac0f8638847ca157daf398e80f7e⟧ }
    goto error;
    { … 6 line(s) … ⟦tj:9e0335838d70364fe0d2e730a906f72d⟧ }
      goto error;
    { … 8 line(s) … ⟦tj:3c23987fd499e6cb90ad9589eff33350⟧ }
        goto error;
    { … 25 line(s) … ⟦tj:6dd5ef99045a93be307fda4d3d6e207c⟧ }
error:
    { … 278 line(s) … ⟦tj:61500289497c1a6cafa24601e49a9025⟧ }
    goto error;
    { … 5 line(s) … ⟦tj:69ec353f065f915f416ea559eb6953d5⟧ }
      goto error;
    { … 8 line(s) … ⟦tj:d7430ca101438eaf1e1d366123d7c328⟧ }
        goto error;
    { … 6 line(s) … ⟦tj:bca6fa864b27bead9e8ef345991c695b⟧ }
error:
    { … 76 line(s) … ⟦tj:bf16f59189bab6a9db50d108eb417ca0⟧ }
      failf(data, ".netrc parser error");
    { … 163 line(s) … ⟦tj:50334f2aa9f1238a7323d80a0b637b3c⟧ }
    goto error;
    { … 15 line(s) … ⟦tj:2265d793d09e5b6d11f25ce990058b85⟧ }
        goto error;
    { … 11 line(s) … ⟦tj:2858631e1df1e5fa8c6a96d1600bfcde⟧ }
    goto error;
    { … 4 line(s) … ⟦tj:bae53d761e9fd8fc3f0322b12964bd93⟧ }
error:
    { … 408 line(s) … ⟦tj:2320e75c008a894985ec92b735600cfd⟧ }
   * TODO: is this correct in the case of TLS connections that have
    { … 655 line(s) … ⟦tj:fb62a362ff2e7c74d6b4680a61547cdd⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (120525 bytes) is available by calling tinyjuice_retrieve with token "c8d3f33a41d4b3ee01d6a355d08e45d1" (marker ⟦tj:c8d3f33a41d4b3ee01d6a355d08e45d1⟧)]