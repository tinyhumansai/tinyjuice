from __future__ import annotations

import collections.abc as cabc
import os
import sys
import typing as t
import weakref
from datetime import timedelta
from inspect import iscoroutinefunction
from itertools import chain
from types import TracebackType
from urllib.parse import quote as _url_quote

import click
from werkzeug.datastructures import Headers
from werkzeug.datastructures import ImmutableDict
from werkzeug.exceptions import BadRequestKeyError
from werkzeug.exceptions import HTTPException
from werkzeug.exceptions import InternalServerError
from werkzeug.routing import BuildError
from werkzeug.routing import MapAdapter
from werkzeug.routing import RequestRedirect
from werkzeug.routing import RoutingException
from werkzeug.routing import Rule
from werkzeug.serving import is_running_from_reloader
from werkzeug.wrappers import Response as BaseResponse

from . import cli
from . import typing as ft
from .ctx import AppContext
from .ctx import RequestContext
from .globals import _cv_app
from .globals import _cv_request
from .globals import current_app
from .globals import g
from .globals import request
from .globals import request_ctx
from .globals import session
from .helpers import get_debug_flag
from .helpers import get_flashed_messages
from .helpers import get_load_dotenv
from .helpers import send_from_directory
from .sansio.app import App
from .sansio.scaffold import _sentinel
from .sessions import SecureCookieSessionInterface
from .sessions import SessionInterface
from .signals import appcontext_tearing_down
from .signals import got_request_exception
from .signals import request_finished
from .signals import request_started
from .signals import request_tearing_down
from .templating import Environment
from .wrappers import Request
from .wrappers import Response

if t.TYPE_CHECKING:  # pragma: no cover
    from _typeshed.wsgi import StartResponse
    from _typeshed.wsgi import WSGIEnvironment

    from .testing import FlaskClient
    from .testing import FlaskCliRunner

T_shell_context_processor = t.TypeVar(
    "T_shell_context_processor", bound=ft.ShellContextProcessorCallable
)
T_teardown = t.TypeVar("T_teardown", bound=ft.TeardownCallable)
T_template_filter = t.TypeVar("T_template_filter", bound=ft.TemplateFilterCallable)
T_template_global = t.TypeVar("T_template_global", bound=ft.TemplateGlobalCallable)
T_template_test = t.TypeVar("T_template_test", bound=ft.TemplateTestCallable)


def _make_timedelta(value: timedelta | int | None) -> timedelta | None:
    ...  # 4 line(s) collapsed ⟦tj:b60a2157d06e3e85f2a4d553eb7d29e3⟧


