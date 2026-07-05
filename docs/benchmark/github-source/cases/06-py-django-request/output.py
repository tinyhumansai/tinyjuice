import codecs
import copy
from io import BytesIO
from itertools import chain
from urllib.parse import parse_qsl, quote, urlencode, urljoin, urlsplit

from django.conf import settings
from django.core import signing
from django.core.exceptions import (
    BadRequest,
    DisallowedHost,
    ImproperlyConfigured,
    RequestDataTooBig,
    TooManyFieldsSent,
)
from django.core.files import uploadhandler
from django.http.multipartparser import (
    MultiPartParser,
    MultiPartParserError,
    TooManyFilesSent,
)
from django.utils.datastructures import (
    CaseInsensitiveMapping,
    ImmutableList,
    MultiValueDict,
)
from django.utils.encoding import escape_uri_path, iri_to_uri
from django.utils.functional import cached_property
from django.utils.http import is_same_domain, parse_header_parameters
from django.utils.regex_helper import _lazy_re_compile

RAISE_ERROR = object()
host_validation_re = _lazy_re_compile(
    r"^([a-z0-9.-]+|\[[a-f0-9]*:[a-f0-9.:]+\])(?::([0-9]+))?$"
)


class UnreadablePostError(OSError):
    pass


class RawPostDataException(Exception):
    """
    You cannot access raw_post_data from a request that has
    multipart/* POST data if it has been accessed via POST,
    FILES, etc..
    """

    pass


class HttpRequest:
    """A basic HTTP request."""

    # The encoding used in GET/POST dicts. None means use default setting.
    _encoding = None
    _upload_handlers = []

    def __init__(self):
        # WARNING: The `WSGIRequest` subclass doesn't call `super`.
        # Any variable assignment made here should also happen in
        # `WSGIRequest.__init__()`.

        ...  # 12 line(s) collapsed ⟦tj:b9a9700f66d87c55899082077d7390ae⟧

    def __repr__(self):
        ...  # 7 line(s) collapsed ⟦tj:3624eaa7571754d7caf41137e20d182c⟧

    @cached_property
    def headers(self):
        return HttpHeaders(self.META)

    @cached_property
    def accepted_types(self):
        """Return a list of MediaType instances."""
        return parse_accept_header(self.headers.get("Accept", "*/*"))

    def accepts(self, media_type):
        return any(
            accepted_type.match(media_type) for accepted_type in self.accepted_types
        )

    def _set_content_type_params(self, meta):
        ...  # 11 line(s) collapsed ⟦tj:318f71979ba5bf1615ad7516e3d9d795⟧

    def _get_raw_host(self):
        ...  # 16 line(s) collapsed ⟦tj:ae3998ffd7f8984b619ad34f1c2c0a84⟧

    def get_host(self):
        ...  # 20 line(s) collapsed ⟦tj:92ead705017835fa7e977416b6ebe742⟧

    def get_port(self):
        ...  # 6 line(s) collapsed ⟦tj:a6cd2f4b9875f5b4287eac24730e5958⟧

    def get_full_path(self, force_append_slash=False):
        return self._get_full_path(self.path, force_append_slash)

    def get_full_path_info(self, force_append_slash=False):
        return self._get_full_path(self.path_info, force_append_slash)

    def _get_full_path(self, path, force_append_slash):
        # RFC 3986 requires query string arguments to be in the ASCII range.
        # Rather than crash if this doesn't happen, we encode defensively.
        ...  # 9 line(s) collapsed ⟦tj:8ae0640bc4a6e039ca6a1d6d8d248489⟧

    def get_signed_cookie(self, key, default=RAISE_ERROR, salt="", max_age=None):
        ...  # 22 line(s) collapsed ⟦tj:d7c5be08cdfd772cf5817ffdbba2a65b⟧

    def build_absolute_uri(self, location=None):
        ...  # 37 line(s) collapsed ⟦tj:9dcce7c20e41e6f5e6522f07bfeba41f⟧

    @cached_property
    def _current_scheme_host(self):
        return "{}://{}".format(self.scheme, self.get_host())

    def _get_scheme(self):
        ...  # 5 line(s) collapsed ⟦tj:d62fb1afebc60e5b1ead32fb5db7370f⟧

    @property
    def scheme(self):
        ...  # 13 line(s) collapsed ⟦tj:4cf453b034bcdd27c0cefccd2bd7a161⟧

    def is_secure(self):
        return self.scheme == "https"

    @property
    def encoding(self):
        return self._encoding

    @encoding.setter
    def encoding(self, val):
        ...  # 10 line(s) collapsed ⟦tj:c9804e728bfd4632633920a7843a9b8b⟧

    def _initialize_handlers(self):
        ...  # 4 line(s) collapsed ⟦tj:e9593daced472f85ff316627081d311a⟧

    @property
    def upload_handlers(self):
        ...  # 4 line(s) collapsed ⟦tj:96df52b9cbc7baf52e6faf73ff3267a8⟧

    @upload_handlers.setter
    def upload_handlers(self, upload_handlers):
        ...  # 6 line(s) collapsed ⟦tj:7a01c56a44f13610a0d4d8cc6a158274⟧

    def parse_file_upload(self, META, post_data):
        ...  # 10 line(s) collapsed ⟦tj:e45198db389c45d59776c6840cee3e5e⟧

    @property
    def body(self):
        ...  # 24 line(s) collapsed ⟦tj:383069cacbc7b7d2c83d21bdff2a95fb⟧

    def _mark_post_parse_error(self):
        self._post = QueryDict()
        self._files = MultiValueDict()

    def _load_post_and_files(self):
        ...  # 42 line(s) collapsed ⟦tj:312c17501c406a0fb71790bf49b1bc97⟧

    def close(self):
        if hasattr(self, "_files"):
            for f in chain.from_iterable(list_[1] for list_ in self._files.lists()):
                f.close()

    # File-like and iterator interface.
    #
    # Expects self._stream to be set to an appropriate source of bytes by
    # a corresponding request subclass (e.g. WSGIRequest).
    # Also when request data has already been read by request.POST or
    # request.body, self._stream points to a BytesIO instance
    # containing that data.

    def read(self, *args, **kwargs):
        ...  # 5 line(s) collapsed ⟦tj:d3446275819ab253e49b1b48913498d5⟧

    def readline(self, *args, **kwargs):
        ...  # 5 line(s) collapsed ⟦tj:6b875550635b6d401440c4677ad204f7⟧

    def __iter__(self):
        return iter(self.readline, b"")

    def readlines(self):
        return list(self)


