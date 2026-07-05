# JSON SmartCrusher

Real OpenHuman JSON snapshots: tool catalogs, API responses, schemas, package metadata, Lottie payloads, and config files. TinyJuice now chooses the smallest useful JSON representation before CCR, using Markdown tables only when they beat minified JSON.

Each row links to the full raw input and the exact compacted output used by the benchmark. Percentages are **token reduction: higher is better**; 0% means pass-through. `Bytes` shows the raw input size -> compressor-only output size and its byte reduction. `Pass 1` disables CCR (compressed with omission markers, no recovery footer). `Pass 2` is the final model-facing result with CCR enabled — it reads marginally *lower* than Pass 1 only because the recovery footer adds a few dozen bytes to the output.

## Cases

Every case links to the raw input, the exact model-facing output (with the CCR recovery footer), and a unified diff between the two.

| Case | Input | Output (after CCR) | Diff | Bytes | Pass 1: no CCR | Pass 2: with CCR | Avg latency |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| `02-notion-tools-array` | [input](cases/02-notion-tools-array/input.json) | [output](cases/02-notion-tools-array/output.md) | [diff](cases/02-notion-tools-array/compression.diff) | 890.8 KB -> 231.2 KB (-74%) | 74.1% | 74.0% | 2.628 ms |
| `03-slack-tools-array` | [input](cases/03-slack-tools-array/input.json) | [output](cases/03-slack-tools-array/output.md) | [diff](cases/03-slack-tools-array/compression.diff) | 106.7 KB -> 36.6 KB (-66%) | 65.9% | 65.5% | 0.367 ms |
| `01-github-tools-array` | [input](cases/01-github-tools-array/input.json) | [output](cases/01-github-tools-array/output.md) | [diff](cases/01-github-tools-array/compression.diff) | 110.9 KB -> 43.8 KB (-61%) | 60.7% | 60.3% | 0.402 ms |
| `10-cargo-metadata` | [input](cases/10-cargo-metadata/input.json) | [output](cases/10-cargo-metadata/output.md) | [diff](cases/10-cargo-metadata/compression.diff) | 62.0 KB -> 62.0 KB (-0%) | 0.0% | 0.0% | 0.148 ms |
| `09-package-manifest` | [input](cases/09-package-manifest/input.json) | [output](cases/09-package-manifest/output.md) | [diff](cases/09-package-manifest/compression.diff) | 9.4 KB -> 9.4 KB (-0%) | 0.0% | 0.0% | 0.019 ms |
| `08-lottie-animation` | [input](cases/08-lottie-animation/input.json) | [output](cases/08-lottie-animation/output.md) | [diff](cases/08-lottie-animation/compression.diff) | 16.8 KB -> 16.8 KB (-0%) | 0.0% | 0.0% | 0.030 ms |
| `07-app-schema-object` | [input](cases/07-app-schema-object/input.json) | [output](cases/07-app-schema-object/output.md) | [diff](cases/07-app-schema-object/compression.diff) | 48.6 KB -> 48.6 KB (-0%) | 0.0% | 0.0% | 0.087 ms |
| `06-tauri-capabilities-schema` | [input](cases/06-tauri-capabilities-schema/input.json) | [output](cases/06-tauri-capabilities-schema/output.md) | [diff](cases/06-tauri-capabilities-schema/compression.diff) | 2.4 KB -> 2.4 KB (-0%) | 0.0% | 0.0% | 0.004 ms |
| `05-polymarket-events-list` | [input](cases/05-polymarket-events-list/input.json) | [output](cases/05-polymarket-events-list/output.md) | [diff](cases/05-polymarket-events-list/compression.diff) | 201 B -> 201 B (-0%) | 0.0% | 0.0% | 0.000 ms |
| `04-polymarket-markets-list` | [input](cases/04-polymarket-markets-list/input.json) | [output](cases/04-polymarket-markets-list/output.md) | [diff](cases/04-polymarket-markets-list/compression.diff) | 313 B -> 313 B (-0%) | 0.0% | 0.0% | 0.000 ms |

## What TinyJuice Is Doing

TinyJuice parses JSON before choosing a representation. Homogeneous object arrays can become GitHub-renderable Markdown tables, but minified JSON wins when it is smaller or when the JSON shape is too nested for a readable table. If neither representation saves space, the router returns the original.

## Syntax-Aware Samples

### `02-notion-tools-array`

- [Full input](cases/02-notion-tools-array/input.json)
- [Full output](cases/02-notion-tools-array/output.md)
- [Input vs output diff](cases/02-notion-tools-array/compression.diff)

Input excerpt:

```json
[
  {
    "function": {
      "description": "Bulk-add content blocks to Notion. Text >2000 chars auto-splits. Parses markdown formatting. ⚠️ PARENT BLOCK TYPES: Content is added AS CHILDREN of parent_block_id. - To add content AFTER a head...
      "name": "NOTION_ADD_MULTIPLE_PAGE_CONTENT",
      "parameters": {
        "properties": {
          "after": {
            "description": "Block ID to insert content AFTER (as siblings). Use this to add content after a heading: set parent_block_id to the PAGE ID and 'after' to the HEADING block ID. The new blocks appear immediate...
            "examples": [
              "4b5f6e87-123a-456b-789c-9de8f7a9e4c0"
            ],
            "title": "After",
            "type": "string"
          },
          "content_blocks": {
            "description": "⚠️ CRITICAL: Notion API enforces 2000 char limit per text.content field. Content >2000 chars auto-splits.\nList of blocks to add (max 100). Also accepts 'blocks' as alias. Each item can be...
            "examples": [
              [
                {
                  "block_property": "heading_1",
                  "content": "# Project Status Report"
                },
                {
                  "block_property": "paragraph",
                  "content": "System is **running smoothly** with *excellent* performance."
                },
                {
                  "block_property": "divider"
                },
                {
                  "block_property": "to_do",
                  "content": "Task item"
                }
              ],
              [

```

