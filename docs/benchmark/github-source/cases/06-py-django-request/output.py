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

        self.GET = QueryDict(mutable=True)
        self.POST = QueryDict(mutable=True)
        self.COOKIES = {}
        self.META = {}
        self.FILES = MultiValueDict()

        self.path = ""
        self.path_info = ""
        self.method = None
        self.resolver_match = None
        self.content_type = None
        self.content_params = None

    def __repr__(self):
        if self.method is None or not self.get_full_path():
            return "<%s>" % self.__class__.__name__
        return "<%s: %s %r>" % (
            self.__class__.__name__,
            self.method,
            self.get_full_path(),
        )

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
        """
        Return the HTTP host using the environment or request headers. Skip
...  # 13 line(s) collapsed ⟦tj:ae3998ffd7f8984b619ad34f1c2c0a84⟧
        return host

    def get_host(self):
        """Return the HTTP host using the environment or request headers."""
        host = self._get_raw_host()
...  # 17 line(s) collapsed ⟦tj:92ead705017835fa7e977416b6ebe742⟧
            raise DisallowedHost(msg)

    def get_port(self):
        """Return the port number for the request as a string."""
        if settings.USE_X_FORWARDED_PORT and "HTTP_X_FORWARDED_PORT" in self.META:
            port = self.META["HTTP_X_FORWARDED_PORT"]
        else:
            port = self.META["SERVER_PORT"]
        return str(port)

    def get_full_path(self, force_append_slash=False):
        return self._get_full_path(self.path, force_append_slash)

    def get_full_path_info(self, force_append_slash=False):
        return self._get_full_path(self.path_info, force_append_slash)

    def _get_full_path(self, path, force_append_slash):
        # RFC 3986 requires query string arguments to be in the ASCII range.
        # Rather than crash if this doesn't happen, we encode defensively.
        ...  # 9 line(s) collapsed ⟦tj:8ae0640bc4a6e039ca6a1d6d8d248489⟧

    def get_signed_cookie(self, key, default=RAISE_ERROR, salt="", max_age=None):
        """
        Attempt to return a signed cookie. If the signature fails or the
        cookie has expired, raise an exception, unless the `default` argument
        is provided,  in which case return that value.
        """
        try:
            cookie_value = self.COOKIES[key]
        except KeyError:
            if default is not RAISE_ERROR:
                return default
            else:
                raise
        try:
            value = signing.get_cookie_signer(salt=key + salt).unsign(
                cookie_value, max_age=max_age
            )
        except signing.BadSignature:
            if default is not RAISE_ERROR:
                return default
            else:
                raise
        return value

    def build_absolute_uri(self, location=None):
        """
        Build an absolute URI from the location and the variables available in
