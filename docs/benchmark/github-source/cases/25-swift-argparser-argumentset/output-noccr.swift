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
  var content: [ArgumentDefinition] = []
  var namePositions: [Name: Int] = [:]
  
  init<S: Sequence>(_ arguments: S) where S.Element == ArgumentDefinition {
    self.content = Array(arguments)
    self.namePositions = Dictionary(
      content.enumerated().flatMap { i, arg in arg.names.map { ($0.nameToMatch, i) } },
      uniquingKeysWith: { first, _ in first })
  }
  
  init() {}
  
  init(_ arg: ArgumentDefinition) {
    self.init([arg])
  }

  init(sets: [ArgumentSet]) {
    self.init(sets.joined())
  }
  
  mutating func append(_ arg: ArgumentDefinition) {
    let newPosition = content.count
    content.append(arg)
    for name in arg.names where namePositions[name.nameToMatch] == nil {
      namePositions[name.nameToMatch] = newPosition
    }
  }
}

extension ArgumentSet: CustomDebugStringConvertible {
  var debugDescription: String {
    content
      .map { $0.debugDescription }
      .joined(separator: " / ")
  }
}

extension ArgumentSet: RandomAccessCollection {
  var startIndex: Int { content.startIndex }
  var endIndex: Int { content.endIndex }
  subscript(position: Int) -> ArgumentDefinition {
    content[position]
  }
}

// MARK: Flag