class Flask(App):
    """The flask object implements a WSGI application and acts as the central
    object.  It is passed the name of the module or package of the
    application.  Once it is created it will act as a central registry for
    the view functions, the URL rules, template configuration and much more.

    The name of the package is used to resolve resources from inside the
    package or the folder the module is contained in depending on if the
    package parameter resolves to an actual python package (a folder with
    an :file:`__init__.py` file inside) or a standard module (just a ``.py`` file).

    For more information about resource loading, see :func:`open_resource`.

    Usually you create a :class:`Flask` instance in your main module or
    in the :file:`__init__.py` file of your package like this::

        from flask import Flask
        app = Flask(__name__)

    .. admonition:: About the First Parameter

        The idea of the first parameter is to give Flask an idea of what
        belongs to your application.  This name is used to find resources
        on the filesystem, can be used by extensions to improve debugging
        information and a lot more.

        So it's important what you provide there.  If you are using a single
        module, `__name__` is always the correct value.  If you however are
        using a package, it's usually recommended to hardcode the name of
        your package there.

        For example if your application is defined in :file:`yourapplication/app.py`
        you should create it with one of the two versions below::

            app = Flask('yourapplication')
            app = Flask(__name__.split('.')[0])

        Why is that?  The application will work even with `__name__`, thanks
        to how resources are looked up.  However it will make debugging more
        painful.  Certain extensions can make assumptions based on the
        import name of your application.  For example the Flask-SQLAlchemy
        extension will look for the code in your application that triggered
        an SQL query in debug mode.  If the import name is not properly set
        up, that debugging information is lost.  (For example it would only
        pick up SQL queries in `yourapplication.app` and not
        `yourapplication.views.frontend`)

    .. versionadded:: 0.7
       The `static_url_path`, `static_folder`, and `template_folder`
       parameters were added.

    .. versionadded:: 0.8
       The `instance_path` and `instance_relative_config` parameters were
       added.

    .. versionadded:: 0.11
       The `root_path` parameter was added.

    .. versionadded:: 1.0
       The ``host_matching`` and ``static_host`` parameters were added.

    .. versionadded:: 1.0
       The ``subdomain_matching`` parameter was added. Subdomain
       matching needs to be enabled manually now. Setting
       :data:`SERVER_NAME` does not implicitly enable it.

    :param import_name: the name of the application package
    :param static_url_path: can be used to specify a different path for the
                            static files on the web.  Defaults to the name
                            of the `static_folder` folder.
    :param static_folder: The folder with static files that is served at
        ``static_url_path``. Relative to the application ``root_path``
        or an absolute path. Defaults to ``'static'``.
    :param static_host: the host to use when adding the static route.
        Defaults to None. Required when using ``host_matching=True``
        with a ``static_folder`` configured.
    :param host_matching: set ``url_map.host_matching`` attribute.
        Defaults to False.
    :param subdomain_matching: consider the subdomain relative to
        :data:`SERVER_NAME` when matching routes. Defaults to False.
    :param template_folder: the folder that contains the templates that should
                            be used by the application.  Defaults to
                            ``'templates'`` folder in the root path of the
                            application.
    :param instance_path: An alternative instance path for the application.
                          By default the folder ``'instance'`` next to the
                          package or module is assumed to be the instance
                          path.
    :param instance_relative_config: if set to ``True`` relative filenames
                                     for loading the config are assumed to
                                     be relative to the instance path instead
                                     of the application root.
    :param root_path: The path to the root of the application files.
        This should only be set manually when it can't be detected
        automatically, such as for namespace packages.
    """

    default_config = ImmutableDict(
        {
            "DEBUG": None,
            "TESTING": False,
            "PROPAGATE_EXCEPTIONS": None,
            "SECRET_KEY": None,
            "PERMANENT_SESSION_LIFETIME": timedelta(days=31),
            "USE_X_SENDFILE": False,
            "SERVER_NAME": None,
            "APPLICATION_ROOT": "/",
            "SESSION_COOKIE_NAME": "session",
            "SESSION_COOKIE_DOMAIN": None,
            "SESSION_COOKIE_PATH": None,
            "SESSION_COOKIE_HTTPONLY": True,
            "SESSION_COOKIE_SECURE": False,
            "SESSION_COOKIE_SAMESITE": None,
            "SESSION_REFRESH_EACH_REQUEST": True,
            "MAX_CONTENT_LENGTH": None,
            "SEND_FILE_MAX_AGE_DEFAULT": None,
            "TRAP_BAD_REQUEST_ERRORS": None,
            "TRAP_HTTP_EXCEPTIONS": False,
            "EXPLAIN_TEMPLATE_LOADING": False,
            "PREFERRED_URL_SCHEME": "http",
            "TEMPLATES_AUTO_RELOAD": None,
            "MAX_COOKIE_SIZE": 4093,
        }
    )

    #: The class that is used for request objects.  See :class:`~flask.Request`
    #: for more information.
    request_class: type[Request] = Request

    #: The class that is used for response objects.  See
    #: :class:`~flask.Response` for more information.
    response_class: type[Response] = Response

    #: the session interface to use.  By default an instance of
    #: :class:`~flask.sessions.SecureCookieSessionInterface` is used here.
    #:
    #: .. versionadded:: 0.8
    session_interface: SessionInterface = SecureCookieSessionInterface()

    def __init__(
        self,
        import_name: str,
        static_url_path: str | None = None,
        static_folder: str | os.PathLike[str] | None = "static",
        static_host: str | None = None,
        host_matching: bool = False,
        subdomain_matching: bool = False,
        template_folder: str | os.PathLike[str] | None = "templates",
        instance_path: str | None = None,
        instance_relative_config: bool = False,
        root_path: str | None = None,
    ):
        ...  # 41 line(s) collapsed ⟦tj:33e1d2a8fe9e99df708dd2784dd03098⟧

    def get_send_file_max_age(self, filename: str | None) -> int | None:
        ...  # 25 line(s) collapsed ⟦tj:4713e30d30b4eb3de8df87f0b612ea47⟧

    def send_static_file(self, filename: str) -> Response:
        ...  # 20 line(s) collapsed ⟦tj:da668d109c7e2bc5df9b3b5782b08d71⟧

    def open_resource(self, resource: str, mode: str = "rb") -> t.IO[t.AnyStr]:
        ...  # 25 line(s) collapsed ⟦tj:070d49e3913c3b312720de79ddd4009d⟧

    def open_instance_resource(self, resource: str, mode: str = "rb") -> t.IO[t.AnyStr]:
        ...  # 10 line(s) collapsed ⟦tj:6488ef7bff4b8b020caba6a861c93177⟧

    def create_jinja_environment(self) -> Environment:
        ...  # 38 line(s) collapsed ⟦tj:19098d937d8cc654d03b9595b7528931⟧

    def create_url_adapter(self, request: Request | None) -> MapAdapter | None:
        ...  # 38 line(s) collapsed ⟦tj:e058b20218aa00bbb1b8e3aeba6caa78⟧

    def raise_routing_exception(self, request: Request) -> t.NoReturn:
        ...  # 26 line(s) collapsed ⟦tj:79c97404a030c5c07676d307a4204048⟧

    def update_template_context(self, context: dict[str, t.Any]) -> None:
        ...  # 26 line(s) collapsed ⟦tj:e701ec0b25dbb03c89713be839aa6feb⟧

    def make_shell_context(self) -> dict[str, t.Any]:
        ...  # 10 line(s) collapsed ⟦tj:adcd6e42230232154ed95b3161ec878d⟧

    def run(
        self,
        host: str | None = None,
        port: int | None = None,
        debug: bool | None = None,
        load_dotenv: bool = True,
        **options: t.Any,
    ) -> None:
        ...  # 114 line(s) collapsed ⟦tj:c4177563c6b295ed2421dd70449a5f78⟧

    def test_client(self, use_cookies: bool = True, **kwargs: t.Any) -> FlaskClient:
        ...  # 56 line(s) collapsed ⟦tj:0c712de2ffe9fca87174ab5e6e0f9b4a⟧

    def test_cli_runner(self, **kwargs: t.Any) -> FlaskCliRunner:
        ...  # 15 line(s) collapsed ⟦tj:93b741d1ede9d68babbcffd29dfeb118⟧

    def handle_http_exception(
        self, e: HTTPException
    ) -> HTTPException | ft.ResponseReturnValue:
        ...  # 31 line(s) collapsed ⟦tj:2ef6fda27b1f5adbdb353c0cbce2b83a⟧

    def handle_user_exception(
        self, e: Exception
    ) -> HTTPException | ft.ResponseReturnValue:
        ...  # 28 line(s) collapsed ⟦tj:5f14cc24d45847610b47ef57e89d08ae⟧

    def handle_exception(self, e: Exception) -> Response:
        ...  # 51 line(s) collapsed ⟦tj:2d1dfae3db6bddb3d320b24e64d0bb6f⟧

    def log_exception(
        self,
        exc_info: (tuple[type, BaseException, TracebackType] | tuple[None, None, None]),
    ) -> None:
        ...  # 10 line(s) collapsed ⟦tj:254d98d0ee2c5663b3cbe11b0518168e⟧

    def dispatch_request(self) -> ft.ResponseReturnValue:
        ...  # 23 line(s) collapsed ⟦tj:4a241628a3cadb61573bdae0a57339f9⟧

    def full_dispatch_request(self) -> Response:
        ...  # 16 line(s) collapsed ⟦tj:51e3067e2c6cab6c1dcfd76667b6cd90⟧

    def finalize_request(
        self,
        rv: ft.ResponseReturnValue | HTTPException,
        from_error_handler: bool = False,
    ) -> Response:
        ...  # 25 line(s) collapsed ⟦tj:c8d937043dfda93b95dea0200c5fcb24⟧

    def make_default_options_response(self) -> Response:
        ...  # 11 line(s) collapsed ⟦tj:12f07f5d4742ea14eedb24e8eeab1eb3⟧

    def ensure_sync(self, func: t.Callable[..., t.Any]) -> t.Callable[..., t.Any]:
        ...  # 12 line(s) collapsed ⟦tj:3d3153960708e6865d4280d59106d05c⟧

    def async_to_sync(
        self, func: t.Callable[..., t.Coroutine[t.Any, t.Any, t.Any]]
    ) -> t.Callable[..., t.Any]:
        ...  # 19 line(s) collapsed ⟦tj:794ffb82b1ab45923fe7de5bebb581bf⟧

    def url_for(
        self,
        /,
        endpoint: str,
        *,
        _anchor: str | None = None,
        _method: str | None = None,
        _scheme: str | None = None,
        _external: bool | None = None,
        **values: t.Any,
    ) -> str:
        ...  # 114 line(s) collapsed ⟦tj:877a2787e89ac183e504992c17eb163f⟧

    def make_response(self, rv: ft.ResponseReturnValue) -> Response:
        ...  # 139 line(s) collapsed ⟦tj:e2b77e17655d18e43807c50b1493c349⟧

    def preprocess_request(self) -> ft.ResponseReturnValue | None:
        ...  # 25 line(s) collapsed ⟦tj:934dee1f07c7ad6f7d6bccefd91e3fd8⟧

    def process_response(self, response: Response) -> Response:
        ...  # 26 line(s) collapsed ⟦tj:e1f5d5e3babc68e49f5801318ad608e4⟧

    def do_teardown_request(
        self,
        exc: BaseException | None = _sentinel,  # type: ignore[assignment]
    ) -> None:
        ...  # 29 line(s) collapsed ⟦tj:aaa6460556c542ec009835098202b4e0⟧

    def do_teardown_appcontext(
        self,
        exc: BaseException | None = _sentinel,  # type: ignore[assignment]
    ) -> None:
        ...  # 21 line(s) collapsed ⟦tj:b43e506a3e423ddbd1434453bb7dc2ce⟧

    def app_context(self) -> AppContext:
        ...  # 19 line(s) collapsed ⟦tj:7bb3ba5466759be6893e302adbbacb3c⟧

    def request_context(self, environ: WSGIEnvironment) -> RequestContext:
        ...  # 14 line(s) collapsed ⟦tj:3f9c7449707f2e11608a406028bb820c⟧

    def test_request_context(self, *args: t.Any, **kwargs: t.Any) -> RequestContext:
        ...  # 54 line(s) collapsed ⟦tj:0a33ef5a60fe1edbccc9475bf5e6fcf3⟧

    def wsgi_app(
        self, environ: WSGIEnvironment, start_response: StartResponse
    ) -> cabc.Iterable[bytes]:
        ...  # 46 line(s) collapsed ⟦tj:5d14d166a48f95489c358aef70257d0f⟧

    def __call__(
        self, environ: WSGIEnvironment, start_response: StartResponse
    ) -> cabc.Iterable[bytes]:
        ...  # 5 line(s) collapsed ⟦tj:800589c9c650d2a02f59f3082768eda8⟧
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (60143 bytes) is available by calling tinyjuice_retrieve with token "efe961e9c223dbbae24c4d50d7c3a4d6" (marker ⟦tj:efe961e9c223dbbae24c4d50d7c3a4d6⟧)]