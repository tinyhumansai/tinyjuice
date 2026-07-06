/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { CharCode } from './charCode.js';
import { MarshalledId } from './marshallingIds.js';
import * as paths from './path.js';
import { isWindows } from './platform.js';

const _schemePattern = /^\w[\w\d+.-]*$/;
const _singleSlashStart = /^\//;
const _doubleSlashStart = /^\/\//;

function _validateUri(ret: URI, _strict?: boolean): void {

	// scheme, must be set
	if (!ret.scheme && _strict) {
		throw new Error(`[UriError]: Scheme is missing: {scheme: "", authority: "${ret.authority}", path: "${ret.path}", query: "${ret.query}", fragment: "${ret.fragment}"}`);
	}

	// scheme, https://tools.ietf.org/html/rfc3986#section-3.1
	// ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
	if (ret.scheme && !_schemePattern.test(ret.scheme)) {
		throw new Error('[UriError]: Scheme contains illegal characters.');
	}

	// path, http://tools.ietf.org/html/rfc3986#section-3.3
	// If a URI contains an authority component, then the path component
	// must either be empty or begin with a slash ("/") character.  If a URI
	// does not contain an authority component, then the path cannot begin
	// with two slash characters ("//").
	if (ret.path) {
		if (ret.authority) {
			if (!_singleSlashStart.test(ret.path)) {
				throw new Error('[UriError]: If a URI contains an authority component, then the path component must either be empty or begin with a slash ("/") character');
			}
		} else {
			if (_doubleSlashStart.test(ret.path)) {
				throw new Error('[UriError]: If a URI does not contain an authority component, then the path cannot begin with two slash characters ("//")');
			}
		}
	}
}

// for a while we allowed uris *without* schemes and this is the migration
// for them, e.g. an uri without scheme and without strict-mode warns and falls
// back to the file-scheme. that should cause the least carnage and still be a
// clear warning
function _schemeFix(scheme: string, _strict: boolean): string {
	if (!scheme && !_strict) {
		return 'file';
	}
	return scheme;
}

// implements a bit of https://tools.ietf.org/html/rfc3986#section-5
function _referenceResolution(scheme: string, path: string): string {

	// the slash-character is our 'default base' as we don't
{ … 14 line(s) … ⟦tj:5464103aa9f50450b3236344d6db8e4e⟧ }
	return path;
}

