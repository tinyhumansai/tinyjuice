/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { LRUCachedFunction } from './cache.js';
import { CharCode } from './charCode.js';
import { Lazy } from './lazy.js';
import { Constants } from './uint.js';

export function isFalsyOrWhitespace(str: string | undefined): boolean { … 6 line(s) … ⟦tj:b2944d203ad472cb4e1c7d5d6f5fbc6c⟧ }

const _formatRegexp = /{(\d+)}/g;

/**
 * Helper to produce a string with a variable number of arguments. Insert variable segments
 * into the string using the {n} notation where N is the index of the argument following the string.
 * @param value string to which formatting is applied
 * @param args replacements for {n}-entries
 */
export function format(value: string, ...args: any[]): string { … 11 line(s) … ⟦tj:870557e346c33eff08954ba172e68750⟧ }

const _format2Regexp = /{([^}]+)}/g;

/**
 * Helper to create a string from a template and a string record.
 * Similar to `format` but with objects instead of positional arguments.
 */
export function format2(template: string, values: Record<string, unknown>): string { … 6 line(s) … ⟦tj:d4a4380e8b88957cd07416c86e77b525⟧ }

/**
 * Encodes the given value so that it can be used as literal value in html attributes.
 *
 * In other words, computes `$val`, such that `attr` in `<div attr="$val" />` has the runtime value `value`.
 * This prevents XSS injection.
 */
export function htmlAttributeEncodeValue(value: string): string { … 12 line(s) … ⟦tj:0982072a15a096150d4ded8118db0920⟧ }

/**
 * Converts HTML characters inside the string to use entities instead. Makes the string safe from
 * being used e.g. in HTMLElement.innerHTML.
 */
export function escape(html: string): string { … 10 line(s) … ⟦tj:a9a10288a6bb3b81c3a4ad90a0f0604c⟧ }

/**
 * Escapes regular expression characters in a given string
 */
export function escapeRegExpCharacters(value: string): string {
	return value.replace(/[\\\{\}\*\+\?\|\^\$\.\[\]\(\)]/g, '\\$&');
}

/**
 * Counts how often `substr` occurs inside `value`.
 */
export function count(value: string, substr: string): number { … 9 line(s) … ⟦tj:3584c02b3e0a3d8e4e1d16800f6f7bae⟧ }

export function truncate(value: string, maxLength: number, suffix = '…'): string { … 7 line(s) … ⟦tj:e6664b41d933f31346130876bda08b53⟧ }

export function truncateMiddle(value: string, maxLength: number, suffix = '…'): string { … 10 line(s) … ⟦tj:1f9459ca7d5677d8c301bcb149a91b2b⟧ }

/**
 * Removes all occurrences of needle from the beginning and end of haystack.
 * @param haystack string to trim
 * @param needle the thing to trim (default is a blank)
 */
export function trim(haystack: string, needle: string = ' '): string { … 4 line(s) … ⟦tj:8e3126f6f389a1e533112114f737db34⟧ }

/**
 * Removes all occurrences of needle from the beginning of haystack.
 * @param haystack string to trim
 * @param needle the thing to trim
 */
export function ltrim(haystack: string, needle: string): string { … 17 line(s) … ⟦tj:6c04225d2962caa3e2f4f238b555f2c2⟧ }

/**
 * Removes all occurrences of needle from the end of haystack.
 * @param haystack string to trim
 * @param needle the thing to trim
 */
export function rtrim(haystack: string, needle: string): string { … 28 line(s) … ⟦tj:f906a59dc03564798a1e43fbcabc5295⟧ }

