# JSON SmartCrusher

Real OpenHuman JSON snapshots: tool catalogs, API responses, schemas, package metadata, Lottie payloads, and config files. TinyJuice now chooses the smallest useful JSON representation before CCR, using Markdown tables only when they beat minified JSON.

Each row links to the full raw input and both compacted outputs. Percentages are **token reduction: higher is better**; 0% means pass-through. `Bytes` shows the raw input size -> compressor-only output size and its byte reduction. `Pass 1` disables CCR and is **lossless by construction**: faithful reshapes (JSON tables/minify, HTML->text) still ship because nothing is lost, but anything that *drops* detail (log lines, diff context, search matches, code bodies, sampled JSON rows) passes the original through untouched, since without the cache it could not be recovered. `Pass 2` enables CCR, so information-dropping compression is allowed — every dropped block is offloaded behind a retrieval token. Faithful reshapes (HTML->text) are identical in both passes (Pass 2 is marginally lower only for the recovery footer); pure information-dropping categories (diffs, search, code) are 0% in Pass 1 and compress only in Pass 2. Two categories are hybrids that compress losslessly in Pass 1 and further in Pass 2: JSON renders the full lossless markdown table in Pass 1 (all rows) then samples the middle away in Pass 2; logs collapse runs of byte-identical lines to `line [x N]` in Pass 1 then drop low-signal lines in Pass 2. Each pass links its own output and its own diff against the input.

## Cases

Every case links to the raw input; each pass column carries its percentage plus that pass's exact output and a unified diff against the input.

| Case | Input | Bytes | Pass 1: no CCR | Pass 2: with CCR | Avg latency |
| --- | --- | ---: | ---: | ---: | ---: |
| `02-notion-tools-array` | [input](cases/02-notion-tools-array/input.json) | 890.8 KB -> 16.0 KB (-98%) | 61.1%<br>[output](cases/02-notion-tools-array/output-noccr.md) - [diff](cases/02-notion-tools-array/compression-noccr.diff) | 98.2%<br>[output](cases/02-notion-tools-array/output.md) - [diff](cases/02-notion-tools-array/compression.diff) | 8.651 ms |
| `03-slack-tools-array` | [input](cases/03-slack-tools-array/input.json) | 106.7 KB -> 13.8 KB (-87%) | 32.1%<br>[output](cases/03-slack-tools-array/output-noccr.md) - [diff](cases/03-slack-tools-array/compression-noccr.diff) | 87.0%<br>[output](cases/03-slack-tools-array/output.md) - [diff](cases/03-slack-tools-array/compression.diff) | 1.885 ms |
| `01-github-tools-array` | [input](cases/01-github-tools-array/input.json) | 110.9 KB -> 15.1 KB (-86%) | 34.8%<br>[output](cases/01-github-tools-array/output-noccr.md) - [diff](cases/01-github-tools-array/compression-noccr.diff) | 86.3%<br>[output](cases/01-github-tools-array/output.md) - [diff](cases/01-github-tools-array/compression.diff) | 1.882 ms |
| `07-app-schema-object` | [input](cases/07-app-schema-object/input.json) | 48.6 KB -> 8.9 KB (-82%) | 48.7%<br>[output](cases/07-app-schema-object/output-noccr.md) - [diff](cases/07-app-schema-object/compression-noccr.diff) | 81.5%<br>[output](cases/07-app-schema-object/output.md) - [diff](cases/07-app-schema-object/compression.diff) | 0.729 ms |
| `10-cargo-metadata` | [input](cases/10-cargo-metadata/input.json) | 62.0 KB -> 62.0 KB (-0%) | 0.0%<br>[output](cases/10-cargo-metadata/output-noccr.md) - [diff](cases/10-cargo-metadata/compression-noccr.diff) | 0.0%<br>[output](cases/10-cargo-metadata/output.md) - [diff](cases/10-cargo-metadata/compression.diff) | 0.153 ms |
| `09-package-manifest` | [input](cases/09-package-manifest/input.json) | 9.4 KB -> 9.4 KB (-0%) | 0.0%<br>[output](cases/09-package-manifest/output-noccr.md) - [diff](cases/09-package-manifest/compression-noccr.diff) | 0.0%<br>[output](cases/09-package-manifest/output.md) - [diff](cases/09-package-manifest/compression.diff) | 0.031 ms |
| `08-lottie-animation` | [input](cases/08-lottie-animation/input.json) | 16.8 KB -> 16.8 KB (-0%) | 0.0%<br>[output](cases/08-lottie-animation/output-noccr.md) - [diff](cases/08-lottie-animation/compression-noccr.diff) | 0.0%<br>[output](cases/08-lottie-animation/output.md) - [diff](cases/08-lottie-animation/compression.diff) | 0.030 ms |
| `06-tauri-capabilities-schema` | [input](cases/06-tauri-capabilities-schema/input.json) | 2.4 KB -> 2.4 KB (-0%) | 0.0%<br>[output](cases/06-tauri-capabilities-schema/output-noccr.md) - [diff](cases/06-tauri-capabilities-schema/compression-noccr.diff) | 0.0%<br>[output](cases/06-tauri-capabilities-schema/output.md) - [diff](cases/06-tauri-capabilities-schema/compression.diff) | 0.005 ms |
| `05-polymarket-events-list` | [input](cases/05-polymarket-events-list/input.json) | 201 B -> 201 B (-0%) | 0.0%<br>[output](cases/05-polymarket-events-list/output-noccr.md) - [diff](cases/05-polymarket-events-list/compression-noccr.diff) | 0.0%<br>[output](cases/05-polymarket-events-list/output.md) - [diff](cases/05-polymarket-events-list/compression.diff) | 0.000 ms |
| `04-polymarket-markets-list` | [input](cases/04-polymarket-markets-list/input.json) | 313 B -> 313 B (-0%) | 0.0%<br>[output](cases/04-polymarket-markets-list/output-noccr.md) - [diff](cases/04-polymarket-markets-list/compression-noccr.diff) | 0.0%<br>[output](cases/04-polymarket-markets-list/output.md) - [diff](cases/04-polymarket-markets-list/compression.diff) | 0.000 ms |