class HttpHeaders(CaseInsensitiveMapping):
    HTTP_PREFIX = "HTTP_"
    # PEP 333 gives two headers which aren't prepended with HTTP_.
    UNPREFIXED_HEADERS = {"CONTENT_TYPE", "CONTENT_LENGTH"}

    def __init__(self, environ):
        ...  # 6 line(s) collapsed ⟦tj:ebe66ca05658c532793442d98a155566⟧

    def __getitem__(self, key):
        """Allow header lookup using underscores in place of hyphens."""
        return super().__getitem__(key.replace("_", "-"))

    @classmethod
    def parse_header_name(cls, header):
        ...  # 5 line(s) collapsed ⟦tj:3c6a675f8d98c4c74f872d55ca44c1f5⟧

    @classmethod
    def to_wsgi_name(cls, header):
        ...  # 4 line(s) collapsed ⟦tj:60333ba51d054e97077d40b3a6395405⟧

    @classmethod
    def to_asgi_name(cls, header):
        return header.replace("-", "_").upper()

    @classmethod
    def to_wsgi_names(cls, headers):
        ...  # 4 line(s) collapsed ⟦tj:54ccd0ba249a1596e59d74688ba46f57⟧

    @classmethod
    def to_asgi_names(cls, headers):
        ...  # 4 line(s) collapsed ⟦tj:b148291d10aea80a2bd4933d8eb71387⟧