Output excerpt:

```markdown
[json table: 48 rows × 2 cols · blank=absent key · exact original via retrieve footer]
| function | type |
| --- | --- |
| {"description":"Bulk-add content blocks to Notion. Text >2000 chars auto-splits. Parses markdown formatting. ⚠️ PARENT BLOCK TYPES: Content is added AS CHILDREN of parent_block_id. - To add content AFTER a heading,...
| {"description":"DEPRECATED: Use 'add_multiple_page_content' for better performance. Adds a single content block to a Notion page/block. CRITICAL: Notion API enforces a HARD LIMIT of 2000 characters per text.content fie...
| {"description":"DEPRECATED: Use NOTION_APPEND_TEXT_BLOCKS, NOTION_APPEND_TASK_BLOCKS, NOTION_APPEND_CODE_BLOCKS, NOTION_APPEND_MEDIA_BLOCKS, NOTION_APPEND_LAYOUT_BLOCKS, or NOTION_APPEND_TABLE_BLOCKS instead. Appends r...
| {"description":"Append code and technical blocks (code, quote, equation) to a Notion page. Use for: - Code snippets and programming examples (code) - Citations and highlighted quotes (quote) - Mathematical formulas and...
| {"description":"Append layout blocks (divider, TOC, breadcrumb, columns) to a Notion page. Supported types: - divider: Horizontal line separator - table_of_contents: Auto-generated from headings - breadcrumb: Page hier...
| {"description":"Append media blocks (image, video, audio, file, pdf, embed, bookmark) to a Notion page. Use for: - Images and screenshots (image) - YouTube/Vimeo videos or direct video URLs (video) - Audio files and po...
| {"description":"Append table blocks to a Notion page. Use for structured tabular data like spreadsheets, comparison charts, and status trackers. Example: { \\"table_width\\": 3, \\"has_column_header\\": true, \\"rows\\...
| {"description":"Append task blocks (to-do, toggle, callout) to a Notion page or block. Supported block types: - to_do: Checkbox items (checkable/uncheckable) - toggle: Collapsible sections - callout: Highlighted boxes ...
| {"description":"Append text blocks (paragraphs, headings, lists) to a Notion page. This is the most commonly used action for adding content to Notion. Use for: documentation, notes, articles, outlines, lists. Supported...
| {"description":"Archives (moves to trash) or unarchives (restores from trash) a specified Notion page. Limitation: Workspace-level pages (top-level pages with no parent page or database) cannot be archived via the API ...
| {"description":"Adds a comment to a Notion page (via `parent_page_id`) OR to an existing discussion thread (via `discussion_id`); cannot create new discussion threads on specific blocks (inline comments).","name":"NOTI...
| {"description":"Creates a new Notion database as a subpage under a specified parent page with a defined properties schema. IMPORTANT NOTES: - The parent page MUST be shared with your integration, otherwise you'll get a...
| {"description":"Tool to create a Notion FileUpload object and retrieve an upload URL. Use when you need to automate attaching local or external files directly into Notion without external hosting.","name":"NOTION_CREAT...
| {"description":"Creates a new page in a Notion workspace under a specified parent page or database. Supports creating pages with markdown content using the native markdown parameter, or as an empty page that can be pop...
| {"description":"Archives a Notion block, page, or database using its ID, which sets its 'archived' property to true (like moving to \\"Trash\\" in the UI) and allows it to be restored later. Note: This operation will f...
| {"description":"Duplicates a Notion page, including all its content, properties, and nested blocks, under a specified parent page or workspace.","name":"NOTION_DUPLICATE_PAGE","parameters":{"description":"Defines the p...
| {"description":"Tool to fetch all child blocks for a given Notion block. Use when you need a complete listing of a block's children beyond a single page; supports optional recursive expansion of nested blocks.","name":...
| {"description":"Retrieves a paginated list of direct, first-level child block objects along with contents for a given parent Notion block or page ID; use block IDs from the response for subsequent calls to access deepl...
| {"description":"Fetches metadata for a Notion block (including pages, which are special blocks) using its UUID. Returns block type, properties, and basic info but not child content. Prerequisites: 1) Block/page must be...
| {"description":"Fetches unresolved comments for a specified Notion block or page ID. The block/page must be shared with your Notion integration and the integration must have 'Read comments' capability enabled, otherwis...
[... 18 row(s) omitted ... ⟦tj:9be16660ff540bdcc913a1b7116fd0ed⟧]
| {"description":"Tool to retrieve a specific property object of a Notion database. Use when you need to get details about a single database column/property.","name":"NOTION_RETRIEVE_DATABASE_PROPERTY","parameters":{"pro...
| {"description":"Tool to retrieve details of a Notion File Upload object by its identifier. Use when you need to check the status or details of an existing file upload.","name":"NOTION_RETRIEVE_FILE_UPLOAD","parameters"...
| {"description":"Retrieve a Notion page's properties/metadata (not block content) by page_id. Use when you have a page URL/ID and need to access its properties; for page content use block-children tools.","name":"NOTION...
| {"description":"Searches Notion pages and databases by title. Use specific search terms to find items by title (primary approach). KNOWN LIMITATIONS: (1) Search indexing is not immediate - recently shared items may not...
| {"description":"Tool to transmit file contents to Notion for a file upload object. Use after creating a file upload object to send the actual file data.","name":"NOTION_SEND_FILE_UPLOAD","parameters":{"properties":{"fi...
| {"description":"Updates existing Notion block's text content. ⚠️ CRITICAL: Content limited to 2000 chars. Cannot change block type or archive blocks. Content exceeding 2000 chars will fail with validation error. Fo...
| {"description":"Update page properties, icon, cover, or archive status. IMPORTANT: Property names are workspace-specific and case-sensitive. Use NOTION_FETCH_ROW or NOTION_FETCH_DATABASE first to discover exact propert...
| {"description":"Updates a specific row/page within a Notion database by its page UUID (row_id). IMPORTANT CLARIFICATION: This action updates INDIVIDUAL ROWS (pages) in a database, NOT the database structure. - To updat...
| {"description":"Updates an existing Notion database's schema including title, description, and/or properties (columns). IMPORTANT NOTES: - At least one update (title, description, or properties) must be provided - The ...
| {"description":"Tool to upsert rows in a Notion database by querying for existing rows and creating or updating them. Use when you need to sync data to Notion without creating duplicates. Each item is matched by a filt...
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]


```