const _empty = '';
const _slash = '/';
const _regexp = /^(([^:/?#]+?):)?(\/\/([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?/;

/**
 * Uniform Resource Identifier (URI) http://tools.ietf.org/html/rfc3986.
 * This class is a simple parser which creates the basic component parts
 * (http://tools.ietf.org/html/rfc3986#section-3) with minimal validation
 * and encoding.
 *
 * ```txt
 *       foo://example.com:8042/over/there?name=ferret#nose
 *       \_/   \______________/\_________/ \_________/ \__/
 *        |           |            |            |        |
 *     scheme     authority       path        query   fragment
 *        |   _____________________|__
 *       / \ /                        \
 *       urn:example:animal:ferret:nose
 * ```
 */
export class URI implements UriComponents {

	static isUri(thing: any): thing is URI {
		if (thing instanceof URI) {
			return true;
		{ … 11 line(s) … ⟦tj:1c5429c9a1b270b0e13830f228d49723⟧ }
			&& typeof (<URI>thing).toString === 'function';
}

	/**
	 * scheme is the 'http' part of 'http://www.example.com/some/path?query#fragment'.
	 * The part before the first colon.
	 */
	readonly scheme: string;

	/**
	 * authority is the 'www.example.com' part of 'http://www.example.com/some/path?query#fragment'.
	 * The part between the first double slashes and the next slash.
	 */
	readonly authority: string;

	/**
	 * path is the '/some/path' part of 'http://www.example.com/some/path?query#fragment'.
	 */
	readonly path: string;

	/**
	 * query is the 'query' part of 'http://www.example.com/some/path?query#fragment'.
	 */
	readonly query: string;

	/**
	 * fragment is the 'fragment' part of 'http://www.example.com/some/path?query#fragment'.
	 */
	readonly fragment: string;

	/**
	 * @internal
	 */
	protected constructor(scheme: string, authority?: string, path?: string, query?: string, fragment?: string, _strict?: boolean);

	/**
	 * @internal
	 */
	protected constructor(components: UriComponents);

	/**
	 * @internal
	 */
	protected constructor(schemeOrData: string | UriComponents, authority?: string, path?: string, query?: string, fragment?: string, _strict: boolean = false) {

		if (typeof schemeOrData === 'object') {
			this.scheme = schemeOrData.scheme || _empty;
			this.authority = schemeOrData.authority || _empty;
			this.path = schemeOrData.path || _empty;
			this.query = schemeOrData.query || _empty;
			this.fragment = schemeOrData.fragment || _empty;
			// no validation because it's this URI
			// that creates uri components.
			// _validateUri(this);
		} else {
			this.scheme = _schemeFix(schemeOrData, _strict);
			this.authority = authority || _empty;
			this.path = _referenceResolution(this.scheme, path || _empty);
			this.query = query || _empty;
			this.fragment = fragment || _empty;

			_validateUri(this, _strict);
		}
	}

	// ---- filesystem path -----------------------

	/**
	 * Returns a string representing the corresponding file system path of this URI.
	 * Will handle UNC paths, normalizes windows drive letters to lower-case, and uses the
	 * platform specific path separator.
	 *
	 * * Will *not* validate the path for invalid characters and semantics.
	 * * Will *not* look at the scheme of this URI.
	 * * The result shall *not* be used for display purposes but for accessing a file on disk.
	 *
	 *
	 * The *difference* to `URI#path` is the use of the platform specific separator and the handling
	 * of UNC paths. See the below sample of a file-uri with an authority (UNC path).
	 *
	 * ```ts
		const u = URI.parse('file://server/c$/folder/file.txt')
		u.authority === 'server'
		u.path === '/shares/c$/file.txt'
		u.fsPath === '\\server\c$\folder\file.txt'
	```
	 *
	 * Using `URI#path` to read a file (using fs-apis) would not be enough because parts of the path,
	 * namely the server name, would be missing. Therefore `URI#fsPath` exists - it's sugar to ease working
	 * with URIs that represent files on disk (`file` scheme).
	 */
	get fsPath(): string {
		// if (this.scheme !== 'file') {
		// 	console.warn(`[UriError] calling fsPath with scheme ${this.scheme}`);
		// }
		return uriToFsPath(this, false);
	}

	// ---- modify to new -------------------------

	with(change: { scheme?: string; authority?: string | null; path?: string | null; query?: string | null; fragment?: string | null }): URI {

		if (!change) {
{ … 39 line(s) … ⟦tj:212723d05e5d1990128ee33d5201f061⟧ }
		return new Uri(scheme, authority, path, query, fragment);
}

	// ---- parse & validate ------------------------

	/**
	 * Creates a new URI from a string, e.g. `http://www.example.com/some/path`,
	 * `file:///usr/home`, or `scheme:with/path`.
	 *
	 * @param value A string which represents an URI (see `URI#toString`).
	 */
	static parse(value: string, _strict: boolean = false): URI {
		const match = _regexp.exec(value);
		if (!match) {
		{ … 9 line(s) … ⟦tj:187ea2dc3fd36c99b3cc8df3c1f78448⟧ }
		);
}

	/**
	 * Creates a new URI from a file system path, e.g. `c:\my\files`,
	 * `/usr/home`, or `\\server\share\some\path`.
	 *
	 * The *difference* between `URI#parse` and `URI#file` is that the latter treats the argument
	 * as path, not as stringified-uri. E.g. `URI.file(path)` is **not the same as**
	 * `URI.parse('file://' + path)` because the path might contain characters that are
	 * interpreted (# and ?). See the following sample:
	 * ```ts
	const good = URI.file('/coding/c#/project1');
	good.scheme === 'file';
	good.path === '/coding/c#/project1';
	good.fragment === '';
	const bad = URI.parse('file://' + '/coding/c#/project1');
	bad.scheme === 'file';
	bad.path === '/coding/c'; // path is now broken
	bad.fragment === '/project1';
	```
	 *
	 * @param path A file system path (see `URI#fsPath`)
	 */
	static file(path: string): URI {

		let authority = _empty;
{ … 21 line(s) … ⟦tj:c9082ca90142cafbdd31326e10d450f5⟧ }
		return new Uri('file', authority, path, _empty, _empty);
}

	/**
	 * Creates new URI from uri components.
	 *
	 * Unless `strict` is `true` the scheme is defaults to be `file`. This function performs
	 * validation and should be used for untrusted uri components retrieved from storage,
	 * user input, command arguments etc
	 */
	static from(components: UriComponents, strict?: boolean): URI { … 11 line(s) … ⟦tj:dee873aec4631e007eae19e14b9b170f⟧ }

	/**
	 * Join a URI path with path fragments and normalizes the resulting path.
	 *
	 * @param uri The input URI.
	 * @param pathFragment The path fragment to add to the URI path.
	 * @returns The resulting URI.
	 */
	static joinPath(uri: URI, ...pathFragment: string[]): URI { … 12 line(s) … ⟦tj:61b9fa3fb681e1d82a759eeb62ed2996⟧ }

	// ---- printing/externalize ---------------------------

	/**
	 * Creates a string representation for this URI. It's guaranteed that calling
	 * `URI.parse` with the result of this function creates an URI which is equal
	 * to this URI.
	 *
	 * * The result shall *not* be used for display purposes but for externalization or transport.
	 * * The result will be encoded using the percentage encoding and encoding happens mostly
	 * ignore the scheme-specific encoding rules.
	 *
	 * @param skipEncoding Do not encode the result, default is `false`
	 */
	toString(skipEncoding: boolean = false): string {
		return _asFormatted(this, skipEncoding);
	}

	toJSON(): UriComponents {
		return this;
	}

	/**
	 * A helper function to revive URIs.
	 *
	 * **Note** that this function should only be used when receiving URI#toJSON generated data
	 * and that it doesn't do any validation. Use {@link URI.from} when received "untrusted"
	 * uri components such as command arguments or data from storage.
	 *
	 * @param data The URI components or URI to revive.
	 * @returns The revived URI or undefined or null.
	 */
	static revive(data: UriComponents | URI): URI;
	static revive(data: UriComponents | URI | undefined): URI | undefined;
	static revive(data: UriComponents | URI | null): URI | null;
	static revive(data: UriComponents | URI | undefined | null): URI | undefined | null;
	static revive(data: UriComponents | URI | undefined | null): URI | undefined | null { … 12 line(s) … ⟦tj:59e69e3742912e28dc7c6fbd6417ec47⟧ }

	[Symbol.for('debug.description')]() {
		return `URI(${this.toString()})`;
	}
}

export interface UriComponents {
	scheme: string;
	authority?: string;
	path?: string;
	query?: string;
	fragment?: string;
}

export function isUriComponents(thing: any): thing is UriComponents { … 10 line(s) … ⟦tj:d85bd96bc51dc4877b8061060aa4c309⟧ }

interface UriState extends UriComponents {
	$mid: MarshalledId.Uri;
	external?: string;
	fsPath?: string;
	_sep?: 1;
}

const _pathSepMarker = isWindows ? 1 : undefined;

// This class exists so that URI is compatible with vscode.Uri (API).
class Uri extends URI {

	_formatted: string | null = null;
	_fsPath: string | null = null;

	override get fsPath(): string {
		if (!this._fsPath) {
			this._fsPath = uriToFsPath(this, false);
		}
		return this._fsPath;
	}

	override toString(skipEncoding: boolean = false): string { … 11 line(s) … ⟦tj:d74be94362da385e328912d10faafafb⟧ }

	override toJSON(): UriComponents {
		// eslint-disable-next-line local/code-no-dangerous-type-assertions
		const res = <UriState>{
		{ … 30 line(s) … ⟦tj:c62499753706595e0d7002428baab8fa⟧ }
		return res;
}
}

// reserved characters: https://tools.ietf.org/html/rfc3986#section-2.2
const encodeTable: { [ch: number]: string } = {
	[CharCode.Colon]: '%3A', // gen-delims
	[CharCode.Slash]: '%2F',
	[CharCode.QuestionMark]: '%3F',
	[CharCode.Hash]: '%23',
	[CharCode.OpenSquareBracket]: '%5B',
	[CharCode.CloseSquareBracket]: '%5D',
	[CharCode.AtSign]: '%40',

	[CharCode.ExclamationMark]: '%21', // sub-delims
	[CharCode.DollarSign]: '%24',
	[CharCode.Ampersand]: '%26',
	[CharCode.SingleQuote]: '%27',
	[CharCode.OpenParen]: '%28',
	[CharCode.CloseParen]: '%29',
	[CharCode.Asterisk]: '%2A',
	[CharCode.Plus]: '%2B',
	[CharCode.Comma]: '%2C',
	[CharCode.Semicolon]: '%3B',
	[CharCode.Equals]: '%3D',

	[CharCode.Space]: '%20',
};

function encodeURIComponentFast(uriComponent: string, isPath: boolean, isAuthority: boolean): string {
	let res: string | undefined = undefined;
	let nativeEncodePos = -1;
	{ … 58 line(s) … ⟦tj:4ab6816d1f5e5ec18d5fc3ff9b487604⟧ }
	return res !== undefined ? res : uriComponent;
}

function encodeURIComponentMinimal(path: string): string {
	let res: string | undefined = undefined;
	for (let pos = 0; pos < path.length; pos++) {
	{ … 12 line(s) … ⟦tj:aec32af8b1e057b4e7976f6a3a4bd18e⟧ }
	return res !== undefined ? res : path;
}

/**
 * Compute `fsPath` for the given uri
 */
export function uriToFsPath(uri: URI, keepDriveLetterCasing: boolean): string {

	let value: string;
{ … 21 line(s) … ⟦tj:cf4dcabef15b1c3e4e2da59a4ada3f87⟧ }
	return value;
}

/**
 * Create the external version of a uri
 */
function _asFormatted(uri: URI, skipEncoding: boolean): string {

	const encoder = !skipEncoding
{ … 64 line(s) … ⟦tj:17b293d3fadb7801cd5e5f6d9764de2b⟧ }
	return res;
}

// --- decode

function decodeURIComponentGraceful(str: string): string { … 11 line(s) … ⟦tj:017c7ac3071060da44d7af25f760ac69⟧ }

const _rEncodedAsHex = /(%[0-9A-Za-z][0-9A-Za-z])+/g;

function percentDecode(str: string): string {
	if (!str.match(_rEncodedAsHex)) {
		return str;
	}
	return str.replace(_rEncodedAsHex, (match) => decodeURIComponentGraceful(match));
}

/**
 * Mapped-type that replaces all occurrences of URI with UriComponents
 */
export type UriDto<T> = { [K in keyof T]: T[K] extends URI
	? UriComponents
	: UriDto<T[K]> };
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[PARTIAL view — full original (22643 bytes): call tinyjuice_retrieve with token "4369ae3ad11dddac2f76c89b6e73d33c"]