class QueryDict(MultiValueDict):
    """
    A specialized MultiValueDict which represents a query string.

    A QueryDict can be used to represent GET or POST data. It subclasses
    MultiValueDict since keys in such data can be repeated, for instance
    in the data from a form with a <select multiple> field.

    By default QueryDicts are immutable, though the copy() method
    will always return a mutable copy.

    Both keys and values set on this class are converted from the given encoding
    (DEFAULT_CHARSET by default) to str.
    """

    # These are both reset in __init__, but is specified here at the class
    # level so that unpickling will have valid values
    _mutable = True
    _encoding = None

    def __init__(self, query_string=None, mutable=False, encoding=None):
        ...  # 28 line(s) collapsed ⟦tj:21602b6d23cfbf1ffb99f54b59259830⟧

    @classmethod
    def fromkeys(cls, iterable, value="", mutable=False, encoding=None):
        ...  # 10 line(s) collapsed ⟦tj:ea89dde0f66ac8216d01388e25ec01d4⟧

    @property
    def encoding(self):
        if self._encoding is None:
            self._encoding = settings.DEFAULT_CHARSET
        return self._encoding

    @encoding.setter
    def encoding(self, value):
        self._encoding = value

    def _assert_mutable(self):
        if not self._mutable:
            raise AttributeError("This QueryDict instance is immutable")

    def __setitem__(self, key, value):
        ...  # 4 line(s) collapsed ⟦tj:ef7a601a575024b4440fc86e76a66c14⟧

    def __delitem__(self, key):
        self._assert_mutable()
        super().__delitem__(key)

    def __copy__(self):
        ...  # 4 line(s) collapsed ⟦tj:00483be2ce0b075eb4599b533f223ade⟧

    def __deepcopy__(self, memo):
        ...  # 5 line(s) collapsed ⟦tj:a2a4ead5359bcb95692a8ffe50172b21⟧

    def setlist(self, key, list_):
        ...  # 4 line(s) collapsed ⟦tj:18dfa97d9bbac3b1e1491d3a9264a793⟧

    def setlistdefault(self, key, default_list=None):
        self._assert_mutable()
        return super().setlistdefault(key, default_list)

    def appendlist(self, key, value):
        ...  # 4 line(s) collapsed ⟦tj:f00b5faf6a084a8fca195321539c74ef⟧

    def pop(self, key, *args):
        self._assert_mutable()
        return super().pop(key, *args)

    def popitem(self):
        self._assert_mutable()
        return super().popitem()

    def clear(self):
        self._assert_mutable()
        super().clear()

    def setdefault(self, key, default=None):
        ...  # 4 line(s) collapsed ⟦tj:6c4ba3e2c610a96b86c98b4429360636⟧

    def copy(self):
        """Return a mutable copy of this object."""
        return self.__deepcopy__({})

    def urlencode(self, safe=None):
        ...  # 30 line(s) collapsed ⟦tj:077d124acdf701eb6f0c0cf9ff26860f⟧


class MediaType:
    def __init__(self, media_type_raw_line):
        ...  # 4 line(s) collapsed ⟦tj:ec365039e2a9b4efbfe48dd414ed2df6⟧

    def __str__(self):
        ...  # 6 line(s) collapsed ⟦tj:829290fca6122314137bd218abc9009a⟧

    def __repr__(self):
        return "<%s: %s>" % (self.__class__.__qualname__, self)

    @property
    def is_all_types(self):
        return self.main_type == "*" and self.sub_type == "*"

    def match(self, other):
        ...  # 6 line(s) collapsed ⟦tj:91cac5eaab5279c9939f736c2a2f073e⟧


# It's neither necessary nor appropriate to use
# django.utils.encoding.force_str() for parsing URLs and form inputs. Thus,
# this slightly more restricted function, used by QueryDict.
def bytes_to_text(s, encoding):
    ...  # 11 line(s) collapsed ⟦tj:ec78e83a35a56554122206f8f2467627⟧


def split_domain_port(host):
    ...  # 11 line(s) collapsed ⟦tj:4997cd3dd5f1aee3a5966b311967fba3⟧


def validate_host(host, allowed_hosts):
    ...  # 17 line(s) collapsed ⟦tj:29bd899770727238f6dd31965295822f⟧


def parse_accept_header(header):
    return [MediaType(token) for token in header.split(",") if token.strip()]
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (25750 bytes) is available by calling tinyjuice_retrieve with token "48f28b669ac2435fad75241abaca52f4" (marker ⟦tj:48f28b669ac2435fad75241abaca52f4⟧)]