...  # 34 line(s) collapsed ⟦tj:9dcce7c20e41e6f5e6522f07bfeba41f⟧
        return iri_to_uri(location)

    @cached_property
    def _current_scheme_host(self):
        return "{}://{}".format(self.scheme, self.get_host())

    def _get_scheme(self):
        """
        Hook for subclasses like WSGIRequest to implement. Return 'http' by
        default.
        """
        return "http"

    @property
    def scheme(self):
        if settings.SECURE_PROXY_SSL_HEADER:
            try:
                header, secure_value = settings.SECURE_PROXY_SSL_HEADER
            except ValueError:
                raise ImproperlyConfigured(
                    "The SECURE_PROXY_SSL_HEADER setting must be a tuple containing "
                    "two values."
                )
            header_value = self.META.get(header)
            if header_value is not None:
                header_value, *_ = header_value.split(",", 1)
                return "https" if header_value.strip() == secure_value else "http"
        return self._get_scheme()

    def is_secure(self):
        return self.scheme == "https"

    @property
    def encoding(self):
        return self._encoding

    @encoding.setter
    def encoding(self, val):
        ...  # 10 line(s) collapsed ⟦tj:c9804e728bfd4632633920a7843a9b8b⟧

    def _initialize_handlers(self):
        self._upload_handlers = [
            uploadhandler.load_handler(handler, self)
            for handler in settings.FILE_UPLOAD_HANDLERS
        ]

    @property
    def upload_handlers(self):
        if not self._upload_handlers:
            # If there are no upload handlers defined, initialize them from settings.
            self._initialize_handlers()
        return self._upload_handlers

    @upload_handlers.setter
    def upload_handlers(self, upload_handlers):
        if hasattr(self, "_files"):
            raise AttributeError(
                "You cannot set the upload handlers after the upload has been "
                "processed."
            )
        self._upload_handlers = upload_handlers

    def parse_file_upload(self, META, post_data):
        ...  # 10 line(s) collapsed ⟦tj:e45198db389c45d59776c6840cee3e5e⟧

    @property
    def body(self):
        if not hasattr(self, "_body"):
            if self._read_started:
                raise RawPostDataException(
                    "You cannot access body after reading from request's data stream"
                )

            # Limit the maximum request data size that will be handled in-memory.
            if (
                settings.DATA_UPLOAD_MAX_MEMORY_SIZE is not None
                and int(self.META.get("CONTENT_LENGTH") or 0)
                > settings.DATA_UPLOAD_MAX_MEMORY_SIZE
            ):
                raise RequestDataTooBig(
                    "Request body exceeded settings.DATA_UPLOAD_MAX_MEMORY_SIZE."
                )

            try:
                self._body = self.read()
            except OSError as e:
                raise UnreadablePostError(*e.args) from e
            finally:
                self._stream.close()
            self._stream = BytesIO(self._body)
        return self._body

    def _mark_post_parse_error(self):
        self._post = QueryDict()
        self._files = MultiValueDict()

    def _load_post_and_files(self):
        """Populate self._post and self._files if the content-type is a form type"""
        if self.method != "POST":
            self._post, self._files = (
                QueryDict(encoding=self._encoding),
                MultiValueDict(),
            )
            return
        if self._read_started and not hasattr(self, "_body"):
            self._mark_post_parse_error()
            return

        if self.content_type == "multipart/form-data":
            if hasattr(self, "_body"):
                # Use already read data
                data = BytesIO(self._body)
            else:
                data = self
            try:
                self._post, self._files = self.parse_file_upload(self.META, data)
            except (MultiPartParserError, TooManyFilesSent):
                # An error occurred while parsing POST data. Since when
                # formatting the error the request handler might access
                # self.POST, set self._post and self._file to prevent
                # attempts to parse POST data again.
                self._mark_post_parse_error()
                raise
        elif self.content_type == "application/x-www-form-urlencoded":
            # According to RFC 1866, the "application/x-www-form-urlencoded"
            # content type does not have a charset and should be always treated
            # as UTF-8.
            if self._encoding is not None and self._encoding.lower() != "utf-8":
                raise BadRequest(
                    "HTTP requests with the 'application/x-www-form-urlencoded' "
                    "content type must be UTF-8 encoded."
                )
            self._post = QueryDict(self.body, encoding="utf-8")
            self._files = MultiValueDict()
        else:
            self._post, self._files = (
                QueryDict(encoding=self._encoding),
                MultiValueDict(),
            )

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
        self._read_started = True
        try:
            return self._stream.read(*args, **kwargs)
        except OSError as e:
            raise UnreadablePostError(*e.args) from e

    def readline(self, *args, **kwargs):
        self._read_started = True
        try:
            return self._stream.readline(*args, **kwargs)
        except OSError as e:
            raise UnreadablePostError(*e.args) from e

    def __iter__(self):
        return iter(self.readline, b"")

    def readlines(self):
        return list(self)