extension ArgumentSet {
  /// Creates an argument set for a single Boolean flag.
  static func flag(key: InputKey, name: NameSpecification, default initialValue: Bool?, help: ArgumentHelp?) -> ArgumentSet {
    // The flag is required if initialValue is `nil`, otherwise it's optional
    let helpOptions: ArgumentDefinition.Help.Options = initialValue != nil ? .isOptional : []
    { … 16 line(s) … }
    return ArgumentSet(arg)

  static func updateFlag<Value: Equatable>(key: InputKey, value: Value, origin: InputOrigin, values: inout ParsedValues, exclusivity: FlagExclusivity) throws {
    let hasUpdated: Bool
    if let previous = values.element(forKey: key) {
    { … 21 line(s) … }
    }
  
  /// Creates an argument set for a pair of inverted Boolean flags.
  static func flag(
    key: InputKey,
    name: NameSpecification,
    default initialValue: Bool?,
    required: Bool,
    inversion: FlagInversion,
    exclusivity: FlagExclusivity,
    help: ArgumentHelp?) -> ArgumentSet
    let helpOptions: ArgumentDefinition.Help.Options = required ? [] : .isOptional
        
    { … 18 line(s) … }
    return ArgumentSet([enableArg, disableArg])
  
  /// Creates an argument set for an incrementing integer flag.
  static func counter(key: InputKey, name: NameSpecification, help: ArgumentHelp?) -> ArgumentSet {
    let help = ArgumentDefinition.Help(allValueStrings: [], options: [.isOptional, .isRepeating], help: help, defaultValue: nil, key: key, isComposite: false)
    let arg = ArgumentDefinition(kind: .name(key: key, specification: name), help: help, completion: .default, update: .nullary({ (origin, name, values) in
      guard let a = values.element(forKey: key)?.value, let b = a as? Int else {
        { … 9 line(s) … }

extension ArgumentSet {
  /// Fills the given `ParsedValues` instance with initial values from this
  /// argument set.
  func setInitialValues(into parsed: inout ParsedValues) throws {
    for arg in self {
      try arg.initial(InputOrigin(), &parsed)
    }
  }
}

extension ArgumentSet {
  /// Find an `ArgumentDefinition` that matches the given `ParsedArgument`.
  ///
  /// As we iterate over the values from the command line, we try to find a
  /// definition that matches the particular element.
  /// - Parameters:
  ///   - parsed: The argument from the command line
  ///   - origin: Where `parsed` came from.
  /// - Returns: The matching definition.
  func first(
    matching parsed: ParsedArgument
  ) -> ArgumentDefinition? {
    namePositions[parsed.name].map { content[$0] }
  }
  
  func firstPositional(
    named name: String
  ) -> ArgumentDefinition? {
    let key = InputKey(name: name, parent: nil)
    return first(where: { $0.help.keys.contains(key) })
  }
}

/// A parser for a given input and set of arguments defined by the given
/// command.
///
/// This parser will consume only the arguments that it understands. If any
/// arguments are declared to capture all remaining input, or a subcommand
/// is configured as such, parsing stops on the first positional argument or
/// unrecognized dash-prefixed argument.
struct LenientParser {
  var command: ParsableCommand.Type
  var argumentSet: ArgumentSet
  var inputArguments: SplitArguments
  
  init(_ command: ParsableCommand.Type, _ split: SplitArguments) {
    self.command = command
    self.argumentSet = ArgumentSet(command, visibility: .private, parent: nil)
    self.inputArguments = split
  }
  
  var defaultCapturesForPassthrough: Bool {
    command.defaultIncludesPassthroughArguments
  }
  
  var subcommands: [ParsableCommand.Type] {
    command.configuration.subcommands
  }
  
  mutating func parseValue(
    _ argument: ArgumentDefinition,
    _ parsed: ParsedArgument,
    _ originElement: InputOrigin.Element,
    _ update: ArgumentDefinition.Update.Unary,
    _ result: inout ParsedValues,
    _ usedOrigins: inout InputOrigin
  ) throws {
    let origin = InputOrigin(elements: [originElement])
    switch argument.parsingStrategy {
    { … 29 line(s) … }
      } else if argument.allowsJoinedValue,
          let (origin2, value) = inputArguments.extractJoinedElement(at: originElement) {
        // Found a joined argument
        let origins = origin.inserting(origin2)
        { … 18 line(s) … }
      } else if argument.allowsJoinedValue,
          let (origin2, value) = inputArguments.extractJoinedElement(at: originElement) {
        // Found a joined argument
        let origins = origin.inserting(origin2)
        try update(origins, parsed.name, String(value), &result)
        usedOrigins.formUnion(origins)
      } else {
        guard let (origin2, value) = inputArguments.popNextElementAsValue(after: originElement) else {
          throw ParserError.missingValueForOption(origin, parsed.name)
        }
          { … 14 line(s) … }
      } else if argument.allowsJoinedValue,
          let (origin2, value) = inputArguments.extractJoinedElement(at: originElement) {
        // Found a joined argument
        let origins = origin.inserting(origin2)
        { … 18 line(s) … }
      } else if argument.allowsJoinedValue,
          let (origin2, value) = inputArguments.extractJoinedElement(at: originElement) {
        // Found a joined argument
        let origins = origin.inserting(origin2)
        try update(origins, parsed.name, String(value), &result)
        usedOrigins.formUnion(origins)
        inputArguments.removeAll(in: usedOrigins)
      }
      
      // Clear out the initial origin first, since it can include
      // the exploded elements of an options group (see issue #327).
      usedOrigins.formUnion(origin)
      inputArguments.removeAll(in: origin)
      
      // Fix incorrect error message
      // for @Option array without values (see issue #434).
      guard let first = inputArguments.elements.first,
            first.isValue
      else {
        throw ParserError.missingValueForOption(origin, parsed.name)
      }
      
      // ...and then consume the arguments until hitting an option
      while let (origin2, value) = inputArguments.popNextElementIfValue() {
        let origins = origin.inserting(origin2)
        try update(origins, parsed.name, value, &result)
        usedOrigins.formUnion(origins)
      }
      
    case .postTerminator, .allUnrecognized:
      // These parsing kinds are for arguments only.
      throw ParserError.invalidState
    }
  }
  
  mutating func parsePositionalValues(
    from unusedInput: SplitArguments,
    into result: inout ParsedValues
  ) throws {
    var endOfInput = unusedInput.elements.endIndex
    
    { … 27 line(s) … }
    /// dash-prefixed inputs unless `unconditional` is `true`.
    func next(unconditional: Bool) -> SplitArguments.Element? {
      while let arg = argumentStack.popFirst() {
        if arg.isValue || unconditional {
      { … 16 line(s) … }
        continue ArgumentLoop
      guard case let .unary(update) = argumentDefinition.update else {
        preconditionFailure("Shouldn't see a nullary positional argument.")
      }
      let allowOptionsAsInput = argumentDefinition.parsingStrategy == .allRemainingInput
      
      repeat {
        guard let arg = next(unconditional: allowOptionsAsInput) else {
          break ArgumentLoop
        }
          { … 21 line(s) … }
    }

  mutating func parse() throws -> ParsedValues {
    let originalInput = inputArguments
    defer { inputArguments = originalInput }
    { … 85 line(s) … }
  }