### `03-slack-tools-array`

- [Full input](cases/03-slack-tools-array/input.json)
- [Full output](cases/03-slack-tools-array/output.md)
- [Input vs output diff](cases/03-slack-tools-array/compression.diff)

Input excerpt:

```json
[
  {
    "function": {
      "description": "Registers new participants added to a Slack call.",
      "name": "SLACK_ADD_CALL_PARTICIPANTS",
      "parameters": {
        "description": "Request schema for `AddCallParticipants`",
        "properties": {
          "id": {
            "description": "ID of the call returned by the add method.",
            "examples": [
              "R0123456789"
            ],
            "title": "Id",
            "type": "string"
          },
          "users": {
            "description": "The list of users to add as participants in the call. users is a JSON array (formatted as a string) containing information for each user. Each element must include a `slack_id`. For example: `...
            "examples": [
              "[{\"slack_id\": \"U1H77\"}]",
              "[{\"slack_id\": \"U2ABC123\"}]",
              "[{\"slack_id\": \"U1H77\"}, {\"slack_id\": \"U2ABC123\"}]"
            ],
            "title": "Users",
            "type": "string"
          }
        },
        "required": [
          "id",
          "users"
        ],
        "title": "AddCallParticipantsRequest",
        "type": "object"
      }
    },
    "type": "function"

