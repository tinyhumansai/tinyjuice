"""
requests.sessions
~~~~~~~~~~~~~~~~~

This module provides a Session object to manage and persist settings across
requests (cookies, auth, proxies).
"""
import os
import sys
import time
from collections import OrderedDict
from datetime import timedelta

from ._internal_utils import to_native_string
from .adapters import HTTPAdapter
from .auth import _basic_auth_str
from .compat import Mapping, cookielib, urljoin, urlparse
from .cookies import (
    RequestsCookieJar,
    cookiejar_from_dict,
    extract_cookies_to_jar,
    merge_cookies,
)
from .exceptions import (
    ChunkedEncodingError,
    ContentDecodingError,
    InvalidSchema,
    TooManyRedirects,
)
from .hooks import default_hooks, dispatch_hook

# formerly defined here, reexposed here for backward compatibility
from .models import (  # noqa: F401
    DEFAULT_REDIRECT_LIMIT,
    REDIRECT_STATI,
    PreparedRequest,
    Request,
)
from .status_codes import codes
from .structures import CaseInsensitiveDict
from .utils import (  # noqa: F401
    DEFAULT_PORTS,
    default_headers,
    get_auth_from_url,
    get_environ_proxies,
    get_netrc_auth,
    requote_uri,
    resolve_proxies,
    rewind_body,
    should_bypass_proxies,
    to_key_val_list,
)

# Preferred clock, based on which one is more accurate on a given system.
if sys.platform == "win32":
    preferred_clock = time.perf_counter
else:
    preferred_clock = time.time


def merge_setting(request_setting, session_setting, dict_class=OrderedDict):
    ...  # 27 line(s) collapsed ⟦tj:077cde73280053b1704e089a0e9ed7be⟧


def merge_hooks(request_hooks, session_hooks, dict_class=OrderedDict):
    ...  # 12 line(s) collapsed ⟦tj:75f64ea8dfcfa010949254bf9caf5d94⟧


class SessionRedirectMixin:
    def get_redirect_target(self, resp):
        ...  # 18 line(s) collapsed ⟦tj:171e3999e1bb483ef1c5e024c0740ae3⟧

    def should_strip_auth(self, old_url, new_url):
        ...  # 30 line(s) collapsed ⟦tj:3cb2ed79205a5da81fb84ed9613afccc⟧

    def resolve_redirects(
        self,
        resp,
        req,
        stream=False,
        timeout=None,
        verify=True,
        cert=None,
        proxies=None,
        yield_requests=False,
        **adapter_kwargs,
    ):
        ...  # 110 line(s) collapsed ⟦tj:b9402e6225d5ce6b36c792a90e749134⟧

    def rebuild_auth(self, prepared_request, response):
        ...  # 18 line(s) collapsed ⟦tj:b26c57811ccb4aba6547cd59a7bf79ac⟧

    def rebuild_proxies(self, prepared_request, proxies):
        ...  # 29 line(s) collapsed ⟦tj:9dd265d13fb9b594e16e7328053e4a46⟧

    def rebuild_method(self, prepared_request, response):
        ...  # 20 line(s) collapsed ⟦tj:43394ef7b9ad29f024a843b3ce8efc49⟧


class Session(SessionRedirectMixin):
    """A Requests session.

    Provides cookie persistence, connection-pooling, and configuration.

    Basic Usage::

      >>> import requests
      >>> s = requests.Session()
      >>> s.get('https://httpbin.org/get')
      <Response [200]>

    Or as a context manager::

      >>> with requests.Session() as s:
      ...     s.get('https://httpbin.org/get')
      <Response [200]>
    """

    __attrs__ = [
        "headers",
        "cookies",
        "auth",
        "proxies",
        "hooks",
        "params",
        "verify",
        "cert",
        "adapters",
        "stream",
        "trust_env",
        "max_redirects",
    ]

    def __init__(self):
        #: A case-insensitive dictionary of headers to be sent on each
        #: :class:`Request <Request>` sent from this
        #: :class:`Session <Session>`.
        ...  # 56 line(s) collapsed ⟦tj:21af54fde059abf1b9f9dc4187de1a58⟧

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.close()

    def prepare_request(self, request):
        ...  # 41 line(s) collapsed ⟦tj:42b08a709fd99701fa8b1005a8498241⟧

    def request(
        self,
        method,
        url,
        params=None,
        data=None,
        headers=None,
        cookies=None,
        files=None,
        auth=None,
        timeout=None,
        allow_redirects=True,
        proxies=None,
        hooks=None,
        stream=None,
        verify=None,
        cert=None,
        json=None,
    ):
        ...  # 73 line(s) collapsed ⟦tj:ac4d895638fd05b557e26c6af9447447⟧

    def get(self, url, **kwargs):
        ...  # 9 line(s) collapsed ⟦tj:35a284a1d859efd30ab89ccfabe4fde5⟧

    def options(self, url, **kwargs):
        ...  # 9 line(s) collapsed ⟦tj:c6d5a653ac49a7e43374f8a465cc292e⟧

    def head(self, url, **kwargs):
        ...  # 9 line(s) collapsed ⟦tj:00b1fcd456f202ad04bd384586f0db5e⟧

    def post(self, url, data=None, json=None, **kwargs):
        ...  # 11 line(s) collapsed ⟦tj:eb6f65113720a2a096e9bf3f6860b3e5⟧

    def put(self, url, data=None, **kwargs):
        ...  # 10 line(s) collapsed ⟦tj:0f3058e56e5d9c92d80c6383b2aa7537⟧

    def patch(self, url, data=None, **kwargs):
        ...  # 10 line(s) collapsed ⟦tj:ac1a622e707b4b5044615919578a8542⟧

    def delete(self, url, **kwargs):
        ...  # 8 line(s) collapsed ⟦tj:78d77344449dcc0e101e9e35b84cf497⟧

    def send(self, request, **kwargs):
        ...  # 75 line(s) collapsed ⟦tj:d062f5520ac770dc9e78b00193dc55af⟧

    def merge_environment_settings(self, url, proxies, stream, verify, cert):
        ...  # 29 line(s) collapsed ⟦tj:7fc3edefc61e95e96f948aca2278a36e⟧

    def get_adapter(self, url):
        ...  # 11 line(s) collapsed ⟦tj:57d464a7af4bc58426e9989d3a9b308c⟧

    def close(self):
        """Closes all adapters and as such the session"""
        for v in self.adapters.values():
            v.close()

    def mount(self, prefix, adapter):
        ...  # 9 line(s) collapsed ⟦tj:16f26fd167d46459b3406509e7039be4⟧

    def __getstate__(self):
        state = {attr: getattr(self, attr, None) for attr in self.__attrs__}
        return state

    def __setstate__(self, state):
        for attr, value in state.items():
            setattr(self, attr, value)


def session():
    ...  # 12 line(s) collapsed ⟦tj:cc0a4f0e52c49614c51ca0227d63022d⟧
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (30495 bytes) is available by calling tinyjuice_retrieve with token "ca44c8f145864a5b4e7c7d3b1caa2594" (marker ⟦tj:ca44c8f145864a5b4e7c7d3b1caa2594⟧)]