class HttpHeaders(CaseInsensitiveMapping):
    HTTP_PREFIX = "HTTP_"
    # PEP 333 gives two headers which aren't prepended with HTTP_.
    UNPREFIXED_HEADERS = {"CONTENT_TYPE", "CONTENT_LENGTH"}

    def __init__(self, environ):
        headers = {}
        for header, value in environ.items():
            name = self.parse_header_name(header)
            if name:
                headers[name] = value
        super().__init__(headers)

    def __getitem__(self, key):
        """Allow header lookup using underscores in place of hyphens."""
        return super().__getitem__(key.replace("_", "-"))

    @classmethod
    def parse_header_name(cls, header):
        if header.startswith(cls.HTTP_PREFIX):
            header = header.removeprefix(cls.HTTP_PREFIX)
        elif header not in cls.UNPREFIXED_HEADERS:
            return None
        return header.replace("_", "-").title()

    @classmethod
    def to_wsgi_name(cls, header):
        header = header.replace("-", "_").upper()
        if header in cls.UNPREFIXED_HEADERS:
            return header
        return f"{cls.HTTP_PREFIX}{header}"

    @classmethod
    def to_asgi_name(cls, header):
        return header.replace("-", "_").upper()

    @classmethod
    def to_wsgi_names(cls, headers):
        return {
            cls.to_wsgi_name(header_name): value
            for header_name, value in headers.items()
        }

    @classmethod
    def to_asgi_names(cls, headers):
        return {
            cls.to_asgi_name(header_name): value
            for header_name, value in headers.items()
        }


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
        super().__init__()
        self.encoding = encoding or settings.DEFAULT_CHARSET
        query_string = query_string or ""
        parse_qsl_kwargs = {
            "keep_blank_values": True,
            "encoding": self.encoding,
            "max_num_fields": settings.DATA_UPLOAD_MAX_NUMBER_FIELDS,
        }
        if isinstance(query_string, bytes):
            # query_string normally contains URL-encoded data, a subset of ASCII.
            try:
                query_string = query_string.decode(self.encoding)
            except UnicodeDecodeError:
                # ... but some user agents are misbehaving :-(
                query_string = query_string.decode("iso-8859-1")
        try:
            for key, value in parse_qsl(query_string, **parse_qsl_kwargs):
                self.appendlist(key, value)
        except ValueError as e:
            # ValueError can also be raised if the strict_parsing argument to
            # parse_qsl() is True. As that is not used by Django, assume that
            # the exception was raised by exceeding the value of max_num_fields
            # instead of fragile checks of exception message strings.
            raise TooManyFieldsSent(
                "The number of GET/POST parameters exceeded "
                "settings.DATA_UPLOAD_MAX_NUMBER_FIELDS."
            ) from e
        self._mutable = mutable

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
        self._assert_mutable()
        key = bytes_to_text(key, self.encoding)
        value = bytes_to_text(value, self.encoding)
        super().__setitem__(key, value)

    def __delitem__(self, key):
        self._assert_mutable()
        super().__delitem__(key)

    def __copy__(self):
        result = self.__class__("", mutable=True, encoding=self.encoding)
        for key, value in self.lists():
            result.setlist(key, value)
        return result

    def __deepcopy__(self, memo):
        result = self.__class__("", mutable=True, encoding=self.encoding)
        memo[id(self)] = result
        for key, value in self.lists():
            result.setlist(copy.deepcopy(key, memo), copy.deepcopy(value, memo))
        return result

    def setlist(self, key, list_):
        self._assert_mutable()
        key = bytes_to_text(key, self.encoding)
        list_ = [bytes_to_text(elt, self.encoding) for elt in list_]
        super().setlist(key, list_)

    def setlistdefault(self, key, default_list=None):
        self._assert_mutable()
        return super().setlistdefault(key, default_list)

    def appendlist(self, key, value):
        self._assert_mutable()
        key = bytes_to_text(key, self.encoding)
        value = bytes_to_text(value, self.encoding)
        super().appendlist(key, value)

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
        self._assert_mutable()
        key = bytes_to_text(key, self.encoding)
        default = bytes_to_text(default, self.encoding)
        return super().setdefault(key, default)

    def copy(self):
        """Return a mutable copy of this object."""
        return self.__deepcopy__({})

    def urlencode(self, safe=None):
        """
        Return an encoded string of all query string arguments.
...  # 27 line(s) collapsed ⟦tj:077d124acdf701eb6f0c0cf9ff26860f⟧
        return "&".join(output)


class MediaType:
    def __init__(self, media_type_raw_line):
        full_type, self.params = parse_header_parameters(
            media_type_raw_line if media_type_raw_line else ""
        )
        self.main_type, _, self.sub_type = full_type.partition("/")

    def __str__(self):
        params_str = "".join("; %s=%s" % (k, v) for k, v in self.params.items())
        return "%s%s%s" % (
            self.main_type,
            ("/%s" % self.sub_type) if self.sub_type else "",
            params_str,
        )

    def __repr__(self):
        return "<%s: %s>" % (self.__class__.__qualname__, self)

    @property
    def is_all_types(self):
        return self.main_type == "*" and self.sub_type == "*"

    def match(self, other):
        if self.is_all_types:
            return True
        other = MediaType(other)
        if self.main_type == other.main_type and self.sub_type in {"*", other.sub_type}:
            return True
        return False


# It's neither necessary nor appropriate to use
# django.utils.encoding.force_str() for parsing URLs and form inputs. Thus,
# this slightly more restricted function, used by QueryDict.
def bytes_to_text(s, encoding):
    ...  # 11 line(s) collapsed ⟦tj:ec78e83a35a56554122206f8f2467627⟧


def split_domain_port(host):
    ...  # 11 line(s) collapsed ⟦tj:4997cd3dd5f1aee3a5966b311967fba3⟧


def validate_host(host, allowed_hosts):
    """
    Validate the given host for this site.
...  # 14 line(s) collapsed ⟦tj:29bd899770727238f6dd31965295822f⟧
    )


def parse_accept_header(header):
    return [MediaType(token) for token in header.split(",") if token.strip()]
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (25750 bytes) is available by calling tinyjuice_retrieve with token "48f28b669ac2435fad75241abaca52f4" (marker ⟦tj:48f28b669ac2435fad75241abaca52f4⟧)]