```

Output excerpt:

```markdown
[json table: 60 rows × 2 cols · blank=absent key · exact original via retrieve footer]
| function | type |
| --- | --- |
| {"description":"Registers new participants added to a Slack call.","name":"SLACK_ADD_CALL_PARTICIPANTS","parameters":{"description":"Request schema for `AddCallParticipants`","properties":{"id":{"description":"ID of th...
| {"description":"Adds a custom emoji to a Slack workspace given a unique name and an image URL; subject to workspace emoji limits.","name":"SLACK_ADD_EMOJI","parameters":{"description":"Request schema for `AddEmoji`","p...
| {"description":"Adds an alias for an existing custom emoji in a Slack Enterprise Grid organization.","name":"SLACK_ADD_EMOJI_ALIAS","parameters":{"description":"Request schema for `AddEmojiAlias`","properties":{"alias_...
| {"description":"Adds an Enterprise user to a workspace. Use when you need to assign an existing Enterprise Grid user to a specific workspace with optional guest restrictions.","name":"SLACK_ADD_ENTERPRISE_USER_TO_WORKS...
| {"description":"Adds a specified emoji reaction to an existing message in a Slack channel, identified by its timestamp; does not remove or retrieve reactions.","name":"SLACK_ADD_REACTION_TO_AN_ITEM","parameters":{"desc...
| {"description":"Adds a reference to an external file (e.g., Google Drive, Dropbox) to Slack for discovery and sharing, requiring a unique `external_id` and an `external_url` accessible by Slack.","name":"SLACK_ADD_REMO...
| {"description":"Stars a channel, file, file comment, or a specific message in Slack.","name":"SLACK_ADD_STAR","parameters":{"description":"Request schema for the `stars.add` API method. Used to add a star to a channel,...
| {"description":"Tool to search for public or private channels in an Enterprise organization. Use when you need to find channels by name, type, or other criteria within an Enterprise Grid workspace.","name":"SLACK_ADMIN...
| {"description":"Tool to check API calling code by testing connectivity and authentication to the Slack API. Use when you need to verify that API credentials are valid and the connection is working properly.","name":"SL...
| {"description":"Archives a Slack conversation by its ID, rendering it read-only and hidden while retaining history, ideal for cleaning up inactive channels; be aware that some channels (like #general or certain DMs) ca...
| {"description":"Search across Slack messages, files, channels, and users using Real-time Search API. BEFORE USING: Call SLACK_ASSISTANT_SEARCH_INFO to check workspace capabilities. - If is_ai_search_enabled=true → Us...
| {"description":"Check if semantic (AI-powered) search is available on the Slack workspace. Returns whether natural language queries will trigger semantic search in assistant.search.context calls.","name":"SLACK_ASSISTA...
| {"description":"Closes a Slack direct message (DM) or multi-person direct message (MPDM) channel, removing it from the user's sidebar without deleting history; this action affects only the calling user's view.","name":...
| {"description":"Convert a public Slack channel to private using the Admin API. This is an Enterprise Grid only feature and requires an org-installed user token with admin.conversations:write scope.","name":"SLACK_CONVE...
| {"description":"Creates a Slack reminder with specified text and time; time accepts Unix timestamps, seconds from now, or natural language (e.g., 'in 15 minutes', 'every Thursday at 2pm').","name":"SLACK_CREATE_A_REMIN...
| {"description":"Creates a new Slack Canvas with the specified title and optional content.","name":"SLACK_CREATE_CANVAS","parameters":{"properties":{"channel_id":{"description":"Optional channel ID (e.g., 'C1234567890')...
| {"description":"Initiates a public or private channel-based conversation in a Slack workspace. Immediately creates the channel; invoke only after explicit user confirmation.","name":"SLACK_CREATE_CHANNEL","parameters":...
| {"description":"Creates a new public or private Slack channel with a unique name; the channel can be org-wide, or team-specific if `team_id` is given (required if `org_wide` is false or not provided).","name":"SLACK_CR...
| {"description":"Tool to create an Enterprise team in Slack. Use when you need to create a new team (workspace) within an Enterprise Grid organization. Requires admin.teams:write scope.","name":"SLACK_CREATE_ENTERPRISE_...
| {"description":"Creates a new User Group (often referred to as a subteam) in a Slack workspace.","name":"SLACK_CREATE_USER_GROUP","parameters":{"description":"Request schema for `CreateUserGroup`","properties":{"additi...
[... 30 row(s) omitted ... ⟦tj:533d7a087ff11a8e5cdf15928a563e20⟧]
| {"description":"Retrieves conversation preferences (e.g., who can post, who can thread) for a specified channel, primarily for use within Slack Enterprise Grid environments.","name":"SLACK_GET_CHANNEL_CONVERSATION_PREF...
| {"description":"Retrieves detailed information for an existing Slack reminder specified by its ID; this is a read-only operation.","name":"SLACK_GET_REMINDER","parameters":{"description":"Request schema for `GetReminde...
| {"description":"Retrieve information about a remote file added to Slack via the files.remote API. Does not work for standard Slack-hosted file uploads.","name":"SLACK_GET_REMOTE_FILE","parameters":{"description":"Reque...
| {"description":"Retrieves all profile field definitions for a Slack team, optionally filtered by visibility, to understand the team's profile structure.","name":"SLACK_GET_TEAM_PROFILE","parameters":{"description":"Req...
| {"description":"Retrieves a user's current Do Not Disturb status.","name":"SLACK_GET_USER_DND_STATUS","parameters":{"description":"Request schema for `GetUserDndStatus`","properties":{"team_id":{"description":"The work...
| {"description":"Retrieves a Slack user's current real-time presence (e.g., 'active', 'away') to determine their availability, noting this action does not provide historical data or status reasons.","name":"SLACK_GET_US...
| {"description":"Tool to get all workspaces a channel is connected to within an Enterprise org. Use when you need to determine which workspaces have access to a specific public or private channel in an Enterprise Grid o...
| {"description":"Retrieves detailed settings for a specific Slack workspace, primarily for administrators in an Enterprise Grid organization to view or audit workspace configurations.","name":"SLACK_GET_WORKSPACE_SETTIN...
| {"description":"Invites users to an existing Slack channel using their valid Slack User IDs. Response is always HTTP 200; inspect `ok`, `error`, and `errors` fields to confirm users were added.","name":"SLACK_INVITE_US...
| {"description":"Invites users to a specified Slack channel; this action is restricted to Enterprise Grid workspaces and requires the authenticated user to be a member of the target channel.","name":"SLACK_INVITE_USER_T...
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]


```

### `01-github-tools-array`

- [Full input](cases/01-github-tools-array/input.json)
- [Full output](cases/01-github-tools-array/output.md)
- [Input vs output diff](cases/01-github-tools-array/compression.diff)

Input excerpt:

```json
[
  {
    "function": {
      "description": "Tool to abort a repository migration that is queued or in progress. Use when you need to cancel an ongoing migration operation.",
      "name": "GITHUB_ABORT_REPOSITORY_MIGRATION",
      "parameters": {
        "description": "Request parameters for aborting a repository migration.",
        "properties": {
          "clientMutationId": {
            "description": "A unique identifier for the client performing the mutation. This can be used to track the mutation in logs or for idempotency purposes.",
            "examples": [
              "mutation-12345",
              "abort-migration-xyz"
            ],
            "title": "Client Mutation Id",
            "type": "string"
          },
          "migrationId": {
            "description": "The ID of the repository migration to abort. This is the migration ID returned when a migration was queued (e.g., 'RM_kgDOABCDEF').",
            "examples": [
              "RM_kgDOABCDEF",
              "RM_kgDOGw8pTA"
            ],
            "title": "Migration Id",
            "type": "string"
          }
        },
        "required": [
          "migrationId"
        ],
        "title": "AbortRepositoryMigrationRequest",
        "type": "object"
      }
    },
    "type": "function"
  },

```

Output excerpt:

```markdown
[json table: 60 rows × 2 cols · blank=absent key · exact original via retrieve footer]
| function | type |
| --- | --- |
| {"description":"Tool to abort a repository migration that is queued or in progress. Use when you need to cancel an ongoing migration operation.","name":"GITHUB_ABORT_REPOSITORY_MIGRATION","parameters":{"description":"R...
| {"description":"Accepts a PENDING repository invitation that has been issued to the authenticated user.","name":"GITHUB_ACCEPT_REPOSITORY_INVITATION","parameters":{"description":"Request schema for accepting a reposito...
| {"description":"Adds GitHub Apps to the list of apps allowed to push to a protected branch. The branch must already have protection rules with restrictions enabled. This endpoint only works for organization repositorie...
| {"description":"Adds a GitHub user as a repository collaborator, or updates their permission if already a collaborator; `permission` applies to organization-owned repositories (personal ones default to 'push' and ignor...
| {"description":"Adds assignees to a GitHub issue. This action only adds users - it does not remove existing assignees. Changes are silently ignored if the authenticated user lacks push access to the repository.","name"...
| {"description":"Adds one or more email addresses (which will be initially unverified) to the authenticated user's GitHub account; use this to associate new emails, noting an email verified for another account will erro...
| {"description":"Tool to add a custom field to a user-owned GitHub Projects V2 project. Use when you need to add fields like status, priority, or custom data to organize project items.","name":"GITHUB_ADD_FIELD_TO_USER_...
| {"description":"Tool to add an issue or pull request to a user-owned GitHub project. Use when you need to add existing repository items to a project board.","name":"GITHUB_ADD_ITEM_TO_USER_PROJECT","parameters":{"prope...
| {"description":"Adds labels (provided in the request body) to a repository issue; labels that do not already exist are created.","name":"GITHUB_ADD_LABELS_TO_AN_ISSUE","parameters":{"description":"Specifies the reposit...
| {"description":"Adds new custom labels to an existing self-hosted runner for an organization; existing labels are not removed, and duplicates are not added.","name":"GITHUB_ADD_ORG_RUNNER_LABELS","parameters":{"descrip...
| {"description":"Adds a GitHub user to a team or updates their role (member or maintainer), inviting them to the organization if not already a member; idempotent, returning current details if no change is made.","name":...
| {"description":"Adds a classic project to a team or updates the team's permission on it. This endpoint grants or updates permissions for a team on a specific classic project (not Projects V2). The authenticated user mu...
| {"description":"Sets or updates a team's permission level for a repository within an organization; the team must be a member of the organization.","name":"GITHUB_ADD_OR_UPDATE_TEAM_REPOSITORY_PERMISSIONS","parameters":...
| {"description":"Adds a specified GitHub user as a collaborator to an existing organization project with a given permission level. Note: This endpoint is for organization projects (classic). You must be an organization ...
| {"description":"Adds a repository to a GitHub App installation, granting the app access; requires authenticated user to have admin rights for the repository and access to the installation.","name":"GITHUB_ADD_REPOSITOR...
| {"description":"Adds a repository to an existing organization-level GitHub Actions secret that is configured for 'selected' repository access.","name":"GITHUB_ADD_REPO_TO_ORG_SECRET_WITH_SELECTED_ACCESS","parameters":{...
| {"description":"Grants an existing repository access to an existing organization-level Dependabot secret when the secret's visibility is set to 'selected'; the repository must belong to the organization, and the call s...
| {"description":"Adds and appends custom labels to a self-hosted repository runner, which must be registered and active.","name":"GITHUB_ADD_RUNNER_LABELS","parameters":{"description":"Request schema for `AddCustomLabel...
| {"description":"Adds a repository to an organization secret's access list when the secret's visibility is 'selected'; this operation is idempotent.","name":"GITHUB_ADD_SELECTED_REPOSITORY_TO_ORGANIZATION_SECRET","param...
| {"description":"Grants a repository access to an organization-level GitHub Actions variable, if that variable's visibility is set to 'selected_repositories'.","name":"GITHUB_ADD_SELECTED_REPOSITORY_TO_ORGANIZATION_VARI...
[... 30 row(s) omitted ... ⟦tj:fb9a60a5829a7520e1e00170f8f2b747⟧]
| {"description":"Checks if a GitHub App or OAuth access_token is valid for the specified client_id and retrieves its details, typically to verify its active status and grants. NOTE: This endpoint requires Basic Authenti...
| {"description":"Tool to clear the value of a field for an item in a GitHub Project V2. Use when you need to remove or reset a field value (text, number, date, assignees, labels, single-select, iteration, or milestone f...
| {"description":"Deletes GitHub Actions caches from a repository matching a specific `key` and an optional Git `ref`, used to manage storage or clear outdated/corrupted caches; the action succeeds even if no matching ca...
| {"description":"Removes all custom labels from a self-hosted runner for an organization; default labels (e.g., 'self-hosted', 'linux', 'x64') will remain.","name":"GITHUB_CLEAR_SELF_HOSTED_RUNNER_ORG_LABELS","parameter...
| {"description":"Tool to atomically create, update, or delete multiple files in a GitHub repository as a single commit. Uses Git Data APIs to avoid SHA mismatch conflicts that occur with the Contents API when multiple f...
| {"description":"Compares two commit points (commits, branches, tags, or SHAs) within a repository or across forks, using `BASE...HEAD` or `OWNER:REF...OWNER:REF` format for the `basehead` parameter.","name":"GITHUB_COM...
| {"description":"Generates a JIT configuration for a GitHub organization's new self-hosted runner to run a single job then unregister; requires admin:org scope and the runner_group_id must exist in the organization.","n...
| {"description":"Sets or updates the OIDC subject claim customization template for an existing GitHub organization by specifying which claims (e.g., 'repo', 'actor') form the OIDC token's subject (`sub`). This action cu...
| {"description":"Converts an existing organization member, who is not an owner, to an outside collaborator, restricting their access to explicitly granted repositories.","name":"GITHUB_CONVERT_ORG_MEMBER_TO_OUTSIDE_COLL...
| {"description":"Creates a Git blob in a repository, requiring content and encoding ('utf-8' or 'base64'). Requires write access to the repository.","name":"GITHUB_CREATE_A_BLOB","parameters":{"description":"Defines the...
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]


```

### `10-cargo-metadata`

- [Full input](cases/10-cargo-metadata/input.json)
- [Full output](cases/10-cargo-metadata/output.md)
- [Input vs output diff](cases/10-cargo-metadata/compression.diff)

Input excerpt:

```json
{"packages":[{"name":"openhuman","version":"0.58.11","id":"path+file://<OPENHUMAN_ROOT>#openhuman@0.58.11","license":null,"license_file":null,"description":"OpenHuman core business logic and RPC server","source":null,"de...

```

Output excerpt:

```markdown
{"packages":[{"name":"openhuman","version":"0.58.11","id":"path+file://<OPENHUMAN_ROOT>#openhuman@0.58.11","license":null,"license_file":null,"description":"OpenHuman core business logic and RPC server","source":null,"de...

```

### `09-package-manifest`

- [Full input](cases/09-package-manifest/input.json)
- [Full output](cases/09-package-manifest/output.md)
- [Input vs output diff](cases/09-package-manifest/compression.diff)

Input excerpt:

```json
{
  "name": "openhuman-app",
  "version": "0.58.11",
  "type": "module",
  "engines": {
    "node": ">=24.0.0"
  },
  "scripts": {
    "dev": "vite",
    "dev:web": "vite",
    "dev:app": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && bash ../scripts/setup-chromium-safe-storage.sh && source ../scripts/load-dotenv.sh && APPLE_SIGNING_IDENTITY='OpenHuman Dev Signe...
    "dev:app:win": "\"C:/Program Files/Git/bin/bash.exe\" ../scripts/run-dev-win.sh",
    "dev:cef": "pnpm dev:app",
    "dev:wry": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && source ../scripts/load-dotenv.sh && cargo tauri dev --no-default-features --features wry",
    "core:stage": "echo '[core:stage] no-op — core is linked in-process; sidecar removed (PR #1061)'",
    "tauri:ensure": "bash ../scripts/ensure-tauri-cli.sh",
    "tauri:ios:init": "bash ../scripts/ios-init.sh",
    "tauri:ios:dev": "cd src-tauri-mobile && IPHONEOS_DEPLOYMENT_TARGET=${IPHONEOS_DEPLOYMENT_TARGET:-16.0} npx --package=@tauri-apps/cli@^2 tauri ios dev",
    "tauri:ios:build": "cd src-tauri-mobile && IPHONEOS_DEPLOYMENT_TARGET=${IPHONEOS_DEPLOYMENT_TARGET:-16.0} npx --package=@tauri-apps/cli@^2 tauri ios build",
    "tauri:android:init": "bash ../scripts/android-init.sh",
    "tauri:android:dev": "cd src-tauri-mobile && ../node_modules/.bin/tauri android dev",
    "tauri:android:build": "cd src-tauri-mobile && ../node_modules/.bin/tauri android build",
    "release:android:play": "bash ../scripts/release/upload-android-to-play.sh",
    "build": "tsc && vite build",
    "build:app": "tsc && vite build",
    "build:app:e2e": "tsc && vite build --mode development",
    "build:web:e2e": "bash ./scripts/e2e-web-build.sh",
    "build:web": "cross-env VITE_OPENHUMAN_TARGET=web tsc && cross-env VITE_OPENHUMAN_TARGET=web vite build",
    "compile": "tsc --noEmit",
    "preview": "vite preview",
    "tauri": "tauri",
    "tauri:build:ui": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && cargo tauri build -- --bin OpenHuman",
    "macos:build:intel": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && source ../scripts/load-dotenv.sh && cargo tauri build --bundles app dmg --target x86_64-apple-darwin -- --bin OpenHuman...
    "macos:build:intel:debug": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && source ../scripts/load-dotenv.sh && cargo tauri build --debug --bundles app dmg --target x86_64-apple-darwin -- -...
    "macos:build:debug": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && source ../scripts/load-dotenv.sh && cargo tauri build --debug --bundles app dmg -- --bin OpenHuman",
    "macos:build:release": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && source ../scripts/load-dotenv.sh && cargo tauri build --bundles app dmg -- --bin OpenHuman",

```

Output excerpt:

```markdown
{
  "name": "openhuman-app",
  "version": "0.58.11",
  "type": "module",
  "engines": {
    "node": ">=24.0.0"
  },
  "scripts": {
    "dev": "vite",
    "dev:web": "vite",
    "dev:app": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && bash ../scripts/setup-chromium-safe-storage.sh && source ../scripts/load-dotenv.sh && APPLE_SIGNING_IDENTITY='OpenHuman Dev Signe...
    "dev:app:win": "\"C:/Program Files/Git/bin/bash.exe\" ../scripts/run-dev-win.sh",
    "dev:cef": "pnpm dev:app",
    "dev:wry": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && source ../scripts/load-dotenv.sh && cargo tauri dev --no-default-features --features wry",
    "core:stage": "echo '[core:stage] no-op — core is linked in-process; sidecar removed (PR #1061)'",
    "tauri:ensure": "bash ../scripts/ensure-tauri-cli.sh",
    "tauri:ios:init": "bash ../scripts/ios-init.sh",
    "tauri:ios:dev": "cd src-tauri-mobile && IPHONEOS_DEPLOYMENT_TARGET=${IPHONEOS_DEPLOYMENT_TARGET:-16.0} npx --package=@tauri-apps/cli@^2 tauri ios dev",
    "tauri:ios:build": "cd src-tauri-mobile && IPHONEOS_DEPLOYMENT_TARGET=${IPHONEOS_DEPLOYMENT_TARGET:-16.0} npx --package=@tauri-apps/cli@^2 tauri ios build",
    "tauri:android:init": "bash ../scripts/android-init.sh",
    "tauri:android:dev": "cd src-tauri-mobile && ../node_modules/.bin/tauri android dev",
    "tauri:android:build": "cd src-tauri-mobile && ../node_modules/.bin/tauri android build",
    "release:android:play": "bash ../scripts/release/upload-android-to-play.sh",
    "build": "tsc && vite build",
    "build:app": "tsc && vite build",
    "build:app:e2e": "tsc && vite build --mode development",
    "build:web:e2e": "bash ./scripts/e2e-web-build.sh",
    "build:web": "cross-env VITE_OPENHUMAN_TARGET=web tsc && cross-env VITE_OPENHUMAN_TARGET=web vite build",
    "compile": "tsc --noEmit",
    "preview": "vite preview",
    "tauri": "tauri",
    "tauri:build:ui": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && cargo tauri build -- --bin OpenHuman",
    "macos:build:intel": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && source ../scripts/load-dotenv.sh && cargo tauri build --bundles app dmg --target x86_64-apple-darwin -- --bin OpenHuman...
    "macos:build:intel:debug": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && source ../scripts/load-dotenv.sh && cargo tauri build --debug --bundles app dmg --target x86_64-apple-darwin -- -...
    "macos:build:debug": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && source ../scripts/load-dotenv.sh && cargo tauri build --debug --bundles app dmg -- --bin OpenHuman",
    "macos:build:release": "pnpm tauri:ensure && export CEF_PATH=\"$HOME/Library/Caches/tauri-cef\" && source ../scripts/load-dotenv.sh && cargo tauri build --bundles app dmg -- --bin OpenHuman",

```

### `08-lottie-animation`

- [Full input](cases/08-lottie-animation/input.json)
- [Full output](cases/08-lottie-animation/output.md)
- [Input vs output diff](cases/08-lottie-animation/compression.diff)

Input excerpt:

```json
{
  "v": "4.8.0",
  "meta": {
    "g": "LottieFiles AE 3.0.2",
    "a": "",
    "k": "",
    "d": "",
    "tc": ""
  },
  "fr": 60,
  "ip": 0,
  "op": 77,
  "w": 500,
  "h": 500,
  "nm": "security tick",
  "ddd": 0,
  "assets": [],
  "layers": [
    {
      "ddd": 0,
      "ind": 1,
      "ty": 4,
      "nm": "tick",
      "sr": 1,
      "ks": {
        "o": {
          "a": 0,
          "k": 100,
          "ix": 11
        },
        "r": {
          "a": 0,
          "k": 0,
          "ix": 10
        },
        "p": {

```

Output excerpt:

```markdown
{
  "v": "4.8.0",
  "meta": {
    "g": "LottieFiles AE 3.0.2",
    "a": "",
    "k": "",
    "d": "",
    "tc": ""
  },
  "fr": 60,
  "ip": 0,
  "op": 77,
  "w": 500,
  "h": 500,
  "nm": "security tick",
  "ddd": 0,
  "assets": [],
  "layers": [
    {
      "ddd": 0,
      "ind": 1,
      "ty": 4,
      "nm": "tick",
      "sr": 1,
      "ks": {
        "o": {
          "a": 0,
          "k": 100,
          "ix": 11
        },
        "r": {
          "a": 0,
          "k": 0,
          "ix": 10
        },
        "p": {

```

### `07-app-schema-object`

- [Full input](cases/07-app-schema-object/input.json)
- [Full output](cases/07-app-schema-object/output.md)
- [Input vs output diff](cases/07-app-schema-object/compression.diff)

Input excerpt:

```json
{
  "methods": [
    {
      "description": "Liveness probe for the core JSON-RPC server.",
      "function": "ping",
      "inputs": [],
      "method": "core.ping",
      "namespace": "core",
      "outputs": [
        {
          "comment": "Always true when the server is reachable.",
          "name": "ok",
          "required": true,
          "ty": "Bool"
        }
      ]
    },
    {
      "description": "Lists all JSON-RPC methods and their input/output schemas.",
      "function": "rpc_schema_dump",
      "inputs": [],
      "method": "core.rpc_schema_dump",
      "namespace": "core",
      "outputs": [
        {
          "comment": "All JSON-RPC method schemas available to clients.",
          "name": "methods",
          "required": true,
          "ty": {
            "Array": {
              "Object": {
                "fields": [
                  {
                    "comment": "Fully-qualified JSON-RPC method name.",
                    "name": "method",
                    "required": true,

```

Output excerpt:

```markdown
{
  "methods": [
    {
      "description": "Liveness probe for the core JSON-RPC server.",
      "function": "ping",
      "inputs": [],
      "method": "core.ping",
      "namespace": "core",
      "outputs": [
        {
          "comment": "Always true when the server is reachable.",
          "name": "ok",
          "required": true,
          "ty": "Bool"
        }
      ]
    },
    {
      "description": "Lists all JSON-RPC methods and their input/output schemas.",
      "function": "rpc_schema_dump",
      "inputs": [],
      "method": "core.rpc_schema_dump",
      "namespace": "core",
      "outputs": [
        {
          "comment": "All JSON-RPC method schemas available to clients.",
          "name": "methods",
          "required": true,
          "ty": {
            "Array": {
              "Object": {
                "fields": [
                  {
                    "comment": "Fully-qualified JSON-RPC method name.",
                    "name": "method",
                    "required": true,

```

### `06-tauri-capabilities-schema`

- [Full input](cases/06-tauri-capabilities-schema/input.json)
- [Full output](cases/06-tauri-capabilities-schema/output.md)
- [Input vs output diff](cases/06-tauri-capabilities-schema/compression.diff)

Input excerpt:

```json
{
  "default": {
    "identifier": "default",
    "description": "Capability for the main and overlay windows (desktop only)",
    "local": true,
    "windows": [
      "main",
      "overlay"
    ],
    "permissions": [
      "core:default",
      "core:window:default",
      "core:window:allow-hide",
      "core:window:allow-show",
      "core:window:allow-set-focus",
      "core:window:allow-unminimize",
      "core:window:allow-start-dragging",
      "core:window:allow-set-always-on-top",
      "core:event:default",
      "deep-link:default",
      "notification:default",
      "notification:allow-is-permission-granted",
      "notification:allow-request-permission",
      "notification:allow-notify",
      "opener:default",
      "opener:allow-reveal-item-in-dir",
      {
        "identifier": "opener:allow-open-url",
        "allow": [
          {
            "url": "obsidian://open*"
          },
          {
            "url": "https://ollama.com/*"
          }
        ]

```

Output excerpt:

```markdown
{
  "default": {
    "identifier": "default",
    "description": "Capability for the main and overlay windows (desktop only)",
    "local": true,
    "windows": [
      "main",
      "overlay"
    ],
    "permissions": [
      "core:default",
      "core:window:default",
      "core:window:allow-hide",
      "core:window:allow-show",
      "core:window:allow-set-focus",
      "core:window:allow-unminimize",
      "core:window:allow-start-dragging",
      "core:window:allow-set-always-on-top",
      "core:event:default",
      "deep-link:default",
      "notification:default",
      "notification:allow-is-permission-granted",
      "notification:allow-request-permission",
      "notification:allow-notify",
      "opener:default",
      "opener:allow-reveal-item-in-dir",
      {
        "identifier": "opener:allow-open-url",
        "allow": [
          {
            "url": "obsidian://open*"
          },
          {
            "url": "https://ollama.com/*"
          }
        ]

```

### `05-polymarket-events-list`

- [Full input](cases/05-polymarket-events-list/input.json)
- [Full output](cases/05-polymarket-events-list/output.md)
- [Input vs output diff](cases/05-polymarket-events-list/compression.diff)

Input excerpt:

```json
[
  {
    "id": "event-1",
    "title": "Ethereum milestones",
    "slug": "ethereum-milestones"
  },
  {
    "id": "event-2",
    "title": "Bitcoin milestones",
    "slug": "bitcoin-milestones"
  }
]

```

Output excerpt:

```markdown
[
  {
    "id": "event-1",
    "title": "Ethereum milestones",
    "slug": "ethereum-milestones"
  },
  {
    "id": "event-2",
    "title": "Bitcoin milestones",
    "slug": "bitcoin-milestones"
  }
]

```

### `04-polymarket-markets-list`

- [Full input](cases/04-polymarket-markets-list/input.json)
- [Full output](cases/04-polymarket-markets-list/output.md)
- [Input vs output diff](cases/04-polymarket-markets-list/compression.diff)

Input excerpt:

```json
[
  {
    "id": "12345",
    "slug": "will-eth-hit-10k",
    "question": "Will ETH hit $10k by Dec 31, 2026?",
    "active": true,
    "closed": false
  },
  {
    "id": "67890",
    "slug": "will-btc-hit-200k",
    "question": "Will BTC hit $200k by Dec 31, 2026?",
    "active": true,
    "closed": false
  }
]

```

Output excerpt:

```markdown
[
  {
    "id": "12345",
    "slug": "will-eth-hit-10k",
    "question": "Will ETH hit $10k by Dec 31, 2026?",
    "active": true,
    "closed": false
  },
  {
    "id": "67890",
    "slug": "will-btc-hit-200k",
    "question": "Will BTC hit $200k by Dec 31, 2026?",
    "active": true,
    "closed": false
  }
]

```