## What TinyJuice Is Doing

TinyJuice parses JSON before choosing a representation. Homogeneous object arrays can become GitHub-renderable Markdown tables, but minified JSON wins when it is smaller or when the JSON shape is too nested for a readable table. If neither representation saves space, the router returns the original.

## Syntax-Aware Samples

### `02-notion-tools-array`

- [Full input](cases/02-notion-tools-array/input.json)
- [Output with CCR](cases/02-notion-tools-array/output.md) - [diff](cases/02-notion-tools-array/compression.diff)
- [Output without CCR](cases/02-notion-tools-array/output-noccr.md) - [diff](cases/02-notion-tools-array/compression-noccr.diff)

Input excerpt:

```json
[
  {
    "function": {
      "description": "Bulk-add content blocks to Notion. Text >2000 chars auto-splits. Parses markdown formatting. ⚠️ PARENT BLOCK TYPES: Content is added AS CHILDREN of parent_block_id. - To add content AFTER a heading,...
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
            "description": "⚠️ CRITICAL: Notion API enforces 2000 char limit per text.content field. Content >2000 chars auto-splits.\nList of blocks to add (max 100). Also accepts 'blocks' as alias. Each item can be in ...
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
[all rows: type="function"]
[json table: 48 rows × 3 cols · blank=absent key · exact original via retrieve footer]
| function.name | function.description | function.… |
| --- | --- | --- |
| NOTION_ADD_MULTIPLE_PAGE_CONTENT | Bulk-add content blocks to Notion. Text >2000 chars auto-splits. Parses markdown formatting. ⚠️ PARENT BLOCK TYPES: Content is added AS CHILDREN of parent_block_id. - To add content A...
| NOTION_ADD_PAGE_CONTENT | DEPRECATED: Use 'add_multiple_page_content' for better performance. Adds a single content block to a Notion page/block. CRITICAL: Notion API enforces a HARD LIMIT of 2000 characters per text.c...
| NOTION_APPEND_BLOCK_CHILDREN | DEPRECATED: Use NOTION_APPEND_TEXT_BLOCKS, NOTION_APPEND_TASK_BLOCKS, NOTION_APPEND_CODE_BLOCKS, NOTION_APPEND_MEDIA_BLOCKS, NOTION_APPEND_LAYOUT_BLOCKS, or NOTION_APPEND_TABLE_BLOCKS ins...
| NOTION_APPEND_CODE_BLOCKS | Append code and technical blocks (code, quote, equation) to a Notion page. Use for: - Code snippets and programming examples (code) - Citations and highlighted quotes (quote) - Mathematical ...
| NOTION_APPEND_LAYOUT_BLOCKS | Append layout blocks (divider, TOC, breadcrumb, columns) to a Notion page. Supported types: - divider: Horizontal line separator - table_of_contents: Auto-generated from headings - breadcr...
| NOTION_APPEND_MEDIA_BLOCKS | Append media blocks (image, video, audio, file, pdf, embed, bookmark) to a Notion page. Use for: - Images and screenshots (image) - YouTube/Vimeo videos or direct video URLs (video) - Audio...
| NOTION_APPEND_TABLE_BLOCKS | Append table blocks to a Notion page. Use for structured tabular data like spreadsheets, comparison charts, and status trackers. Example: { "table_width": 3, "has_column_header": true, "row...
| NOTION_APPEND_TASK_BLOCKS | Append task blocks (to-do, toggle, callout) to a Notion page or block. Supported block types: - to_do: Checkbox items (checkable/uncheckable) - toggle: Collapsible sections - callout: Highli...
| NOTION_APPEND_TEXT_BLOCKS | Append text blocks (paragraphs, headings, lists) to a Notion page. This is the most commonly used action for adding content to Notion. Use for: documentation, notes, articles, outlines, list...
| NOTION_ARCHIVE_NOTION_PAGE | Archives (moves to trash) or unarchives (restores from trash) a specified Notion page. Limitation: Workspace-level pages (top-level pages with no parent page or database) cannot be archived...
| NOTION_CREATE_COMMENT | Adds a comment to a Notion page (via `parent_page_id`) OR to an existing discussion thread (via `discussion_id`); cannot create new discussion threads on specific blocks (inline comments). | {"p...
| NOTION_CREATE_DATABASE | Creates a new Notion database as a subpage under a specified parent page with a defined properties schema. IMPORTANT NOTES: - The parent page MUST be shared with your integration, otherwise you...
| NOTION_CREATE_FILE_UPLOAD | Tool to create a Notion FileUpload object and retrieve an upload URL. Use when you need to automate attaching local or external files directly into Notion without external hosting. | {"param...
| NOTION_CREATE_NOTION_PAGE | Creates a new page in a Notion workspace under a specified parent page or database. Supports creating pages with markdown content using the native markdown parameter, or as an empty page tha...
| NOTION_DELETE_BLOCK | Archives a Notion block, page, or database using its ID, which sets its 'archived' property to true (like moving to "Trash" in the UI) and allows it to be restored later. Note: This operation will...
| NOTION_DUPLICATE_PAGE | Duplicates a Notion page, including all its content, properties, and nested blocks, under a specified parent page or workspace. | {"parameters":{"description":"Defines the parameters for duplica...
| NOTION_FETCH_ALL_BLOCK_CONTENTS | Tool to fetch all child blocks for a given Notion block. Use when you need a complete listing of a block's children beyond a single page; supports optional recursive expansion of neste...
| NOTION_FETCH_BLOCK_CONTENTS | Retrieves a paginated list of direct, first-level child block objects along with contents for a given parent Notion block or page ID; use block IDs from the response for subsequent calls t...
| NOTION_FETCH_BLOCK_METADATA | Fetches metadata for a Notion block (including pages, which are special blocks) using its UUID. Returns block type, properties, and basic info but not child content. Prerequisites: 1) Bloc...
| NOTION_FETCH_COMMENTS | Fetches unresolved comments for a specified Notion block or page ID. The block/page must be shared with your Notion integration and the integration must have 'Read comments' capability enabled, ...
[... 7 row(s) omitted ... ⟦tj:d59d9156632736f6bbc04659eb5082b0⟧]
| NOTION_INSERT_ROW_DATABASE | Creates a new page (row) in a specified Notion database. Prerequisites: - Database must be shared with your integration - Property names AND types must match schema exactly (case-sensitive)...
[... 10 row(s) omitted ... ⟦tj:c0baa5579936dc34fa7722439268f6aa⟧]
| NOTION_RETRIEVE_DATABASE_PROPERTY | Tool to retrieve a specific property object of a Notion database. Use when you need to get details about a single database column/property. | {"parameters":{"properties":{"database_i...
| NOTION_RETRIEVE_FILE_UPLOAD | Tool to retrieve details of a Notion File Upload object by its identifier. Use when you need to check the status or details of an existing file upload. | {"parameters":{"properties":{"file...
| NOTION_RETRIEVE_PAGE | Retrieve a Notion page's properties/metadata (not block content) by page_id. Use when you have a page URL/ID and need to access its properties; for page content use block-children tools. | {"para...
| NOTION_SEARCH_NOTION_PAGE | Searches Notion pages and databases by title. Use specific search terms to find items by title (primary approach). KNOWN LIMITATIONS: (1) Search indexing is not immediate - recently shared i...
| NOTION_SEND_FILE_UPLOAD | Tool to transmit file contents to Notion for a file upload object. Use after creating a file upload object to send the actual file data. | {"parameters":{"properties":{"file":{"description":"F...
| NOTION_UPDATE_BLOCK | Updates existing Notion block's text content. ⚠️ CRITICAL: Content limited to 2000 chars. Cannot change block type or archive blocks. Content exceeding 2000 chars will fail with validation error. ...
| NOTION_UPDATE_PAGE | Update page properties, icon, cover, or archive status. IMPORTANT: Property names are workspace-specific and case-sensitive. Use NOTION_FETCH_ROW or NOTION_FETCH_DATABASE first to discover exact pr...
| NOTION_UPDATE_ROW_DATABASE | Updates a specific row/page within a Notion database by its page UUID (row_id). IMPORTANT CLARIFICATION: This action updates INDIVIDUAL ROWS (pages) in a database, NOT the database structur...
| NOTION_UPDATE_SCHEMA_DATABASE | Updates an existing Notion database's schema including title, description, and/or properties (columns). IMPORTANT NOTES: - At least one update (title, description, or properties) must be...

```

