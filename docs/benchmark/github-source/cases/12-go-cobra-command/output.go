// Copyright 2013-2023 The Cobra Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Package cobra is a commander providing a simple interface to create powerful modern CLI interfaces.
// In addition to providing an interface, Cobra simultaneously provides a controller to organize your application code.
package cobra

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"

	flag "github.com/spf13/pflag"
)

const (
	FlagSetByCobraAnnotation     = "cobra_annotation_flag_set_by_cobra"
	CommandDisplayNameAnnotation = "cobra_annotation_command_display_name"
)

// FParseErrWhitelist configures Flag parse errors to be ignored
type FParseErrWhitelist flag.ParseErrorsWhitelist

// Group Structure to manage groups for commands
type Group struct {
	ID    string
	Title string
}

// Command is just that, a command for your application.
// E.g.  'go run ...' - 'run' is the command. Cobra requires
// you to define the usage and description as part of your command
// definition to ensure usability.
type Command struct {
    { … 74 line(s) … ⟦tj:aa08af03f8b8838b2e3e7c0f39f184c7⟧ }
	// PersistentPreRunE: PersistentPreRun but returns an error.
	PersistentPreRunE func(cmd *Command, args []string) error
	// PreRun: children of this command will not inherit.
	PreRun func(cmd *Command, args []string)
	// PreRunE: PreRun but returns an error.
	PreRunE func(cmd *Command, args []string) error
	// Run: Typically the actual work function. Most commands will only implement this.
	Run func(cmd *Command, args []string)
	// RunE: Run but returns an error.
	RunE func(cmd *Command, args []string) error
	// PostRun: run after the Run command.
	PostRun func(cmd *Command, args []string)
	// PostRunE: PostRun but returns an error.
	PostRunE func(cmd *Command, args []string) error
	// PersistentPostRun: children of this command will inherit and execute after PostRun.
	PersistentPostRun func(cmd *Command, args []string)
	// PersistentPostRunE: PersistentPostRun but returns an error.
	PersistentPostRunE func(cmd *Command, args []string) error
    { … 6 line(s) … ⟦tj:a3d150c27953c7dea340d773bb3253d7⟧ }
	// flagErrorBuf contains all error messages from pflag.
    { … 16 line(s) … ⟦tj:22a9395e1bc3e520a8ebb2f424401e9d⟧ }
	usageFunc func(*Command) error
	// usageTemplate is usage template defined by user.
	usageTemplate string
	// flagErrorFunc is func defined by user and it's called when the parsing of
	// flags returns an error.
	flagErrorFunc func(*Command, error) error
    { … 16 line(s) … ⟦tj:2c30b67bb62015b288178ae74cc954af⟧ }
	// errPrefix is the error message prefix defined by user.
    { … 9 line(s) … ⟦tj:b1ef810b823c24d541aea2e07e781472⟧ }
	// FParseErrWhitelist flag parse errors to be ignored
    { … 30 line(s) … ⟦tj:46913385db83ae942bf5c57a8901826d⟧ }
	// SilenceErrors is an option to quiet errors down stream.
	SilenceErrors bool

	// SilenceUsage is an option to silence usage when an error occurs.
    { … 22 line(s) … ⟦tj:285a8445a9ab90c8c72befcf0d270612⟧ }

// Context returns underlying command context. If command was executed
// with ExecuteContext or the context was set with SetContext, the
// previously set context will be returned. Otherwise, nil is returned.
//
// Notice that a call to Execute and ExecuteC will replace a nil context of
// a command with a context.Background, so a background context will be
// returned by Context after one of these functions has been called.
func (c *Command) Context() context.Context {
	return c.ctx
}

// SetContext sets context for the command. This context will be overwritten by
// Command.ExecuteContext or Command.ExecuteContextC.
func (c *Command) SetContext(ctx context.Context) {
	c.ctx = ctx
}

// SetArgs sets arguments for the command. It is set to os.Args[1:] by default, if desired, can be overridden
// particularly useful when testing.
func (c *Command) SetArgs(a []string) {
	c.args = a
}

// SetOutput sets the destination for usage and error messages.
// If output is nil, os.Stderr is used.
// Deprecated: Use SetOut and/or SetErr instead
func (c *Command) SetOutput(output io.Writer) {
	c.outWriter = output
	c.errWriter = output
}

// SetOut sets the destination for usage messages.
// If newOut is nil, os.Stdout is used.
func (c *Command) SetOut(newOut io.Writer) {
	c.outWriter = newOut
}

// SetErr sets the destination for error messages.
// If newErr is nil, os.Stderr is used.
func (c *Command) SetErr(newErr io.Writer) {
	c.errWriter = newErr
}

// SetIn sets the source for input data
// If newIn is nil, os.Stdin is used.
func (c *Command) SetIn(newIn io.Reader) {
	c.inReader = newIn
}

// SetUsageFunc sets usage function. Usage can be defined by application.
func (c *Command) SetUsageFunc(f func(*Command) error) {
	c.usageFunc = f
}

// SetUsageTemplate sets usage template. Can be defined by Application.
func (c *Command) SetUsageTemplate(s string) {
	c.usageTemplate = s
}

// SetFlagErrorFunc sets a function to generate an error when flag parsing
// fails.
func (c *Command) SetFlagErrorFunc(f func(*Command, error) error) {
	c.flagErrorFunc = f
}

// SetHelpFunc sets help function. Can be defined by Application.
func (c *Command) SetHelpFunc(f func(*Command, []string)) {
	c.helpFunc = f
}

// SetHelpCommand sets help command.
func (c *Command) SetHelpCommand(cmd *Command) {
	c.helpCommand = cmd
}

// SetHelpCommandGroupID sets the group id of the help command.
func (c *Command) SetHelpCommandGroupID(groupID string) {
    { … 6 line(s) … ⟦tj:2ce6ca126ddacec56ddb1af101ed8619⟧ }

// SetCompletionCommandGroupID sets the group id of the completion command.
func (c *Command) SetCompletionCommandGroupID(groupID string) {
	// completionCommandGroupID is used if no completion command is defined by the user
	c.Root().completionCommandGroupID = groupID
}

// SetHelpTemplate sets help template to be used. Application can use it to set custom template.
func (c *Command) SetHelpTemplate(s string) {
	c.helpTemplate = s
}

// SetVersionTemplate sets version template to be used. Application can use it to set custom template.
func (c *Command) SetVersionTemplate(s string) {
	c.versionTemplate = s
}

// SetErrPrefix sets error message prefix to be used. Application can use it to set custom prefix.
func (c *Command) SetErrPrefix(s string) {
	c.errPrefix = s
}

// SetGlobalNormalizationFunc sets a normalization function to all flag sets and also to child commands.
// The user should not have a cyclic dependency on commands.
func (c *Command) SetGlobalNormalizationFunc(n func(f *flag.FlagSet, name string) flag.NormalizedName) {
    { … 8 line(s) … ⟦tj:660ba1ee2df7857597889657ba11db02⟧ }

// OutOrStdout returns output to stdout.
func (c *Command) OutOrStdout() io.Writer {
	return c.getOut(os.Stdout)
}

// OutOrStderr returns output to stderr
func (c *Command) OutOrStderr() io.Writer {
	return c.getOut(os.Stderr)
}

// ErrOrStderr returns output to stderr
func (c *Command) ErrOrStderr() io.Writer {
	return c.getErr(os.Stderr)
}

// InOrStdin returns input to stdin
func (c *Command) InOrStdin() io.Reader {
	return c.getIn(os.Stdin)
}

func (c *Command) getOut(def io.Writer) io.Writer {
    { … 8 line(s) … ⟦tj:c6e21ba151ae0dc1c413eb8323ab64e9⟧ }

func (c *Command) getErr(def io.Writer) io.Writer {
    { … 8 line(s) … ⟦tj:e3154980230a53bd2a4c9edd1c9189c6⟧ }

func (c *Command) getIn(def io.Reader) io.Reader {
    { … 8 line(s) … ⟦tj:097101a979c70205b37619245c587487⟧ }

// UsageFunc returns either the function set by SetUsageFunc for this command
// or a parent, or it returns a default usage function.
func (c *Command) UsageFunc() (f func(*Command) error) {
    { … 6 line(s) … ⟦tj:d817b7f74ca458341994c96443911118⟧ }
	return func(c *Command) error {
    { … 8 line(s) … ⟦tj:5227a7ed5510233bb40349a37238f064⟧ }

// Usage puts out the usage for the command.
// Used when a user provides invalid input.
// Can be defined by user by overriding UsageFunc.
func (c *Command) Usage() error {
	return c.UsageFunc()(c)
}

// HelpFunc returns either the function set by SetHelpFunc for this command
// or a parent, or it returns a function with default help behavior.
func (c *Command) HelpFunc() func(*Command, []string) {
    { … 16 line(s) … ⟦tj:f1f709311180f851921b8b5e02d8144a⟧ }

// Help puts out the help for the command.
// Used when a user calls help [command].
// Can be defined by user by overriding HelpFunc.
func (c *Command) Help() error {
	c.HelpFunc()(c, []string{})
	return nil
}

// UsageString returns usage string.
func (c *Command) UsageString() string {
    { … 16 line(s) … ⟦tj:b268c64ad258ef95282c56d1a89391ac⟧ }

// FlagErrorFunc returns either the function set by SetFlagErrorFunc for this
// command or a parent, or it returns a function which returns the original
// error.
func (c *Command) FlagErrorFunc() (f func(*Command, error) error) {
    { … 7 line(s) … ⟦tj:043c0c9c4f3201ccf5f0f848092c145d⟧ }
	return func(c *Command, err error) error {
		return err
	}
}

var minUsagePadding = 25

// UsagePadding return padding for the usage.
func (c *Command) UsagePadding() int {
    { … 5 line(s) … ⟦tj:8d41d5deee859c3f2c0824fcd1dd5bd0⟧ }

var minCommandPathPadding = 11

// CommandPathPadding return padding for the command path.
func (c *Command) CommandPathPadding() int {
    { … 5 line(s) … ⟦tj:9e0dc07188e4c951a7575c92ef890916⟧ }

var minNamePadding = 11

// NamePadding returns padding for the name.
func (c *Command) NamePadding() int {
    { … 5 line(s) … ⟦tj:a35636139fa027731c1b9d8c4617fc90⟧ }

// UsageTemplate returns usage template for the command.
func (c *Command) UsageTemplate() string {
    { … 38 line(s) … ⟦tj:974922b7d69468711b961af11099a3c6⟧ }

// HelpTemplate return help template for the command.
func (c *Command) HelpTemplate() string {
    { … 11 line(s) … ⟦tj:6543e325d724ca9c0307fdd169366216⟧ }

// VersionTemplate return version template for the command.
func (c *Command) VersionTemplate() string {
    { … 10 line(s) … ⟦tj:821b900f3669bced190a15bbb0c265b7⟧ }

// ErrPrefix return error message prefix for the command
func (c *Command) ErrPrefix() string {
    { … 9 line(s) … ⟦tj:d373ffd46de816ebf369c412c4a82138⟧ }

func hasNoOptDefVal(name string, fs *flag.FlagSet) bool {
    { … 6 line(s) … ⟦tj:19bba1d7b9c27a523b79f6d9eb42be18⟧ }

func shortHasNoOptDefVal(name string, fs *flag.FlagSet) bool {
    { … 10 line(s) … ⟦tj:e2627b219c6fdf51afd034b3c0ca9409⟧ }

func stripFlags(args []string, c *Command) []string {
    { … 36 line(s) … ⟦tj:f84698717a2eb4570fc3695d5a34be16⟧ }

// argsMinusFirstX removes only the first x from args.  Otherwise, commands that look like
// openshift admin policy add-role-to-user admin my-user, lose the admin argument (arg[4]).
// Special care needs to be taken not to remove a flag value.
func (c *Command) argsMinusFirstX(args []string, x string) []string {
    { … 33 line(s) … ⟦tj:dc9087eb2b0de3dfd8d6aff5be45487a⟧ }

func isFlagArg(arg string) bool {
	return ((len(arg) >= 3 && arg[0:2] == "--") ||
		(len(arg) >= 2 && arg[0] == '-' && arg[1] != '-'))
}

// Find the target command given the args and command tree
// Meant to be run on the highest node. Only searches down.
func (c *Command) Find(args []string) (*Command, []string, error) {
    { … 22 line(s) … ⟦tj:c6d1234344adb1356aa89922c4b40fa2⟧ }

func (c *Command) findSuggestions(arg string) string {
    { … 15 line(s) … ⟦tj:b83931d037b102fce8d59b22119c78c1⟧ }

func (c *Command) findNext(next string) *Command {
    { … 19 line(s) … ⟦tj:bba8621e95f17e2e23820ecd07abce3f⟧ }

// Traverse the command tree to find the command, and parse args for
// each parent.
func (c *Command) Traverse(args []string) (*Command, []string, error) {
    { … 7 line(s) … ⟦tj:94c850c28a5d671a49e21fa071271951⟧ }
			// TODO: this isn't quite right, we should really check ahead for 'true' or 'false'
    { … 31 line(s) … ⟦tj:eae2fb82a6a2f1b8704e4309dad666bf⟧ }

// SuggestionsFor provides suggestions for the typedName.
func (c *Command) SuggestionsFor(typedName string) []string {
    { … 18 line(s) … ⟦tj:781bbc41491e18cb3a4b3985a9bf1a8a⟧ }

// VisitParents visits all parents of the command and invokes fn on each parent.
func (c *Command) VisitParents(fn func(*Command)) {
    { … 5 line(s) … ⟦tj:8e1d08f3305525207923f3cd116efaec⟧ }

// Root finds root command.
func (c *Command) Root() *Command {
    { … 5 line(s) … ⟦tj:fec5652e8d819400a1c536675dc00ac4⟧ }

// ArgsLenAtDash will return the length of c.Flags().Args at the moment
// when a -- was found during args parsing.
func (c *Command) ArgsLenAtDash() int {
	return c.Flags().ArgsLenAtDash()
}

func (c *Command) execute(a []string) (err error) {
    { … 139 line(s) … ⟦tj:b0c686008a04c374663cb0f4c2340d81⟧ }

func (c *Command) preRun() {
    { … 4 line(s) … ⟦tj:f3db42372e1cb4ce0d7e1c239a745257⟧ }

func (c *Command) postRun() {
    { … 4 line(s) … ⟦tj:1f17d540077d65b33e2c70cc545617d8⟧ }

// ExecuteContext is the same as Execute(), but sets the ctx on the command.
// Retrieve ctx by calling cmd.Context() inside your *Run lifecycle or ValidArgs
// functions.
func (c *Command) ExecuteContext(ctx context.Context) error {
	c.ctx = ctx
	return c.Execute()
}

// Execute uses the args (os.Args[1:] by default)
// and run through the command tree finding appropriate matches
// for commands and then corresponding flags.
func (c *Command) Execute() error {
	_, err := c.ExecuteC()
	return err
}

// ExecuteContextC is the same as ExecuteC(), but sets the ctx on the command.
// Retrieve ctx by calling cmd.Context() inside your *Run lifecycle or ValidArgs
// functions.
func (c *Command) ExecuteContextC(ctx context.Context) (*Command, error) {
	c.ctx = ctx
	return c.ExecuteC()
}

// ExecuteC executes the command.
func (c *Command) ExecuteC() (cmd *Command, err error) {
    { … 66 line(s) … ⟦tj:f77882a6e4b4141ce432ad8c9a6c30bc⟧ }
		if errors.Is(err, flag.ErrHelp) {
    { … 18 line(s) … ⟦tj:75144093f89bf56628432ff91413e1ce⟧ }

func (c *Command) ValidateArgs(args []string) error {
    { … 5 line(s) … ⟦tj:a13dd31b96b5cc49cbc9f8a69e11e08c⟧ }

// ValidateRequiredFlags validates all required flags are present and returns an error otherwise
func (c *Command) ValidateRequiredFlags() error {
    { … 21 line(s) … ⟦tj:5081adba46330cfe78149821934c479a⟧ }

// checkCommandGroups checks if a command has been added to a group that does not exists.
// If so, we panic because it indicates a coding error that should be corrected.
func (c *Command) checkCommandGroups() {
	for _, sub := range c.commands {
		// if Group is not defined let the developer know right away
		if sub.GroupID != "" && !c.ContainsGroup(sub.GroupID) {
			panic(fmt.Sprintf("group id '%s' is not defined for subcommand '%s'", sub.GroupID, sub.CommandPath()))
    { … 5 line(s) … ⟦tj:987342f43315e63fd72c8f133749fe9f⟧ }

// InitDefaultHelpFlag adds default help flag to c.
// It is called automatically by executing the c or by calling help and usage.
// If c already has help flag, it will do nothing.
func (c *Command) InitDefaultHelpFlag() {
    { … 12 line(s) … ⟦tj:fc687f6a4fc87e4deb5e1fb2f4faec87⟧ }

// InitDefaultVersionFlag adds default version flag to c.
// It is called automatically by executing the c.
// If c already has a version flag, it will do nothing.
// If c.Version is empty, it will do nothing.
func (c *Command) InitDefaultVersionFlag() {
    { … 20 line(s) … ⟦tj:52c6a6021baaf73b3fba7ff3c57d2f2a⟧ }

// InitDefaultHelpCmd adds default help command to c.
// It is called automatically by executing the c or by calling help and usage.
// If c already has help command or c has no subcommands, it will do nothing.
func (c *Command) InitDefaultHelpCmd() {
    { … 46 line(s) … ⟦tj:8a2b04c59ddacf34173cf9da0a642ca8⟧ }

// ResetCommands delete parent, subcommand and help command from c.
func (c *Command) ResetCommands() {
    { … 5 line(s) … ⟦tj:c404397d458a91d4193868a0948cf4b5⟧ }

// Sorts commands by their names.
type commandSorterByName []*Command

func (c commandSorterByName) Len() int           { return len(c) }
func (c commandSorterByName) Swap(i, j int)      { c[i], c[j] = c[j], c[i] }
func (c commandSorterByName) Less(i, j int) bool { return c[i].Name() < c[j].Name() }

// Commands returns a sorted slice of child commands.
func (c *Command) Commands() []*Command {
    { … 7 line(s) … ⟦tj:c47b1949727be75eff04fadf76aec6e5⟧ }

// AddCommand adds one or more commands to this parent command.
func (c *Command) AddCommand(cmds ...*Command) {
	for i, x := range cmds {
		if cmds[i] == c {
			panic("Command can't be a child of itself")
    { … 23 line(s) … ⟦tj:5cf10b15f18b1769e72e8a314c022647⟧ }

// Groups returns a slice of child command groups.
func (c *Command) Groups() []*Group {
	return c.commandgroups
}

// AllChildCommandsHaveGroup returns if all subcommands are assigned to a group
func (c *Command) AllChildCommandsHaveGroup() bool {
    { … 7 line(s) … ⟦tj:c99f20faeedd8eaddbf43ae343cccf24⟧ }

// ContainsGroup return if groupID exists in the list of command groups.
func (c *Command) ContainsGroup(groupID string) bool {
    { … 7 line(s) … ⟦tj:079f147e9e289258a42f52bd3639002f⟧ }

// AddGroup adds one or more command groups to this parent command.
func (c *Command) AddGroup(groups ...*Group) {
	c.commandgroups = append(c.commandgroups, groups...)
}

// RemoveCommand removes one or more commands from a parent command.
func (c *Command) RemoveCommand(cmds ...*Command) {
    { … 31 line(s) … ⟦tj:db7445abd9d3d1fcf2fe9a97ca592567⟧ }

// Print is a convenience method to Print to the defined output, fallback to Stderr if not set.
func (c *Command) Print(i ...interface{}) {
	fmt.Fprint(c.OutOrStderr(), i...)
}

// Println is a convenience method to Println to the defined output, fallback to Stderr if not set.
func (c *Command) Println(i ...interface{}) {
	c.Print(fmt.Sprintln(i...))
}

// Printf is a convenience method to Printf to the defined output, fallback to Stderr if not set.
func (c *Command) Printf(format string, i ...interface{}) {
	c.Print(fmt.Sprintf(format, i...))
}

// PrintErr is a convenience method to Print to the defined Err output, fallback to Stderr if not set.
func (c *Command) PrintErr(i ...interface{}) {
	fmt.Fprint(c.ErrOrStderr(), i...)
}

// PrintErrln is a convenience method to Println to the defined Err output, fallback to Stderr if not set.
func (c *Command) PrintErrln(i ...interface{}) {
	c.PrintErr(fmt.Sprintln(i...))
}

// PrintErrf is a convenience method to Printf to the defined Err output, fallback to Stderr if not set.
func (c *Command) PrintErrf(format string, i ...interface{}) {
	c.PrintErr(fmt.Sprintf(format, i...))
}

// CommandPath returns the full path to this command.
func (c *Command) CommandPath() string {
    { … 8 line(s) … ⟦tj:78c902fa3e41049d49441bc7f5f81dc5⟧ }

// UseLine puts out the full usage for a given command (including parents).
func (c *Command) UseLine() string {
    { … 14 line(s) … ⟦tj:8a6358a27ab9bea6fe4c477a219031d5⟧ }

// DebugFlags used to determine which flags have been assigned to which commands
// and which persist.
// nolint:goconst
func (c *Command) DebugFlags() {
    { … 37 line(s) … ⟦tj:1da43b8008d24c26b2cb3f0fa2aac970⟧ }

// Name returns the command's name: the first word in the use line.
func (c *Command) Name() string {
    { … 7 line(s) … ⟦tj:cf28d45bb3ffcb24bd1e8f92ab4b45f8⟧ }

// HasAlias determines if a given string is an alias of the command.
func (c *Command) HasAlias(s string) bool {
    { … 7 line(s) … ⟦tj:531cb51d88333fee70098a8509aa0a46⟧ }

// CalledAs returns the command name or alias that was used to invoke
// this command or an empty string if the command has not been called.
func (c *Command) CalledAs() string {
    { … 5 line(s) … ⟦tj:0ec2766fbf3e56f5d05a396a218445d6⟧ }

// hasNameOrAliasPrefix returns true if the Name or any of aliases start
// with prefix
func (c *Command) hasNameOrAliasPrefix(prefix string) bool {
    { … 12 line(s) … ⟦tj:a4de1be9efaee76378af524a6a717e07⟧ }

// NameAndAliases returns a list of the command name and all aliases
func (c *Command) NameAndAliases() string {
	return strings.Join(append([]string{c.Name()}, c.Aliases...), ", ")
}

// HasExample determines if the command has example.
func (c *Command) HasExample() bool {
	return len(c.Example) > 0
}

// Runnable determines if the command is itself runnable.
func (c *Command) Runnable() bool {
	return c.Run != nil || c.RunE != nil
}

// HasSubCommands determines if the command has children commands.
func (c *Command) HasSubCommands() bool {
	return len(c.commands) > 0
}

// IsAvailableCommand determines if a command is available as a non-help command
// (this includes all non deprecated/hidden commands).
func (c *Command) IsAvailableCommand() bool {
    { … 14 line(s) … ⟦tj:68b106ac98f515c82b5b180031b1db21⟧ }

// IsAdditionalHelpTopicCommand determines if a command is an additional
// help topic command; additional help topic command is determined by the
// fact that it is NOT runnable/hidden/deprecated, and has no sub commands that
// are runnable/hidden/deprecated.
// Concrete example: https://github.com/spf13/cobra/issues/393#issuecomment-282741924.
func (c *Command) IsAdditionalHelpTopicCommand() bool {
    { … 15 line(s) … ⟦tj:eb7841dd204cfb7dbac20bb0b6f232b9⟧ }

// HasHelpSubCommands determines if a command has any available 'help' sub commands
// that need to be shown in the usage/help default template under 'additional help
// topics'.
func (c *Command) HasHelpSubCommands() bool {
    { … 10 line(s) … ⟦tj:9323ca2d4954dc3c7cab13032cab6e99⟧ }

// HasAvailableSubCommands determines if a command has available sub commands that
// need to be shown in the usage/help default template under 'available commands'.
func (c *Command) HasAvailableSubCommands() bool {
    { … 12 line(s) … ⟦tj:4b73f7913782f1907b5508e369072444⟧ }

// HasParent determines if the command is a child command.
func (c *Command) HasParent() bool {
	return c.parent != nil
}

// GlobalNormalizationFunc returns the global normalization function or nil if it doesn't exist.
func (c *Command) GlobalNormalizationFunc() func(f *flag.FlagSet, name string) flag.NormalizedName {
	return c.globNormFunc
}

// Flags returns the complete FlagSet that applies
// to this command (local and persistent declared here and by all parents).
func (c *Command) Flags() *flag.FlagSet {
    { … 10 line(s) … ⟦tj:ae693c36393c5db13cf7dd12b63ceb62⟧ }

// LocalNonPersistentFlags are flags specific to this command which will NOT persist to subcommands.
func (c *Command) LocalNonPersistentFlags() *flag.FlagSet {
    { … 10 line(s) … ⟦tj:d72b7b6c6645f1317143cec4f96fd2a0⟧ }

// LocalFlags returns the local FlagSet specifically set in the current command.
func (c *Command) LocalFlags() *flag.FlagSet {
    { … 24 line(s) … ⟦tj:9199de84741b261694ece04f89de4079⟧ }

// InheritedFlags returns all flags which were inherited from parent commands.
func (c *Command) InheritedFlags() *flag.FlagSet {
    { … 22 line(s) … ⟦tj:ce99d1e2464cb2eb9e0ce8384252cf48⟧ }

// NonInheritedFlags returns all flags which were not inherited from parent commands.
func (c *Command) NonInheritedFlags() *flag.FlagSet {
	return c.LocalFlags()
}

// PersistentFlags returns the persistent FlagSet specifically set in the current command.
func (c *Command) PersistentFlags() *flag.FlagSet {
    { … 9 line(s) … ⟦tj:615fa5209f36b3930115b0e7f17e59ac⟧ }

// ResetFlags deletes all flags from command.
func (c *Command) ResetFlags() {
    { … 11 line(s) … ⟦tj:1b5d0793e8bdd45faf803c2b8b47cf16⟧ }

// HasFlags checks if the command contains any flags (local plus persistent from the entire structure).
func (c *Command) HasFlags() bool {
	return c.Flags().HasFlags()
}

// HasPersistentFlags checks if the command contains persistent flags.
func (c *Command) HasPersistentFlags() bool {
	return c.PersistentFlags().HasFlags()
}

// HasLocalFlags checks if the command has flags specifically declared locally.
func (c *Command) HasLocalFlags() bool {
	return c.LocalFlags().HasFlags()
}

// HasInheritedFlags checks if the command has flags inherited from its parent command.
func (c *Command) HasInheritedFlags() bool {
	return c.InheritedFlags().HasFlags()
}

// HasAvailableFlags checks if the command contains any flags (local plus persistent from the entire
// structure) which are not hidden or deprecated.
func (c *Command) HasAvailableFlags() bool {
	return c.Flags().HasAvailableFlags()
}

// HasAvailablePersistentFlags checks if the command contains persistent flags which are not hidden or deprecated.
func (c *Command) HasAvailablePersistentFlags() bool {
	return c.PersistentFlags().HasAvailableFlags()
}

// HasAvailableLocalFlags checks if the command has flags specifically declared locally which are not hidden
// or deprecated.
func (c *Command) HasAvailableLocalFlags() bool {
	return c.LocalFlags().HasAvailableFlags()
}

// HasAvailableInheritedFlags checks if the command has flags inherited from its parent command which are
// not hidden or deprecated.
func (c *Command) HasAvailableInheritedFlags() bool {
	return c.InheritedFlags().HasAvailableFlags()
}

// Flag climbs up the command tree looking for matching flag.
func (c *Command) Flag(name string) (flag *flag.Flag) {
    { … 8 line(s) … ⟦tj:114e3237ffe93b14ae7aefa8f8b37009⟧ }

// Recursively find matching persistent flag.
func (c *Command) persistentFlag(name string) (flag *flag.Flag) {
    { … 10 line(s) … ⟦tj:3fa48444795d1c3082e3b0f8c1d8b08f⟧ }

// ParseFlags parses persistent flag tree and local flags.
func (c *Command) ParseFlags(args []string) error {
    { … 21 line(s) … ⟦tj:73960c97bff197a52e372e2d2afd8134⟧ }

// Parent returns a commands parent command.
func (c *Command) Parent() *Command {
	return c.parent
}

// mergePersistentFlags merges c.PersistentFlags() to c.Flags()
// and adds missing persistent flags of all parents.
func (c *Command) mergePersistentFlags() {
    { … 4 line(s) … ⟦tj:84800de2342db8e84ba56085bd09fd2e⟧ }

// updateParentsPflags updates c.parentsPflags by adding
// new persistent flags of all parents.
// If c.parentsPflags == nil, it makes new.
func (c *Command) updateParentsPflags() {
    { … 16 line(s) … ⟦tj:e9e8a87d3d944b5ff5a2fdfed2179a04⟧ }

// commandNameMatches checks if two command names are equal
// taking into account case sensitivity according to
// EnableCaseInsensitive global configuration.
func commandNameMatches(s string, t string) bool {
    { … 6 line(s) … ⟦tj:e27daae11f58e41f79108f5d5a79cb1d⟧ }
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (55213 bytes) is available by calling tinyjuice_retrieve with token "59a0d770bb4e2e52e24e8107551f2e2e" (marker ⟦tj:59a0d770bb4e2e52e24e8107551f2e2e⟧)]