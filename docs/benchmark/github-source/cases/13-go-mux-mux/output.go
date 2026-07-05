// Copyright 2012 The Gorilla Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package mux

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"path"
	"regexp"
)

var (
	// ErrMethodMismatch is returned when the method in the request does not match
	// the method defined against the route.
	ErrMethodMismatch = errors.New("method is not allowed")
	// ErrNotFound is returned when no route match is found.
	ErrNotFound = errors.New("no matching route was found")
)

// NewRouter returns a new router instance.
func NewRouter() *Router {
	return &Router{namedRoutes: make(map[string]*Route)}
}

// Router registers routes to be matched and dispatches a handler.
//
// It implements the http.Handler interface, so it can be registered to serve
// requests:
//
//	var router = mux.NewRouter()
//
//	func main() {
//	    http.Handle("/", router)
//	}
//
// Or, for Google App Engine, register it in a init() function:
//
//	func init() {
//	    http.Handle("/", router)
//	}
//
// This will send all incoming requests to the router.
type Router struct {
	// Configurable Handler to be used when no route matches.
	// This can be used to render your own 404 Not Found errors.
	NotFoundHandler http.Handler

	// Configurable Handler to be used when the request method does not match the route.
	// This can be used to render your own 405 Method Not Allowed errors.
    { … 19 line(s) … ⟦tj:9b029891beb6a901dc16ac69684388d7⟧ }

// common route configuration shared between `Router` and `Route`
type routeConf struct {
    { … 22 line(s) … ⟦tj:de1aa572ce6783527c8e2d7507a5f51b⟧ }

// returns an effective deep copy of `routeConf`
func copyRouteConf(r routeConf) routeConf {
    { … 20 line(s) … ⟦tj:cd7a50514ac30f3c9a09a4d719349300⟧ }

func copyRouteRegexp(r *routeRegexp) *routeRegexp {
	c := *r
	return &c
}

// Match attempts to match the given request against the router's registered routes.
//
// If the request matches a route of this router or one of its subrouters the Route,
// Handler, and Vars fields of the the match argument are filled and this function
// returns true.
//
// If the request does not match any of this router's or its subrouters' routes
// then this function returns false. If available, a reason for the match failure
// will be filled in the match argument's MatchErr field. If the match failure type
// (eg: not found) has a registered handler, the handler is assigned to the Handler
// field of the match argument.
func (r *Router) Match(req *http.Request, match *RouteMatch) bool {
	for _, route := range r.routes {
		if route.Match(req, match) {
			// Build middleware chain if no error was found
    { … 28 line(s) … ⟦tj:c4213d5026ffbc8624f53988951b27d1⟧ }

// ServeHTTP dispatches the handler registered in the matched route.
//
// When there is a match, the route variables can be retrieved calling
// mux.Vars(request).
func (r *Router) ServeHTTP(w http.ResponseWriter, req *http.Request) {
    { … 38 line(s) … ⟦tj:f1ad1dff11f88ef87fabd69f02227cc3⟧ }

// Get returns a route registered with the given name.
func (r *Router) Get(name string) *Route {
	return r.namedRoutes[name]
}

// GetRoute returns a route registered with the given name. This method
// was renamed to Get() and remains here for backwards compatibility.
func (r *Router) GetRoute(name string) *Route {
	return r.namedRoutes[name]
}

// StrictSlash defines the trailing slash behavior for new routes. The initial
// value is false.
//
// When true, if the route path is "/path/", accessing "/path" will perform a redirect
// to the former and vice versa. In other words, your application will always
// see the path as specified in the route.
//
// When false, if the route path is "/path", accessing "/path/" will not match
// this route and vice versa.
//
// The re-direct is a HTTP 301 (Moved Permanently). Note that when this is set for
// routes with a non-idempotent method (e.g. POST, PUT), the subsequent re-directed
// request will be made as a GET by most clients. Use middleware or client settings
// to modify this behaviour as needed.
//
// Special case: when a route sets a path prefix using the PathPrefix() method,
// strict slash is ignored for that route because the redirect behavior can't
// be determined from a prefix alone. However, any subrouters created from that
// route inherit the original StrictSlash setting.
func (r *Router) StrictSlash(value bool) *Router {
	r.strictSlash = value
	return r
}

// SkipClean defines the path cleaning behaviour for new routes. The initial
// value is false. Users should be careful about which routes are not cleaned
//
// When true, if the route path is "/path//to", it will remain with the double
// slash. This is helpful if you have a route like: /fetch/http://xkcd.com/534/
//
// When false, the path will be cleaned, so /fetch/http://xkcd.com/534/ will
// become /fetch/http/xkcd.com/534
func (r *Router) SkipClean(value bool) *Router {
	r.skipClean = value
	return r
}

// UseEncodedPath tells the router to match the encoded original path
// to the routes.
// For eg. "/path/foo%2Fbar/to" will match the path "/path/{var}/to".
//
// If not called, the router will match the unencoded path to the routes.
// For eg. "/path/foo%2Fbar/to" will match the path "/path/foo/bar/to"
func (r *Router) UseEncodedPath() *Router {
	r.useEncodedPath = true
	return r
}

// ----------------------------------------------------------------------------
// Route factories
// ----------------------------------------------------------------------------

// NewRoute registers an empty route.
func (r *Router) NewRoute() *Route {
    { … 5 line(s) … ⟦tj:4b1d291e326539377b038d241472c261⟧ }

// Name registers a new route with a name.
// See Route.Name().
func (r *Router) Name(name string) *Route {
	return r.NewRoute().Name(name)
}

// Handle registers a new route with a matcher for the URL path.
// See Route.Path() and Route.Handler().
func (r *Router) Handle(path string, handler http.Handler) *Route {
	return r.NewRoute().Path(path).Handler(handler)
}

// HandleFunc registers a new route with a matcher for the URL path.
// See Route.Path() and Route.HandlerFunc().
func (r *Router) HandleFunc(path string, f func(http.ResponseWriter,
	*http.Request)) *Route {
	return r.NewRoute().Path(path).HandlerFunc(f)
}

// Headers registers a new route with a matcher for request header values.
// See Route.Headers().
func (r *Router) Headers(pairs ...string) *Route {
	return r.NewRoute().Headers(pairs...)
}

// Host registers a new route with a matcher for the URL host.
// See Route.Host().
func (r *Router) Host(tpl string) *Route {
	return r.NewRoute().Host(tpl)
}

// MatcherFunc registers a new route with a custom matcher function.
// See Route.MatcherFunc().
func (r *Router) MatcherFunc(f MatcherFunc) *Route {
	return r.NewRoute().MatcherFunc(f)
}

// Methods registers a new route with a matcher for HTTP methods.
// See Route.Methods().
func (r *Router) Methods(methods ...string) *Route {
	return r.NewRoute().Methods(methods...)
}

// Path registers a new route with a matcher for the URL path.
// See Route.Path().
func (r *Router) Path(tpl string) *Route {
	return r.NewRoute().Path(tpl)
}

// PathPrefix registers a new route with a matcher for the URL path prefix.
// See Route.PathPrefix().
func (r *Router) PathPrefix(tpl string) *Route {
	return r.NewRoute().PathPrefix(tpl)
}

// Queries registers a new route with a matcher for URL query values.
// See Route.Queries().
func (r *Router) Queries(pairs ...string) *Route {
	return r.NewRoute().Queries(pairs...)
}

// Schemes registers a new route with a matcher for URL schemes.
// See Route.Schemes().
func (r *Router) Schemes(schemes ...string) *Route {
	return r.NewRoute().Schemes(schemes...)
}

// BuildVarsFunc registers a new route with a custom function for modifying
// route variables before building a URL.
func (r *Router) BuildVarsFunc(f BuildVarsFunc) *Route {
	return r.NewRoute().BuildVarsFunc(f)
}

// Walk walks the router and all its sub-routers, calling walkFn for each route
// in the tree. The routes are walked in the order they were added. Sub-routers
// are explored depth-first.
func (r *Router) Walk(walkFn WalkFunc) error {
	return r.walk(walkFn, []*Route{})
}

// SkipRouter is used as a return value from WalkFuncs to indicate that the
// router that walk is about to descend down to should be skipped.
var SkipRouter = errors.New("skip this router")

// WalkFunc is the type of the function called for each route visited by Walk.
// At every invocation, it is given the current route, and the current router,
// and a list of ancestor routes that lead to the current route.
type WalkFunc func(route *Route, router *Router, ancestors []*Route) error

func (r *Router) walk(walkFn WalkFunc, ancestors []*Route) error {
    { … 29 line(s) … ⟦tj:6242c0dc6fcc7dfa8d633067c2ddbfd3⟧ }

// ----------------------------------------------------------------------------
// Context
// ----------------------------------------------------------------------------

// RouteMatch stores information about a matched route.
type RouteMatch struct {
    { … 4 line(s) … ⟦tj:2c4f142e3900d6321f3fb1973f8b7a88⟧ }
	// MatchErr is set to appropriate matching error
	// It is set to ErrMethodMismatch if there is a mismatch in
	// the request method and route method
	MatchErr error
}

type contextKey int

const (
	varsKey contextKey = iota
	routeKey
)

// Vars returns the route variables for the current request, if any.
func Vars(r *http.Request) map[string]string {
    { … 5 line(s) … ⟦tj:b2e7cdb99549fa0f5e2ee6669f4cd5b5⟧ }

// CurrentRoute returns the matched route for the current request, if any.
// This only works when called inside the handler of the matched route
// because the matched route is stored in the request context which is cleared
// after the handler returns.
func CurrentRoute(r *http.Request) *Route {
    { … 5 line(s) … ⟦tj:98ed2f0d576afabe2e6856f6f61e9321⟧ }

func requestWithVars(r *http.Request, vars map[string]string) *http.Request {
	ctx := context.WithValue(r.Context(), varsKey, vars)
	return r.WithContext(ctx)
}

func requestWithRoute(r *http.Request, route *Route) *http.Request {
	ctx := context.WithValue(r.Context(), routeKey, route)
	return r.WithContext(ctx)
}

// ----------------------------------------------------------------------------
// Helpers
// ----------------------------------------------------------------------------

// cleanPath returns the canonical path for p, eliminating . and .. elements.
// Borrowed from the net/http package.
func cleanPath(p string) string {
    { … 15 line(s) … ⟦tj:40d3f9c3a7794c6647bf85f4d75133e6⟧ }

// uniqueVars returns an error if two slices contain duplicated strings.
func uniqueVars(s1, s2 []string) error {
    { … 9 line(s) … ⟦tj:e23c341eeadd7e2bb0273fb0b2cd0be9⟧ }

// checkPairs returns the count of strings passed in, and an error if
// the count is not an even number.
func checkPairs(pairs ...string) (int, error) {
    { … 7 line(s) … ⟦tj:47c3d1eae5d947c4dc365368d67859fd⟧ }

// mapFromPairsToString converts variadic string parameters to a
// string to string map.
func mapFromPairsToString(pairs ...string) (map[string]string, error) {
    { … 10 line(s) … ⟦tj:0230a312f475ccef019cf506463ef33f⟧ }

// mapFromPairsToRegex converts variadic string parameters to a
// string to regex map.
func mapFromPairsToRegex(pairs ...string) (map[string]*regexp.Regexp, error) {
    { … 14 line(s) … ⟦tj:14be423299c7f2b1dc9d5ffdde034d8b⟧ }

// matchInArray returns true if the given string value is in the array.
func matchInArray(arr []string, value string) bool {
    { … 7 line(s) … ⟦tj:e71cee35ef38ea1f66e2353632bdd35e⟧ }

// matchMapWithString returns true if the given key/value pairs exist in a given map.
func matchMapWithString(toCheck map[string]string, toMatch map[string][]string, canonicalKey bool) bool {
    { … 24 line(s) … ⟦tj:3e44af30cfcbaabe9bba1d7787ea7658⟧ }

// matchMapWithRegex returns true if the given key/value pairs exist in a given map compiled against
// the given regex
func matchMapWithRegex(toCheck map[string]*regexp.Regexp, toMatch map[string][]string, canonicalKey bool) bool {
    { … 24 line(s) … ⟦tj:961df635ace7da6da758478f34ab55ba⟧ }

// methodNotAllowed replies to the request with an HTTP status code 405.
func methodNotAllowed(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusMethodNotAllowed)
}

// methodNotAllowedHandler returns a simple request handler
// that replies to each request with a status code 405.
func methodNotAllowedHandler() http.Handler { return http.HandlerFunc(methodNotAllowed) }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (17782 bytes) is available by calling tinyjuice_retrieve with token "9fe967437d9bc37d9dfa8e3e7f742bd9" (marker ⟦tj:9fe967437d9bc37d9dfa8e3e7f742bd9⟧)]