export function convertSimple2RegExpPattern(pattern: string): string {
	return pattern.replace(/[\-\\\{\}\+\?\|\^\$\.\,\[\]\(\)\#\s]/g, '\\$&').replace(/[\*]/g, '.*');
}

export function stripWildcards(pattern: string): string {
	return pattern.replace(/\*/g, '');
}

export interface RegExpOptions {
	matchCase?: boolean;
	wholeWord?: boolean;
	multiline?: boolean;
	global?: boolean;
	unicode?: boolean;
}

export function createRegExp(searchString: string, isRegex: boolean, options: RegExpOptions = {}): RegExp { … 31 line(s) … ⟦tj:ba31bd62edf2046dc41d4c5aaa6e6569⟧ }

export function regExpLeadsToEndlessLoop(regexp: RegExp): boolean { … 12 line(s) … ⟦tj:c0c427b5c9f543234c51ef84d4dc5ca5⟧ }

export function joinStrings(items: (string | undefined | null | false)[], separator: string): string {
	return items.filter(item => item !== undefined && item !== null && item !== false).join(separator);
}

export function splitLines(str: string): string[] {
	return str.split(/\r\n|\r|\n/);
}

export function splitLinesIncludeSeparators(str: string): string[] { … 8 line(s) … ⟦tj:d32af777b8f9b0e8928be91b78bbb5f4⟧ }

/**
 * Returns first index of the string that is not whitespace.
 * If string is empty or contains only whitespaces, returns -1
 */
export function firstNonWhitespaceIndex(str: string): number { … 9 line(s) … ⟦tj:615452a1addd14b680a8407c73d632df⟧ }

/**
 * Returns the leading whitespace of the string.
 * If the string contains only whitespaces, returns entire string
 */
export function getLeadingWhitespace(str: string, start: number = 0, end: number = str.length): string { … 9 line(s) … ⟦tj:59e4831a24308ede8063cb5f1ade0a73⟧ }

/**
 * Returns last index of the string that is not whitespace.
 * If string is empty or contains only whitespaces, returns -1
 */
export function lastNonWhitespaceIndex(str: string, startIndex: number = str.length - 1): number { … 9 line(s) … ⟦tj:55888c87e98ee2f9f58317db1f827771⟧ }

export function getIndentationLength(str: string): number { … 5 line(s) … ⟦tj:e88e02c6815b5edf9b58e262e77ae4fb⟧ }

/**
 * Function that works identically to String.prototype.replace, except, the
 * replace function is allowed to be async and return a Promise.
 */
export function replaceAsync(str: string, search: RegExp, replacer: (match: string, ...args: any[]) => Promise<string>): Promise<string> { … 18 line(s) … ⟦tj:9872a004e1c9be8241b5cebcbc838979⟧ }

export function compare(a: string, b: string): number { … 9 line(s) … ⟦tj:5e228c3845f39bfec635b47fe19d60e3⟧ }

export function compareSubstring(a: string, b: string, aStart: number = 0, aEnd: number = a.length, bStart: number = 0, bEnd: number = b.length): number { … 19 line(s) … ⟦tj:3981c5355f6a5b2997afccc41c121c73⟧ }

export function compareIgnoreCase(a: string, b: string): number {
	return compareSubstringIgnoreCase(a, b, 0, a.length, 0, b.length);
}

export function compareSubstringIgnoreCase(a: string, b: string, aStart: number = 0, aEnd: number = a.length, bStart: number = 0, bEnd: number = b.length): number { … 46 line(s) … ⟦tj:ffcfce8764b6ce93bde8610fdf542afc⟧ }

export function isAsciiDigit(code: number): boolean {
	return code >= CharCode.Digit0 && code <= CharCode.Digit9;
}

export function isLowerAsciiLetter(code: number): boolean {
	return code >= CharCode.a && code <= CharCode.z;
}

export function isUpperAsciiLetter(code: number): boolean {
	return code >= CharCode.A && code <= CharCode.Z;
}

export function equalsIgnoreCase(a: string, b: string): boolean {
	return a.length === b.length && compareSubstringIgnoreCase(a, b) === 0;
}

export function startsWithIgnoreCase(str: string, candidate: string): boolean { … 8 line(s) … ⟦tj:0b32385bb96c28d40a45fe3ce0688f6b⟧ }

/**
 * @returns the length of the common prefix of the two strings.
 */
export function commonPrefixLength(a: string, b: string): number { … 13 line(s) … ⟦tj:7ad543f1b6d25aa8a30cbeead1aa2738⟧ }

/**
 * @returns the length of the common suffix of the two strings.
 */
export function commonSuffixLength(a: string, b: string): number { … 16 line(s) … ⟦tj:b2ed04748bc14dbb23cf60394bb53ab5⟧ }

/**
 * See http://en.wikipedia.org/wiki/Surrogate_pair
 */
export function isHighSurrogate(charCode: number): boolean {
	return (0xD800 <= charCode && charCode <= 0xDBFF);
}

/**
 * See http://en.wikipedia.org/wiki/Surrogate_pair
 */
export function isLowSurrogate(charCode: number): boolean {
	return (0xDC00 <= charCode && charCode <= 0xDFFF);
}

/**
 * See http://en.wikipedia.org/wiki/Surrogate_pair
 */
export function computeCodePoint(highSurrogate: number, lowSurrogate: number): number {
	return ((highSurrogate - 0xD800) << 10) + (lowSurrogate - 0xDC00) + 0x10000;
}

/**
 * get the code point that begins at offset `offset`
 */
export function getNextCodePoint(str: string, len: number, offset: number): number { … 10 line(s) … ⟦tj:a1832651628290d77e630079a0ddd3fe⟧ }

/**
 * get the code point that ends right before offset `offset`
 */
function getPrevCodePoint(str: string, offset: number): number { … 10 line(s) … ⟦tj:a35bbd88f63be6c2d091e0e3d1d505dc⟧ }

export class CodePointIterator {

	private readonly _str: string;
	private readonly _len: number;
	private _offset: number;

	public get offset(): number {
		return this._offset;
	}

	constructor(str: string, offset: number = 0) { … 5 line(s) … ⟦tj:cd292dfc48b35bb510265c82ceedb606⟧ }

	public setOffset(offset: number): void {
		this._offset = offset;
	}

	public prevCodePoint(): number { … 5 line(s) … ⟦tj:3c7a2cb4f354431db63c111d3b583f05⟧ }

	public nextCodePoint(): number { … 5 line(s) … ⟦tj:18434d5e3ad0a1a7f8c4b996041a54da⟧ }

	public eol(): boolean {
		return (this._offset >= this._len);
	}
}

export class GraphemeIterator {

	private readonly _iterator: CodePointIterator;

	public get offset(): number {
		return this._iterator.offset;
	}

	constructor(str: string, offset: number = 0) {
		this._iterator = new CodePointIterator(str, offset);
	}

	public nextGraphemeLength(): number { … 18 line(s) … ⟦tj:b05e8faeb6b0e9605aaad912890fa0d2⟧ }

	public prevGraphemeLength(): number { … 18 line(s) … ⟦tj:8a7928fb13b417790a5763b86e0b8009⟧ }

	public eol(): boolean {
		return this._iterator.eol();
	}
}

export function nextCharLength(str: string, initialOffset: number): number { … 4 line(s) … ⟦tj:bce56aeb96fa1f9aa992486750838e76⟧ }

export function prevCharLength(str: string, initialOffset: number): number { … 4 line(s) … ⟦tj:6b30bb587708b4718ea51e12b90712c9⟧ }

export function getCharContainingOffset(str: string, offset: number): [number, number] { … 8 line(s) … ⟦tj:caabfee39f77a255a897de9ef40f8770⟧ }

export function charCount(str: string): number { … 9 line(s) … ⟦tj:91ae1986621fdcf0516016d2a097fc40⟧ }

let CONTAINS_RTL: RegExp | undefined = undefined;

function makeContainsRtl() { … 4 line(s) … ⟦tj:31d2180dbe956a1d59a094af2f735b5e⟧ }

/**
 * Returns true if `str` contains any Unicode character that is classified as "R" or "AL".
 */
export function containsRTL(str: string): boolean { … 7 line(s) … ⟦tj:74507b5704bf003cf36ad687f4277dd3⟧ }

const IS_BASIC_ASCII = /^[\t\n\r\x20-\x7E]*$/;
/**
 * Returns true if `str` contains only basic ASCII characters in the range 32 - 126 (including 32 and 126) or \n, \r, \t
 */
export function isBasicASCII(str: string): boolean {
	return IS_BASIC_ASCII.test(str);
}

export const UNUSUAL_LINE_TERMINATORS = /[\u2028\u2029]/; // LINE SEPARATOR (LS) or PARAGRAPH SEPARATOR (PS)
/**
 * Returns true if `str` contains unusual line terminators, like LS or PS
 */
export function containsUnusualLineTerminators(str: string): boolean {
	return UNUSUAL_LINE_TERMINATORS.test(str);
}

export function isFullWidthCharacter(charCode: number): boolean { … 45 line(s) … ⟦tj:ca705a15806d018d1a5d5d16af61c2df⟧ }

/**
 * A fast function (therefore imprecise) to check if code points are emojis.
 * Generated using https://github.com/alexdima/unicode-utils/blob/main/emoji-test.js
 */
export function isEmojiImprecise(x: number): boolean { … 9 line(s) … ⟦tj:2d64fa4c974015cc277195717edf6bed⟧ }

/**
 * Given a string and a max length returns a shorted version. Shorting
 * happens at favorable positions - such as whitespace or punctuation characters.
 * The return value can be longer than the given value of `n`. Leading whitespace is always trimmed.
 */
export function lcut(text: string, n: number, prefix = '') { … 24 line(s) … ⟦tj:7fcc144946acd55053b86a20d05e98fb⟧ }

// Escape codes, compiled from https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Functions-using-CSI-_-ordered-by-the-final-character_s_
// Plus additional markers for custom `\x1b]...\x07` instructions.
const CSI_SEQUENCE = /(?:(?:\x1b\[|\x9B)[=?>!]?[\d;:]*["$#'* ]?[a-zA-Z@^`{}|~])|(:?\x1b\].*?\x07)/g;

/** Iterates over parts of a string with CSI sequences */
export function* forAnsiStringParts(str: string) { … 15 line(s) … ⟦tj:d380cb08115578f615aff53713958e1f⟧ }

/**
 * Strips ANSI escape sequences from a string.
 * @param str The dastringa stringo strip the ANSI escape sequences from.
 *
 * @example
 * removeAnsiEscapeCodes('\u001b[31mHello, World!\u001b[0m');
 * // 'Hello, World!'
 */
export function removeAnsiEscapeCodes(str: string): string { … 7 line(s) … ⟦tj:c451ee4cb754b7a3acef958d5fb203aa⟧ }

const PROMPT_NON_PRINTABLE = /\\\[.*?\\\]/g;

/**
 * Strips ANSI escape sequences from a UNIX-style prompt string (eg. `$PS1`).
 * @param str The string to strip the ANSI escape sequences from.
 *
 * @example
 * removeAnsiEscapeCodesFromPrompt('\n\\[\u001b[01;34m\\]\\w\\[\u001b[00m\\]\n\\[\u001b[1;32m\\]> \\[\u001b[0m\\]');
 * // '\n\\w\n> '
 */
export function removeAnsiEscapeCodesFromPrompt(str: string): string {
	return removeAnsiEscapeCodes(str).replace(PROMPT_NON_PRINTABLE, '');
}


// -- UTF-8 BOM

export const UTF8_BOM_CHARACTER = String.fromCharCode(CharCode.UTF8_BOM);

export function startsWithUTF8BOM(str: string): boolean {
	return !!(str && str.length > 0 && str.charCodeAt(0) === CharCode.UTF8_BOM);
}

export function stripUTF8BOM(str: string): string {
	return startsWithUTF8BOM(str) ? str.substr(1) : str;
}

/**
 * Checks if the characters of the provided query string are included in the
 * target string. The characters do not have to be contiguous within the string.
 */
export function fuzzyContains(target: string, query: string): boolean { … 27 line(s) … ⟦tj:ed89a488a56594b9c8a34cce5ca0f778⟧ }

export function containsUppercaseCharacter(target: string, ignoreEscapedChars = false): boolean { … 11 line(s) … ⟦tj:26f491c2efa1a56e37e1e165b598878d⟧ }

export function uppercaseFirstLetter(str: string): string {
	return str.charAt(0).toUpperCase() + str.slice(1);
}

export function getNLines(str: string, n = 1): string { … 21 line(s) … ⟦tj:84eb5cb41bb9b6b1c0fb554f22bd9181⟧ }

/**
 * Produces 'a'-'z', followed by 'A'-'Z'... followed by 'a'-'z', etc.
 */
export function singleLetterHash(n: number): string { … 11 line(s) … ⟦tj:ca666e40349a1b81d0455b9c37ca0a86⟧ }

//#region Unicode Grapheme Break

export function getGraphemeBreakType(codePoint: number): GraphemeBreakType { … 4 line(s) … ⟦tj:0b7490656f6b7788ce4d60ac1c3e6b15⟧ }

function breakBetweenGraphemeBreakType(breakTypeA: GraphemeBreakType, breakTypeB: GraphemeBreakType): boolean { … 79 line(s) … ⟦tj:4a656f8e6fc84cda1062380177801b8a⟧ }

export const enum GraphemeBreakType {
	Other = 0,
	Prepend = 1,
	CR = 2,
	LF = 3,
	Control = 4,
	Extend = 5,
	Regional_Indicator = 6,
	SpacingMark = 7,
	L = 8,
	V = 9,
	T = 10,
	LV = 11,
	LVT = 12,
	ZWJ = 13,
	Extended_Pictographic = 14
}

class GraphemeBreakTree {

	private static _INSTANCE: GraphemeBreakTree | null = null;
	public static getInstance(): GraphemeBreakTree { … 6 line(s) … ⟦tj:111ab375a787f4d4ade0849f0066677e⟧ }

	private readonly _data: number[];

	constructor() {
		this._data = getGraphemeBreakRawData();
	}

	public getGraphemeBreakType(codePoint: number): GraphemeBreakType { … 34 line(s) … ⟦tj:7d26dd0a95e695956ec6a119e58e9b12⟧ }
}

function getGraphemeBreakRawData(): number[] { … 4 line(s) … ⟦tj:f67cd51e02965497f296819a7d740b08⟧ }

//#endregion

/**
 * Computes the offset after performing a left delete on the given string,
 * while considering unicode grapheme/emoji rules.
*/
export function getLeftDeleteOffset(offset: number, str: string): number { … 16 line(s) … ⟦tj:0ba57d4b8aaf366e66263e58719f5c80⟧ }

function getOffsetBeforeLastEmojiComponent(initialOffset: number, str: string): number | undefined { … 35 line(s) … ⟦tj:69133285877b21d7812c34fac1f37d7f⟧ }

function isEmojiModifier(codePoint: number): boolean {
	return 0x1F3FB <= codePoint && codePoint <= 0x1F3FF;
}

const enum CodePoint {
	zwj = 0x200D,

	/**
	 * Variation Selector-16 (VS16)
	*/
	emojiVariantSelector = 0xFE0F,

	/**
	 * Combining Enclosing Keycap
	 */
	enclosingKeyCap = 0x20E3,
}

export const noBreakWhitespace = '\xa0';

export class AmbiguousCharacters {
	private static readonly ambiguousCharacterData = new Lazy<
		Record<
			string | '_common' | '_default',
			/* code point -> ascii code point */ number[]
		>
	>(() => { … 7 line(s) … ⟦tj:55186d511454c3c0c76b7803c862da76⟧ });

	private static readonly cache = new LRUCachedFunction<
		string[],
		AmbiguousCharacters
	>({ getCacheKey: JSON.stringify }, (locales) => { … 56 line(s) … ⟦tj:969a0c67bd72fedbb8bd07baaf557e95⟧ });

	public static getInstance(locales: Set<string>): AmbiguousCharacters {
		return AmbiguousCharacters.cache.get(Array.from(locales));
	}

	private static _locales = new Lazy<string[]>(() =>
		Object.keys(AmbiguousCharacters.ambiguousCharacterData.value).filter(
			(k) => !k.startsWith('_')
		)
	);
	public static getLocales(): string[] {
		return AmbiguousCharacters._locales.value;
	}

	private constructor(
		private readonly confusableDictionary: Map<number, number>
	) { }

	public isAmbiguous(codePoint: number): boolean {
		return this.confusableDictionary.has(codePoint);
	}

	public containsAmbiguousCharacter(str: string): boolean { … 9 line(s) … ⟦tj:8a2f36f31ff93c9b4adeb3d5a0788694⟧ }

	/**
	 * Returns the non basic ASCII code point that the given code point can be confused,
	 * or undefined if such code point does note exist.
	 */
	public getPrimaryConfusable(codePoint: number): number | undefined {
		return this.confusableDictionary.get(codePoint);
	}

	public getConfusableCodePoints(): ReadonlySet<number> {
		return new Set(this.confusableDictionary.keys());
	}
}

export class InvisibleCharacters {
	private static getRawData(): number[] { … 4 line(s) … ⟦tj:41596da487197d78fbc205697f71cdaa⟧ }

	private static _data: Set<number> | undefined = undefined;

	private static getData() { … 6 line(s) … ⟦tj:9034f230197016fbbdfb1b97a5da555a⟧ }

	public static isInvisibleCharacter(codePoint: number): boolean {
		return InvisibleCharacters.getData().has(codePoint);
	}

	public static containsInvisibleCharacter(str: string): boolean { … 10 line(s) … ⟦tj:23d282f0a7bc7c1a7caf0c4fdd3b305e⟧ }

	public static get codePoints(): ReadonlySet<number> {
		return InvisibleCharacters.getData();
	}
}
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (88368 bytes) is available by calling tinyjuice_retrieve with token "88f2644577e2bf476f49d323d0afa9d7" (marker ⟦tj:88f2644577e2bf476f49d323d0afa9d7⟧)]