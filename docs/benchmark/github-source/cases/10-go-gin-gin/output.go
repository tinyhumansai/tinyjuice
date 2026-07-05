// Copyright 2014 Manu Martinez-Almeida. All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE file.

package gin

import (
	"fmt"
	"html/template"
	"net"
	"net/http"
	"os"
	"path"
	"regexp"
	"strings"
	"sync"

	"github.com/gin-gonic/gin/internal/bytesconv"
	"github.com/gin-gonic/gin/render"
	"golang.org/x/net/http2"
	"golang.org/x/net/http2/h2c"
)

const defaultMultipartMemory = 32 << 20 // 32 MB

var (
	default404Body = []byte("404 page not found")
	default405Body = []byte("405 method not allowed")
)

var defaultPlatform string

var defaultTrustedCIDRs = []*net.IPNet{
    { … 9 line(s) … ⟦tj:d30e8e961b5f84edc079373f424d6a7a⟧ }

var regSafePrefix = regexp.MustCompile("[^a-zA-Z0-9/-]+")
var regRemoveRepeatedChar = regexp.MustCompile("/{2,}")

// HandlerFunc defines the handler used by gin middleware as return value.
type HandlerFunc func(*Context)

// OptionFunc defines the function to change the default configuration
type OptionFunc func(*Engine)

// HandlersChain defines a HandlerFunc slice.
type HandlersChain []HandlerFunc

// Last returns the last handler in the chain. i.e. the last handler is the main one.
func (c HandlersChain) Last() HandlerFunc {
    { … 5 line(s) … ⟦tj:aa21f3b66885741332a66d9d1c950580⟧ }

// RouteInfo represents a request route's specification which contains method and path and its handler.
type RouteInfo struct {
    { … 5 line(s) … ⟦tj:db82b4563a6dd86f1eebdc8fd0cc8f27⟧ }

// RoutesInfo defines a RouteInfo slice.
type RoutesInfo []RouteInfo

// Trusted platforms
const (
	// PlatformGoogleAppEngine when running on Google App Engine. Trust X-Appengine-Remote-Addr
	// for determining the client's IP
	PlatformGoogleAppEngine = "X-Appengine-Remote-Addr"
	// PlatformCloudflare when using Cloudflare's CDN. Trust CF-Connecting-IP for determining
	// the client's IP
	PlatformCloudflare = "CF-Connecting-IP"
	// PlatformFlyIO when running on Fly.io. Trust Fly-Client-IP for determining the client's IP
	PlatformFlyIO = "Fly-Client-IP"
)

// Engine is the framework's instance, it contains the muxer, middleware and configuration settings.
// Create an instance of Engine, by using New() or Default()
type Engine struct {
    { … 87 line(s) … ⟦tj:f78abca3b9da69e14887ab4aa5614f74⟧ }

var _ IRouter = (*Engine)(nil)

// New returns a new blank Engine instance without any middleware attached.
// By default, the configuration is:
// - RedirectTrailingSlash:  true
// - RedirectFixedPath:      false
// - HandleMethodNotAllowed: false
// - ForwardedByClientIP:    true
// - UseRawPath:             false
// - UnescapePathValues:     true
func New(opts ...OptionFunc) *Engine {
    { … 30 line(s) … ⟦tj:112d18c1f97c7661e4813c5a976f670b⟧ }

// Default returns an Engine instance with the Logger and Recovery middleware already attached.
func Default(opts ...OptionFunc) *Engine {
    { … 5 line(s) … ⟦tj:346350f1a802708ebfc6d351e9eae8ba⟧ }

func (engine *Engine) Handler() http.Handler {
    { … 7 line(s) … ⟦tj:85f01b995431358cf10777f65661ee96⟧ }

func (engine *Engine) allocateContext(maxParams uint16) *Context {
    { … 4 line(s) … ⟦tj:4cb0ebe8c9a0c9d7f3168dad0977177e⟧ }

// Delims sets template left and right delims and returns an Engine instance.
func (engine *Engine) Delims(left, right string) *Engine {
	engine.delims = render.Delims{Left: left, Right: right}
	return engine
}

// SecureJsonPrefix sets the secureJSONPrefix used in Context.SecureJSON.
func (engine *Engine) SecureJsonPrefix(prefix string) *Engine {
	engine.secureJSONPrefix = prefix
	return engine
}

// LoadHTMLGlob loads HTML files identified by glob pattern
// and associates the result with HTML renderer.
func (engine *Engine) LoadHTMLGlob(pattern string) {
    { … 12 line(s) … ⟦tj:c1cfa93c3649127a14fde2dd1e2eebdd⟧ }

// LoadHTMLFiles loads a slice of HTML files
// and associates the result with HTML renderer.
func (engine *Engine) LoadHTMLFiles(files ...string) {
    { … 8 line(s) … ⟦tj:1a30f9b4f743c9dcc7a6c95cc378c168⟧ }

// SetHTMLTemplate associate a template with HTML renderer.
func (engine *Engine) SetHTMLTemplate(templ *template.Template) {
    { … 6 line(s) … ⟦tj:89ec77ab6c76f33bd902eabcde89e815⟧ }

// SetFuncMap sets the FuncMap used for template.FuncMap.
func (engine *Engine) SetFuncMap(funcMap template.FuncMap) {
	engine.FuncMap = funcMap
}

// NoRoute adds handlers for NoRoute. It returns a 404 code by default.
func (engine *Engine) NoRoute(handlers ...HandlerFunc) {
	engine.noRoute = handlers
	engine.rebuild404Handlers()
}

// NoMethod sets the handlers called when Engine.HandleMethodNotAllowed = true.
func (engine *Engine) NoMethod(handlers ...HandlerFunc) {
	engine.noMethod = handlers
	engine.rebuild405Handlers()
}

// Use attaches a global middleware to the router. i.e. the middleware attached through Use() will be
// included in the handlers chain for every single request. Even 404, 405, static files...
// For example, this is the right place for a logger or error management middleware.
func (engine *Engine) Use(middleware ...HandlerFunc) IRoutes {
    { … 5 line(s) … ⟦tj:b0647e005f1ba9248d288c3488d9929c⟧ }

// With returns a new Engine instance with the provided options.
func (engine *Engine) With(opts ...OptionFunc) *Engine {
    { … 6 line(s) … ⟦tj:332c880d40a41b40bca2d5b26e0b9764⟧ }

func (engine *Engine) rebuild404Handlers() {
	engine.allNoRoute = engine.combineHandlers(engine.noRoute)
}

func (engine *Engine) rebuild405Handlers() {
	engine.allNoMethod = engine.combineHandlers(engine.noMethod)
}

func (engine *Engine) addRoute(method, path string, handlers HandlersChain) {
    { … 22 line(s) … ⟦tj:5deeecbaea67c29b78eef1c2b4c912d6⟧ }

// Routes returns a slice of registered routes, including some useful information, such as:
// the http method, path and the handler name.
func (engine *Engine) Routes() (routes RoutesInfo) {
    { … 5 line(s) … ⟦tj:52b61bde08b45ac545d580d911935a66⟧ }

func iterate(path, method string, routes RoutesInfo, root *node) RoutesInfo {
    { … 15 line(s) … ⟦tj:eede543f50d4fd33f461447a03400d0e⟧ }

// Run attaches the router to a http.Server and starts listening and serving HTTP requests.
// It is a shortcut for http.ListenAndServe(addr, router)
// Note: this method will block the calling goroutine indefinitely unless an error happens.
func (engine *Engine) Run(addr ...string) (err error) {
    { … 12 line(s) … ⟦tj:a5a2c24dcee25964ee9ed18f5a2d59ca⟧ }

func (engine *Engine) prepareTrustedCIDRs() ([]*net.IPNet, error) {
    { … 27 line(s) … ⟦tj:c388988f790d046f1cf730bb5611f864⟧ }

// SetTrustedProxies set a list of network origins (IPv4 addresses,
// IPv4 CIDRs, IPv6 addresses or IPv6 CIDRs) from which to trust
// request's headers that contain alternative client IP when
// `(*gin.Engine).ForwardedByClientIP` is `true`. `TrustedProxies`
// feature is enabled by default, and it also trusts all proxies
// by default. If you want to disable this feature, use
// Engine.SetTrustedProxies(nil), then Context.ClientIP() will
// return the remote address directly.
func (engine *Engine) SetTrustedProxies(trustedProxies []string) error {
	engine.trustedProxies = trustedProxies
	return engine.parseTrustedProxies()
}

// isUnsafeTrustedProxies checks if Engine.trustedCIDRs contains all IPs, it's not safe if it has (returns true)
func (engine *Engine) isUnsafeTrustedProxies() bool {
	return engine.isTrustedProxy(net.ParseIP("0.0.0.0")) || engine.isTrustedProxy(net.ParseIP("::"))
}

// parseTrustedProxies parse Engine.trustedProxies to Engine.trustedCIDRs
func (engine *Engine) parseTrustedProxies() error {
    { … 4 line(s) … ⟦tj:c4c2e3f0d2d8a8119abf4a375763450b⟧ }

// isTrustedProxy will check whether the IP address is included in the trusted list according to Engine.trustedCIDRs
func (engine *Engine) isTrustedProxy(ip net.IP) bool {
    { … 10 line(s) … ⟦tj:cc54f8165d78b923fdf0d25cd4bf2eeb⟧ }

// validateHeader will parse X-Forwarded-For header and return the trusted client IP address
func (engine *Engine) validateHeader(header string) (clientIP string, valid bool) {
    { … 19 line(s) … ⟦tj:9b0d41c56cad29ffe197448d28883aab⟧ }

// parseIP parse a string representation of an IP and returns a net.IP with the
// minimum byte representation or nil if input is invalid.
func parseIP(ip string) net.IP {
    { … 10 line(s) … ⟦tj:d17148c18b636f842b70a94dcfadd096⟧ }

// RunTLS attaches the router to a http.Server and starts listening and serving HTTPS (secure) requests.
// It is a shortcut for http.ListenAndServeTLS(addr, certFile, keyFile, router)
// Note: this method will block the calling goroutine indefinitely unless an error happens.
func (engine *Engine) RunTLS(addr, certFile, keyFile string) (err error) {
    { … 11 line(s) … ⟦tj:c958d59fe4ec9efa0f38345cb2c77914⟧ }

// RunUnix attaches the router to a http.Server and starts listening and serving HTTP requests
// through the specified unix socket (i.e. a file).
// Note: this method will block the calling goroutine indefinitely unless an error happens.
func (engine *Engine) RunUnix(file string) (err error) {
    { … 18 line(s) … ⟦tj:2e07bf767bb9fe721b12817640f4ec17⟧ }

// RunFd attaches the router to a http.Server and starts listening and serving HTTP requests
// through the specified file descriptor.
// Note: this method will block the calling goroutine indefinitely unless an error happens.
func (engine *Engine) RunFd(fd int) (err error) {
    { … 17 line(s) … ⟦tj:c90f1d59d409ceac85b84cae3cc6cb5d⟧ }

// RunListener attaches the router to a http.Server and starts listening and serving HTTP requests
// through the specified net.Listener
func (engine *Engine) RunListener(listener net.Listener) (err error) {
    { … 11 line(s) … ⟦tj:62f86923a1025587e486f9457a3f1427⟧ }

// ServeHTTP conforms to the http.Handler interface.
func (engine *Engine) ServeHTTP(w http.ResponseWriter, req *http.Request) {
    { … 9 line(s) … ⟦tj:088a419231b2bd43ac9fd91324cc0e21⟧ }

// HandleContext re-enters a context that has been rewritten.
// This can be done by setting c.Request.URL.Path to your new target.
// Disclaimer: You can loop yourself to deal with this, use wisely.
func (engine *Engine) HandleContext(c *Context) {
    { … 6 line(s) … ⟦tj:5bd897096d7e8dc95ee08cdc94bf66a4⟧ }

func (engine *Engine) handleHTTPRequest(c *Context) {
    { … 66 line(s) … ⟦tj:0a6be3fbb3ae01e55804912581bd4de9⟧ }

var mimePlain = []string{MIMEPlain}

func serveError(c *Context, code int, defaultMessage []byte) {
    { … 9 line(s) … ⟦tj:dcaefaa97e3fc762560a61e5c866c6f0⟧ }
			debugPrint("cannot write message to writer during serve error: %v", err)
    { … 5 line(s) … ⟦tj:b3e9e8d636942a48caa765bb15610175⟧ }

func redirectTrailingSlash(c *Context) {
    { … 14 line(s) … ⟦tj:b18e7c6c786f377d0e93c83e5a810f52⟧ }

func redirectFixedPath(c *Context, root *node, trailingSlash bool) bool {
    { … 10 line(s) … ⟦tj:d755f6528f617b784bf641fcc85db54f⟧ }

func redirectRequest(c *Context) {
    { … 12 line(s) … ⟦tj:9ad853a1d85c5d4bfc75a62a27680eea⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (23812 bytes) is available by calling tinyjuice_retrieve with token "b396db448f7ac685ae9384a0a6bc9712" (marker ⟦tj:b396db448f7ac685ae9384a0a6bc9712⟧)]