### `07-app-schema-object`

- [Full input](cases/07-app-schema-object/input.json)
- [Output with CCR](cases/07-app-schema-object/output.md) - [diff](cases/07-app-schema-object/compression.diff)
- [Output without CCR](cases/07-app-schema-object/output-noccr.md) - [diff](cases/07-app-schema-object/compression-noccr.diff)

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
[json table: methods — 69 rows × 6 cols · blank=absent key · exact original via retrieve footer]
| description | function | inputs | method | namespace | outputs |
| --- | --- | --- | --- | --- | --- |
| Liveness probe for the core JSON-RPC server. | ping | [] | core.ping | core | [{"comment":"Always true when the server is reachable.","name":"ok","required":true,"ty":"Bool"}] |
| Lists all JSON-RPC methods and their input/output schemas. | rpc_schema_dump | [] | core.rpc_schema_dump | core | [{"comment":"All JSON-RPC method schemas available to clients.","name":"methods","required":true,"ty":{"...
| Returns the core binary version. | version | [] | core.version | core | [{"comment":"Semantic version string for the running core binary.","name":"version","required":true,"ty":"String"}] |
| Run one-shot agent chat with optional model overrides. | chat | [{"comment":"User message.","name":"message","required":true,"ty":"String"},{"comment":"Optional model override.","name":"model_override","required":false...
| Run one-shot lightweight provider chat. | chat_simple | [{"comment":"User message.","name":"message","required":true,"ty":"String"},{"comment":"Optional model override.","name":"model_override","required":false,"ty":{"...
| Terminate REPL session. | repl_session_end | [{"comment":"REPL session id.","name":"session_id","required":true,"ty":"String"}] | openhuman.agent_repl_session_end | agent | [{"comment":"Session end result.","name":"res...
| Clear REPL session history. | repl_session_reset | [{"comment":"REPL session id.","name":"session_id","required":true,"ty":"String"}] | openhuman.agent_repl_session_reset | agent | [{"comment":"Session reset result.","...
| Create a persistent REPL agent session. | repl_session_start | [{"comment":"Optional session id.","name":"session_id","required":false,"ty":{"Option":"String"}},{"comment":"Optional model override.","name":"model_overr...
| Return core runtime URL and status for agent calls. | server_status | [] | openhuman.agent_server_status | agent | [{"comment":"Agent server status payload.","name":"status","required":true,"ty":"Json"}] |
| Remove stored app session credentials. | clear_session | [] | openhuman.auth_clear_session | auth | [{"comment":"Session clear result payload.","name":"result","required":true,"ty":"Json"}] |
| Consume login handoff token and return session JWT. | consume_login_token | [{"comment":"One-time login token.","name":"loginToken","required":true,"ty":"String"}] | openhuman.auth_consume_login_token | auth | [{"comme...
| Read stored app session token. | get_session_token | [] | openhuman.auth_get_session_token | auth | [{"comment":"Session token payload.","name":"token","required":true,"ty":"Json"}] |
| Get current auth/session state. | get_state | [] | openhuman.auth_get_state | auth | [{"comment":"Current auth state response.","name":"state","required":true,"ty":"Json"}] |
| List stored provider credentials. | list_provider_credentials | [{"comment":"Optional provider filter.","name":"provider","required":false,"ty":{"Option":"String"}}] | openhuman.auth_list_provider_credentials | auth | ...
| Create OAuth connect URL for provider. | oauth_connect | [{"comment":"Provider id.","name":"provider","required":true,"ty":"String"},{"comment":"Optional skill id.","name":"skillId","required":false,"ty":{"Option":"Str...
| Fetch integration handoff tokens. | oauth_fetch_integration_tokens | [{"comment":"Integration id.","name":"integrationId","required":true,"ty":"String"},{"comment":"Encryption key.","name":"key","required":true,"ty":"S...
| List OAuth integrations for current session. | oauth_list_integrations | [] | openhuman.auth_oauth_list_integrations | auth | [{"comment":"OAuth integration list.","name":"integrations","required":true,"ty":"Json"}] |
| Revoke OAuth integration. | oauth_revoke_integration | [{"comment":"Integration id.","name":"integrationId","required":true,"ty":"String"}] | openhuman.auth_oauth_revoke_integration | auth | [{"comment":"Integration re...
| Remove provider credentials for a profile. | remove_provider_credentials | [{"comment":"Provider id.","name":"provider","required":true,"ty":"String"},{"comment":"Optional profile name.","name":"profile","required":fal...
| Store provider credentials for a profile. | store_provider_credentials | [{"comment":"Provider id.","name":"provider","required":true,"ty":"String"},{"comment":"Optional profile name.","name":"profile","required":false...
[... 39 row(s) omitted ... ⟦tj:73f36bc4da58298cfeb98d74b7bd4d9d⟧]
| Read recent vision summaries. | vision_recent | [{"comment":"Maximum number of summaries.","name":"limit","required":false,"ty":{"Option":"U64"}}] | openhuman.screen_intelligence_vision_recent | screen_intelligence | [...
| Manage desktop service lifecycle. | install | [] | openhuman.service_install | service | [{"comment":"Service status payload.","name":"status","required":true,"ty":{"Ref":"ServiceStatus"}}] |
| Manage desktop service lifecycle. | start | [] | openhuman.service_start | service | [{"comment":"Service status payload.","name":"status","required":true,"ty":{"Ref":"ServiceStatus"}}] |
| Manage desktop service lifecycle. | status | [] | openhuman.service_status | service | [{"comment":"Service status payload.","name":"status","required":true,"ty":{"Ref":"ServiceStatus"}}] |
| Manage desktop service lifecycle. | stop | [] | openhuman.service_stop | service | [{"comment":"Service status payload.","name":"status","required":true,"ty":{"Ref":"ServiceStatus"}}] |
| Manage desktop service lifecycle. | uninstall | [] | openhuman.service_uninstall | service | [{"comment":"Service status payload.","name":"status","required":true,"ty":{"Ref":"ServiceStatus"}}] |
| Skill runtime socket manager bridge. | connect | [{"comment":"Socket request payload.","name":"payload","required":false,"ty":{"Option":"Json"}}] | openhuman.socket_connect | socket | [{"comment":"Socket response paylo...
| Skill runtime socket manager bridge. | disconnect | [{"comment":"Socket request payload.","name":"payload","required":false,"ty":{"Option":"Json"}}] | openhuman.socket_disconnect | socket | [{"comment":"Socket response...
| Skill runtime socket manager bridge. | emit | [{"comment":"Socket request payload.","name":"payload","required":false,"ty":{"Option":"Json"}}] | openhuman.socket_emit | socket | [{"comment":"Socket response payload.","...
| Skill runtime socket manager bridge. | state | [{"comment":"Socket request payload.","name":"payload","required":false,"ty":{"Option":"Json"}}] | openhuman.socket_state | socket | [{"comment":"Socket response payload."...
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]


```

### `01-github-tools-array`

- [Full input](cases/01-github-tools-array/input.json)
- [Output with CCR](cases/01-github-tools-array/output.md) - [diff](cases/01-github-tools-array/compression.diff)
- [Output without CCR](cases/01-github-tools-array/output-noccr.md) - [diff](cases/01-github-tools-array/compression-noccr.diff)

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
[all rows: type="function"]
[json table: 60 rows × 3 cols · blank=absent key · exact original via retrieve footer]
| function.name | function.description | function.… |
| --- | --- | --- |
| GITHUB_ABORT_REPOSITORY_MIGRATION | Tool to abort a repository migration that is queued or in progress. Use when you need to cancel an ongoing migration operation. | {"parameters":{"description":"Request parameters for...
| GITHUB_ACCEPT_REPOSITORY_INVITATION | Accepts a PENDING repository invitation that has been issued to the authenticated user. | {"parameters":{"description":"Request schema for accepting a repository invitation.","prop...
| GITHUB_ADD_APP_ACCESS_RESTRICTIONS | Adds GitHub Apps to the list of apps allowed to push to a protected branch. The branch must already have protection rules with restrictions enabled. This endpoint only works for org...
| GITHUB_ADD_A_REPOSITORY_COLLABORATOR | Adds a GitHub user as a repository collaborator, or updates their permission if already a collaborator; `permission` applies to organization-owned repositories (personal ones defa...
| GITHUB_ADD_ASSIGNEES_TO_AN_ISSUE | Adds assignees to a GitHub issue. This action only adds users - it does not remove existing assignees. Changes are silently ignored if the authenticated user lacks push access to the ...
| GITHUB_ADD_EMAIL_ADDRESS_FOR_AUTHENTICATED_USER | Adds one or more email addresses (which will be initially unverified) to the authenticated user's GitHub account; use this to associate new emails, noting an email veri...
| GITHUB_ADD_FIELD_TO_USER_PROJECT | Tool to add a custom field to a user-owned GitHub Projects V2 project. Use when you need to add fields like status, priority, or custom data to organize project items. | {"parameters"...
| GITHUB_ADD_ITEM_TO_USER_PROJECT | Tool to add an issue or pull request to a user-owned GitHub project. Use when you need to add existing repository items to a project board. | {"parameters":{"properties":{"id":{"descri...
| GITHUB_ADD_LABELS_TO_AN_ISSUE | Adds labels (provided in the request body) to a repository issue; labels that do not already exist are created. | {"parameters":{"description":"Specifies the repository issue to which la...
| GITHUB_ADD_ORG_RUNNER_LABELS | Adds new custom labels to an existing self-hosted runner for an organization; existing labels are not removed, and duplicates are not added. | {"parameters":{"description":"Request schema...
| GITHUB_ADD_OR_UPDATE_TEAM_MEMBERSHIP_FOR_USER | Adds a GitHub user to a team or updates their role (member or maintainer), inviting them to the organization if not already a member; idempotent, returning current detail...
| GITHUB_ADD_OR_UPDATE_TEAM_PROJECT_PERMISSIONS | Adds a classic project to a team or updates the team's permission on it. This endpoint grants or updates permissions for a team on a specific classic project (not Project...
| GITHUB_ADD_OR_UPDATE_TEAM_REPOSITORY_PERMISSIONS | Sets or updates a team's permission level for a repository within an organization; the team must be a member of the organization. | {"parameters":{"description":"Reque...
| GITHUB_ADD_PROJECT_COLLABORATOR | Adds a specified GitHub user as a collaborator to an existing organization project with a given permission level. Note: This endpoint is for organization projects (classic). You must b...
| GITHUB_ADD_REPOSITORY_TO_APP_INSTALLATION | Adds a repository to a GitHub App installation, granting the app access; requires authenticated user to have admin rights for the repository and access to the installation. |...
| GITHUB_ADD_REPO_TO_ORG_SECRET_WITH_SELECTED_ACCESS | Adds a repository to an existing organization-level GitHub Actions secret that is configured for 'selected' repository access. | {"parameters":{"description":"Reques...
| GITHUB_ADD_REPO_TO_ORG_SECRET_WITH_SELECTED_VISIBILITY | Grants an existing repository access to an existing organization-level Dependabot secret when the secret's visibility is set to 'selected'; the repository must b...
| GITHUB_ADD_RUNNER_LABELS | Adds and appends custom labels to a self-hosted repository runner, which must be registered and active. | {"parameters":{"description":"Request schema for `AddCustomLabelsToASelfHostedRunnerF...
| GITHUB_ADD_SELECTED_REPOSITORY_TO_ORGANIZATION_SECRET | Adds a repository to an organization secret's access list when the secret's visibility is 'selected'; this operation is idempotent. | {"parameters":{"description"...
| GITHUB_ADD_SELECTED_REPOSITORY_TO_ORGANIZATION_VARIABLE | Grants a repository access to an organization-level GitHub Actions variable, if that variable's visibility is set to 'selected_repositories'. | {"parameters":{"...
[... 11 row(s) omitted ... ⟦tj:8586b9ba98d720d944d8a382f1d83d01⟧]
| GITHUB_AUTH_USER_DOCKER_CONFLICT_PACKAGES_LIST | List Docker packages with migration conflicts for the authenticated user. This endpoint lists all Docker packages owned by the authenticated user that encountered namesp...
[... 18 row(s) omitted ... ⟦tj:194cffe66d120fdc6021fd62c88bdbb2⟧]
| GITHUB_CHECK_TOKEN | Checks if a GitHub App or OAuth access_token is valid for the specified client_id and retrieves its details, typically to verify its active status and grants. NOTE: This endpoint requires Basic Aut...
| GITHUB_CLEAR_PROJECT_V2_ITEM_FIELD_VALUE | Tool to clear the value of a field for an item in a GitHub Project V2. Use when you need to remove or reset a field value (text, number, date, assignees, labels, single-select...
| GITHUB_CLEAR_REPOSITORY_CACHE_BY_KEY | Deletes GitHub Actions caches from a repository matching a specific `key` and an optional Git `ref`, used to manage storage or clear outdated/corrupted caches; the action succeeds...
| GITHUB_CLEAR_SELF_HOSTED_RUNNER_ORG_LABELS | Removes all custom labels from a self-hosted runner for an organization; default labels (e.g., 'self-hosted', 'linux', 'x64') will remain. | {"parameters":{"description":"Re...
| GITHUB_COMMIT_MULTIPLE_FILES | Tool to atomically create, update, or delete multiple files in a GitHub repository as a single commit. Uses Git Data APIs to avoid SHA mismatch conflicts that occur with the Contents API ...
| GITHUB_COMPARE_TWO_COMMITS | Compares two commit points (commits, branches, tags, or SHAs) within a repository or across forks, using `BASE...HEAD` or `OWNER:REF...OWNER:REF` format for the `basehead` parameter. | {"pa...
| GITHUB_CONFIGURE_JIT_RUNNER_FOR_ORG | Generates a JIT configuration for a GitHub organization's new self-hosted runner to run a single job then unregister; requires admin:org scope and the runner_group_id must exist in...
| GITHUB_CONFIGURE_OIDC_SUBJECT_CLAIM_TEMPLATE | Sets or updates the OIDC subject claim customization template for an existing GitHub organization by specifying which claims (e.g., 'repo', 'actor') form the OIDC token's ...
| GITHUB_CONVERT_ORG_MEMBER_TO_OUTSIDE_COLLABORATOR | Converts an existing organization member, who is not an owner, to an outside collaborator, restricting their access to explicitly granted repositories. | {"parameters...

```

### `03-slack-tools-array`

- [Full input](cases/03-slack-tools-array/input.json)
- [Output with CCR](cases/03-slack-tools-array/output.md) - [diff](cases/03-slack-tools-array/compression.diff)
- [Output without CCR](cases/03-slack-tools-array/output-noccr.md) - [diff](cases/03-slack-tools-array/compression-noccr.diff)

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
[all rows: type="function"]
[json table: 60 rows × 3 cols · blank=absent key · exact original via retrieve footer]
| function.name | function.description | function.… |
| --- | --- | --- |
| SLACK_ADD_CALL_PARTICIPANTS | Registers new participants added to a Slack call. | {"parameters":{"description":"Request schema for `AddCallParticipants`","properties":{"id":{"description":"ID of the call returned by th...
| SLACK_ADD_EMOJI | Adds a custom emoji to a Slack workspace given a unique name and an image URL; subject to workspace emoji limits. | {"parameters":{"description":"Request schema for `AddEmoji`","properties":{"name":{"...
| SLACK_ADD_EMOJI_ALIAS | Adds an alias for an existing custom emoji in a Slack Enterprise Grid organization. | {"parameters":{"description":"Request schema for `AddEmojiAlias`","properties":{"alias_for":{"description":"...
| SLACK_ADD_ENTERPRISE_USER_TO_WORKSPACE | Adds an Enterprise user to a workspace. Use when you need to assign an existing Enterprise Grid user to a specific workspace with optional guest restrictions. | {"parameters":{"...
| SLACK_ADD_REACTION_TO_AN_ITEM | Adds a specified emoji reaction to an existing message in a Slack channel, identified by its timestamp; does not remove or retrieve reactions. | {"parameters":{"description":"Request sch...
| SLACK_ADD_REMOTE_FILE | Adds a reference to an external file (e.g., Google Drive, Dropbox) to Slack for discovery and sharing, requiring a unique `external_id` and an `external_url` accessible by Slack. | {"parameters"...
| SLACK_ADD_STAR | Stars a channel, file, file comment, or a specific message in Slack. | {"parameters":{"description":"Request schema for the `stars.add` API method. Used to add a star to a channel, file, file comment, ...
| SLACK_ADMIN_CONVERSATIONS_SEARCH | Tool to search for public or private channels in an Enterprise organization. Use when you need to find channels by name, type, or other criteria within an Enterprise Grid workspace. |...
| SLACK_API_TEST | Tool to check API calling code by testing connectivity and authentication to the Slack API. Use when you need to verify that API credentials are valid and the connection is working properly. | {"parame...
| SLACK_ARCHIVE_CONVERSATION | Archives a Slack conversation by its ID, rendering it read-only and hidden while retaining history, ideal for cleaning up inactive channels; be aware that some channels (like #general or ce...
| SLACK_ASSISTANT_SEARCH_CONTEXT | Search across Slack messages, files, channels, and users using Real-time Search API. BEFORE USING: Call SLACK_ASSISTANT_SEARCH_INFO to check workspace capabilities. - If is_ai_search_en...
| SLACK_ASSISTANT_SEARCH_INFO | Check if semantic (AI-powered) search is available on the Slack workspace. Returns whether natural language queries will trigger semantic search in assistant.search.context calls. | {"para...
| SLACK_CLOSE_DM | Closes a Slack direct message (DM) or multi-person direct message (MPDM) channel, removing it from the user's sidebar without deleting history; this action affects only the calling user's view. | {"par...
| SLACK_CONVERT_CHANNEL_TO_PRIVATE | Convert a public Slack channel to private using the Admin API. This is an Enterprise Grid only feature and requires an org-installed user token with admin.conversations:write scope. |...
| SLACK_CREATE_A_REMINDER | Creates a Slack reminder with specified text and time; time accepts Unix timestamps, seconds from now, or natural language (e.g., 'in 15 minutes', 'every Thursday at 2pm'). | {"parameters":{"d...
| SLACK_CREATE_CANVAS | Creates a new Slack Canvas with the specified title and optional content. | {"parameters":{"properties":{"channel_id":{"description":"Optional channel ID (e.g., 'C1234567890'). If provided, the ca...
| SLACK_CREATE_CHANNEL | Initiates a public or private channel-based conversation in a Slack workspace. Immediately creates the channel; invoke only after explicit user confirmation. | {"parameters":{"description":"Reque...
| SLACK_CREATE_CHANNEL_BASED_CONVERSATION | Creates a new public or private Slack channel with a unique name; the channel can be org-wide, or team-specific if `team_id` is given (required if `org_wide` is false or not pr...
| SLACK_CREATE_ENTERPRISE_TEAM | Tool to create an Enterprise team in Slack. Use when you need to create a new team (workspace) within an Enterprise Grid organization. Requires admin.teams:write scope. | {"parameters":{"...
| SLACK_CREATE_USER_GROUP | Creates a new User Group (often referred to as a subteam) in a Slack workspace. | {"parameters":{"description":"Request schema for `CreateUserGroup`","properties":{"additional_channels":{"desc...
[... 18 row(s) omitted ... ⟦tj:94f26625b66ac8a673ad3b365d4d916c⟧]
| SLACK_FETCH_ITEM_REACTIONS | Fetches reactions for a Slack message, file, or file comment. Exactly one identifier path must be provided: `channel`+`timestamp`, `file`, or `file_comment`. Mixing identifiers (e.g., provi...
[... 11 row(s) omitted ... ⟦tj:c4a2ec691ab74e8cfeb5a08bd7c3272e⟧]
| SLACK_GET_CHANNEL_CONVERSATION_PREFERENCES | Retrieves conversation preferences (e.g., who can post, who can thread) for a specified channel, primarily for use within Slack Enterprise Grid environments. | {"parameters"...
| SLACK_GET_REMINDER | Retrieves detailed information for an existing Slack reminder specified by its ID; this is a read-only operation. | {"parameters":{"description":"Request schema for `GetReminder` action. Specifies ...
| SLACK_GET_REMOTE_FILE | Retrieve information about a remote file added to Slack via the files.remote API. Does not work for standard Slack-hosted file uploads. | {"parameters":{"description":"Request schema for `GetRem...
| SLACK_GET_TEAM_PROFILE | Retrieves all profile field definitions for a Slack team, optionally filtered by visibility, to understand the team's profile structure. | {"parameters":{"description":"Request schema to fetch ...
| SLACK_GET_USER_DND_STATUS | Retrieves a user's current Do Not Disturb status. | {"parameters":{"description":"Request schema for `GetUserDndStatus`","properties":{"team_id":{"description":"The workspace ID (team_id) to...
| SLACK_GET_USER_PRESENCE | Retrieves a Slack user's current real-time presence (e.g., 'active', 'away') to determine their availability, noting this action does not provide historical data or status reasons. | {"paramet...
| SLACK_GET_WORKSPACE_CONNECTIONS_FOR_CHANNEL | Tool to get all workspaces a channel is connected to within an Enterprise org. Use when you need to determine which workspaces have access to a specific public or private c...
| SLACK_GET_WORKSPACE_SETTINGS | Retrieves detailed settings for a specific Slack workspace, primarily for administrators in an Enterprise Grid organization to view or audit workspace configurations. | {"parameters":{"de...
| SLACK_INVITE_USERS_TO_A_SLACK_CHANNEL | Invites users to an existing Slack channel using their valid Slack User IDs. Response is always HTTP 200; inspect `ok`, `error`, and `errors` fields to confirm users were added. ...

```

### `10-cargo-metadata`

- [Full input](cases/10-cargo-metadata/input.json)
- [Output with CCR](cases/10-cargo-metadata/output.md) - [diff](cases/10-cargo-metadata/compression.diff)
- [Output without CCR](cases/10-cargo-metadata/output-noccr.md) - [diff](cases/10-cargo-metadata/compression-noccr.diff)

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
- [Output with CCR](cases/09-package-manifest/output.md) - [diff](cases/09-package-manifest/compression.diff)
- [Output without CCR](cases/09-package-manifest/output-noccr.md) - [diff](cases/09-package-manifest/compression-noccr.diff)

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
- [Output with CCR](cases/08-lottie-animation/output.md) - [diff](cases/08-lottie-animation/compression.diff)
- [Output without CCR](cases/08-lottie-animation/output-noccr.md) - [diff](cases/08-lottie-animation/compression-noccr.diff)

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

### `06-tauri-capabilities-schema`

- [Full input](cases/06-tauri-capabilities-schema/input.json)
- [Output with CCR](cases/06-tauri-capabilities-schema/output.md) - [diff](cases/06-tauri-capabilities-schema/compression.diff)
- [Output without CCR](cases/06-tauri-capabilities-schema/output-noccr.md) - [diff](cases/06-tauri-capabilities-schema/compression-noccr.diff)

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
- [Output with CCR](cases/05-polymarket-events-list/output.md) - [diff](cases/05-polymarket-events-list/compression.diff)
- [Output without CCR](cases/05-polymarket-events-list/output-noccr.md) - [diff](cases/05-polymarket-events-list/compression-noccr.diff)

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
- [Output with CCR](cases/04-polymarket-markets-list/output.md) - [diff](cases/04-polymarket-markets-list/compression.diff)
- [Output without CCR](cases/04-polymarket-markets-list/output-noccr.md) - [diff](cases/04-polymarket-markets-list/compression-noccr.diff)

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

