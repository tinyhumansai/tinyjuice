//===----------------------------------------------------------*- swift -*-===//
//
// This source file is part of the Swift Argument Parser open source project
//
// Copyright (c) 2020 Apple Inc. and the Swift project authors
// Licensed under Apache License v2.0 with Runtime Library Exception
//
// See https://swift.org/LICENSE.txt for license information
//
//===----------------------------------------------------------------------===//

/// A nested tree of argument definitions.
///
/// The main reason for having a nested representation is to build help output.
/// For output like:
///
///     Usage: mytool [-v | -f] <input> <output>
///
/// The `-v | -f` part is one *set* that’s optional, `<input> <output>` is
/// another. Both of these can then be combined into a third set.
struct ArgumentSet {
    { … 28 line(s) … ⟦tj:00f93739cb88317f53a04778df83f78e⟧ }

extension ArgumentSet: CustomDebugStringConvertible {
    { … 6 line(s) … ⟦tj:f53151b0ea3bb24462a30a4b79767ba6⟧ }

extension ArgumentSet: RandomAccessCollection {
    { … 6 line(s) … ⟦tj:53a7d32be7b2ec75047bc6d77dfe754b⟧ }

// MARK: Flag

extension ArgumentSet {
    { … 97 line(s) … ⟦tj:bb80a6e46f79830bc695e0f2a49a6f36⟧ }

extension ArgumentSet {
    { … 8 line(s) … ⟦tj:0a2225e0a348145639a6bbe4a124dd58⟧ }

extension ArgumentSet {
    { … 21 line(s) … ⟦tj:8c0a5b0cd9d87b4c128cccca965b6c35⟧ }

/// A parser for a given input and set of arguments defined by the given
/// command.
///
/// This parser will consume only the arguments that it understands. If any
/// arguments are declared to capture all remaining input, or a subcommand
/// is configured as such, parsing stops on the first positional argument or
/// unrecognized dash-prefixed argument.
struct LenientParser {
    { … 139 line(s) … ⟦tj:61a037760b0d349a4462ec34eb362d1f⟧ }
      // Fix incorrect error message
    { … 198 line(s) … ⟦tj:dcafeb458e7af70c260c333af5b61ea1⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (21727 bytes) is available by calling tinyjuice_retrieve with token "eb7a0b429ceb76710ec8596d16de2c48" (marker ⟦tj:eb7a0b429ceb76710ec8596d16de2c48⟧)]