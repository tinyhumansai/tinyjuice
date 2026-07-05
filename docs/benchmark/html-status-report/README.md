# HTML, RSS, And Page Snapshots

Real RSS feeds, noisy web pages, forum pages, and OpenHuman coverage HTML. The HTML compressor strips markup/script noise and keeps readable page text.

Each row links to the full raw input and both compacted outputs. Percentages are **token reduction: higher is better**; 0% means pass-through. `Bytes` shows the raw input size -> compressor-only output size and its byte reduction. `Pass 1` disables CCR (compressed with omission markers, no recovery footer). `Pass 2` is the final model-facing result with CCR enabled — it reads *lower* than Pass 1 only because the recovery footer and per-block retrieval tokens add bytes; the compression itself is identical. Each pass links its own output and its own diff against the input.

## Cases

Every case links to the raw input; each pass column carries its percentage plus that pass's exact output and a unified diff against the input.

| Case | Input | Bytes | Pass 1: no CCR | Pass 2: with CCR | Avg latency |
| --- | --- | ---: | ---: | ---: | ---: |
| `04-forum-rust-users` | [input](cases/04-forum-rust-users/input.html) | 83.9 KB -> 3.3 KB (-96%) | 96.1%<br>[output](cases/04-forum-rust-users/output-noccr.txt) - [diff](cases/04-forum-rust-users/compression-noccr.diff) | 96.0%<br>[output](cases/04-forum-rust-users/output.txt) - [diff](cases/04-forum-rust-users/compression.diff) | 0.168 ms |
| `06-openhuman-coverage-6` | [input](cases/06-openhuman-coverage-6/input.html) | 4.4 KB -> 385 B (-91%) | 91.3%<br>[output](cases/06-openhuman-coverage-6/output-noccr.txt) - [diff](cases/06-openhuman-coverage-6/compression-noccr.diff) | 88.6%<br>[output](cases/06-openhuman-coverage-6/output.txt) - [diff](cases/06-openhuman-coverage-6/compression.diff) | 0.015 ms |
| `08-openhuman-coverage-8` | [input](cases/08-openhuman-coverage-8/input.html) | 5.2 KB -> 471 B (-91%) | 90.9%<br>[output](cases/08-openhuman-coverage-8/output-noccr.txt) - [diff](cases/08-openhuman-coverage-8/compression-noccr.diff) | 88.7%<br>[output](cases/08-openhuman-coverage-8/output.txt) - [diff](cases/08-openhuman-coverage-8/compression.diff) | 0.017 ms |
| `03-noisy-hacker-news` | [input](cases/03-noisy-hacker-news/input.html) | 34.4 KB -> 3.7 KB (-89%) | 89.3%<br>[output](cases/03-noisy-hacker-news/output-noccr.txt) - [diff](cases/03-noisy-hacker-news/compression-noccr.diff) | 89.0%<br>[output](cases/03-noisy-hacker-news/output.txt) - [diff](cases/03-noisy-hacker-news/compression.diff) | 0.098 ms |
| `07-openhuman-coverage-7` | [input](cases/07-openhuman-coverage-7/input.html) | 6.5 KB -> 1.1 KB (-83%) | 83.4%<br>[output](cases/07-openhuman-coverage-7/output-noccr.txt) - [diff](cases/07-openhuman-coverage-7/compression-noccr.diff) | 81.6%<br>[output](cases/07-openhuman-coverage-7/output.txt) - [diff](cases/07-openhuman-coverage-7/compression.diff) | 0.023 ms |
| `10-openhuman-coverage-10` | [input](cases/10-openhuman-coverage-10/input.html) | 5.8 KB -> 1.0 KB (-83%) | 82.7%<br>[output](cases/10-openhuman-coverage-10/output-noccr.txt) - [diff](cases/10-openhuman-coverage-10/compression-noccr.diff) | 80.7%<br>[output](cases/10-openhuman-coverage-10/output.txt) - [diff](cases/10-openhuman-coverage-10/compression.diff) | 0.020 ms |
| `05-openhuman-coverage-5` | [input](cases/05-openhuman-coverage-5/input.html) | 6.6 KB -> 1.2 KB (-82%) | 81.8%<br>[output](cases/05-openhuman-coverage-5/output-noccr.txt) - [diff](cases/05-openhuman-coverage-5/compression-noccr.diff) | 80.1%<br>[output](cases/05-openhuman-coverage-5/output.txt) - [diff](cases/05-openhuman-coverage-5/compression.diff) | 0.024 ms |
| `09-openhuman-coverage-9` | [input](cases/09-openhuman-coverage-9/input.html) | 24.6 KB -> 5.0 KB (-80%) | 79.9%<br>[output](cases/09-openhuman-coverage-9/output-noccr.txt) - [diff](cases/09-openhuman-coverage-9/compression-noccr.diff) | 79.4%<br>[output](cases/09-openhuman-coverage-9/output.txt) - [diff](cases/09-openhuman-coverage-9/compression.diff) | 0.083 ms |
| `02-rss-hacker-news` | [input](cases/02-rss-hacker-news/input.xml) | 15.2 KB -> 8.1 KB (-47%) | 47.0%<br>[output](cases/02-rss-hacker-news/output-noccr.txt) - [diff](cases/02-rss-hacker-news/compression-noccr.diff) | 46.3%<br>[output](cases/02-rss-hacker-news/output.txt) - [diff](cases/02-rss-hacker-news/compression.diff) | 0.055 ms |
| `01-rss-rust-blog` | [input](cases/01-rss-rust-blog/input.xml) | 384.1 KB -> 278.4 KB (-28%) | 27.5%<br>[output](cases/01-rss-rust-blog/output-noccr.txt) - [diff](cases/01-rss-rust-blog/compression-noccr.diff) | 27.5%<br>[output](cases/01-rss-rust-blog/output.txt) - [diff](cases/01-rss-rust-blog/compression.diff) | 1.213 ms |

## What TinyJuice Is Doing

HTML snapshots are converted into readable text. Script/style payloads and repeated markup disappear; the output keeps the content an agent would normally inspect.

## Syntax-Aware Samples

### `04-forum-rust-users`

- [Full input](cases/04-forum-rust-users/input.html)
- [Output with CCR](cases/04-forum-rust-users/output.txt) - [diff](cases/04-forum-rust-users/compression.diff)
- [Output without CCR](cases/04-forum-rust-users/output-noccr.txt) - [diff](cases/04-forum-rust-users/compression-noccr.diff)

Input excerpt:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>The Rust Programming Language Forum</title>
    <meta name="description" content="General discussion of The Rust Programming Language">
    <meta name="generator" content="Discourse 2026.7.0-latest - https://github.com/discourse/discourse version 2a08d5d5ca62c6f4fd4fba8a281bee36976e15d3">
<link rel="icon" type="image/png" href="https://us1.discourse-cdn.com/flex019/uploads/rust_lang/optimized/2X/e/e011218794dba02ebb2b368fd9b831f5585caffe_2_32x32.ico">
<link rel="apple-touch-icon" type="image/png" href="https://us1.discourse-cdn.com/flex019/uploads/rust_lang/optimized/2X/8/83e41956eccfd67ad6ff76f15a2c22e58db31d4f_2_180x180.svg">
<meta name="theme-color" media="all" content="#fff">

<meta name="color-scheme" content="light">

<meta name="viewport" content="width=device-width, initial-scale=1.0, minimum-scale=1.0, viewport-fit=cover">
<link rel="canonical" href="https://users.rust-lang.org/" />
<script type="application/ld+json">{"@context":"http://schema.org","@type":"WebSite","url":"https://users.rust-lang.org","name":"The Rust Programming Language Forum","potentialAction":{"@type":"SearchAction","target":"ht...
<meta name="discourse-track-view-session-id" content="IsmpiE3mCkkkNhlXVoUGmdDDTtThfOT4">
<link rel="search" type="application/opensearchdescription+xml" href="https://users.rust-lang.org/opensearch.xml" title="The Rust Programming Language Forum Search">

    
    <link href="https://sea1.discourse-cdn.com/flex019/stylesheets/color_definitions_light-default_-1_1_d5537d15356de74106d0f6f9a9d1cdbf01cec97b.css?__ws=users.rust-lang.org" media="all" rel="stylesheet" class="light-sch...

<link href="https://sea1.discourse-cdn.com/flex019/stylesheets/common_f6d21c3b4672e59e32927108f6d7b37328968701.css?__ws=users.rust-lang.org" media="all" rel="stylesheet" data-target="common"  />

<link href="https://sea1.discourse-cdn.com/flex019/stylesheets/mobile_f6d21c3b4672e59e32927108f6d7b37328968701.css?__ws=users.rust-lang.org" media="(width < 40rem)" rel="stylesheet" data-target="mobile"  />
<link href="https://sea1.discourse-cdn.com/flex019/stylesheets/desktop_f6d21c3b4672e59e32927108f6d7b37328968701.css?__ws=users.rust-lang.org" media="(width >= 40rem)" rel="stylesheet" data-target="desktop"  />



  <link href="https://sea1.discourse-cdn.com/flex019/stylesheets/automation_f6d21c3b4672e59e32927108f6d7b37328968701.css?__ws=users.rust-lang.org" media="all" rel="stylesheet" data-target="automation"  />
  <link href="https://sea1.discourse-cdn.com/flex019/stylesheets/checklist_f6d21c3b4672e59e32927108f6d7b37328968701.css?__ws=users.rust-lang.org" media="all" rel="stylesheet" data-target="checklist"  />
  <link href="https://sea1.discourse-cdn.com/flex019/stylesheets/discourse-ai_f6d21c3b4672e59e32927108f6d7b37328968701.css?__ws=users.rust-lang.org" media="all" rel="stylesheet" data-target="discourse-ai"  />
  <link href="https://sea1.discourse-cdn.com/flex019/stylesheets/discourse-cakeday_f6d21c3b4672e59e32927108f6d7b37328968701.css?__ws=users.rust-lang.org" media="all" rel="stylesheet" data-target="discourse-cakeday"  />
  <link href="https://sea1.discourse-cdn.com/flex019/stylesheets/discourse-data-explorer_f6d21c3b4672e59e32927108f6d7b37328968701.css?__ws=users.rust-lang.org" media="all" rel="stylesheet" data-target="discourse-data-exp...
  <link href="https://sea1.discourse-cdn.com/flex019/stylesheets/discourse-details_f6d21c3b4672e59e32927108f6d7b37328968701.css?__ws=users.rust-lang.org" media="all" rel="stylesheet" data-target="discourse-details"  />
  <link href="https://sea1.discourse-cdn.com/flex019/stylesheets/discourse-github_f6d21c3b4672e59e32927108f6d7b37328968701.css?__ws=users.rust-lang.org" media="all" rel="stylesheet" data-target="discourse-github"  />

```

Output excerpt:

```text
The Rust Programming Language Forum

The Rust Programming Language Forum

General discussion of The Rust Programming Language

Topic

Replies
Views
Activity

Welcome to the Rust programming language users forum

meta

This forum is for help, discussion, and announcements related to the Rust programming language.
Please read our code of conduct before participating. We will remove any posts that are not respectful, constructive, and o…

1

56124

June 24, 2022

Forum Code Formatting and Syntax Highlighting

meta

To format code in this forum you need to surround the code with three backticks (` ` `). For example, typing this...
` ` `
fn main() {
println!()
}
` ` `


```

### `06-openhuman-coverage-6`

- [Full input](cases/06-openhuman-coverage-6/input.html)
- [Output with CCR](cases/06-openhuman-coverage-6/output.txt) - [diff](cases/06-openhuman-coverage-6/compression.diff)
- [Output without CCR](cases/06-openhuman-coverage-6/output-noccr.txt) - [diff](cases/06-openhuman-coverage-6/compression-noccr.diff)

Input excerpt:

```html

<!doctype html>
<html lang="en">

<head>
    <title>Code coverage report for src/assets/icons</title>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="../../../prettify.css" />
    <link rel="stylesheet" href="../../../base.css" />
    <link rel="shortcut icon" type="image/x-icon" href="../../../favicon.png" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type='text/css'>
        .coverage-summary .sorter {
            background-image: url(../../../sort-arrow-sprite.png);
        }
    </style>
</head>
    
<body>
<div class='wrapper'>
    <div class='pad1'>
        <h1><a href="../../../index.html">All files</a> src/assets/icons</h1>
        <div class='clearfix'>
            
            <div class='fl pad1y space-right2'>
                <span class="strong">0% </span>
                <span class="quiet">Statements</span>
                <span class='fraction'>0/2</span>
            </div>
        
            
            <div class='fl pad1y space-right2'>
                <span class="strong">0% </span>
                <span class="quiet">Branches</span>
                <span class='fraction'>0/1</span>
            </div>

```

Output excerpt:

```text
Code coverage report for src/assets/icons

All files src/assets/icons

0%
Statements
0/2

0%
Branches
0/1

0%
Functions
0/1

0%
Lines
0/2

Press n or j to go to the next uncovered block, b, p or k for the previous block.

Filter:

File

Statements

Branches

Functions

Lines

GoogleIcon.tsx


```

### `08-openhuman-coverage-8`

- [Full input](cases/08-openhuman-coverage-8/input.html)
- [Output with CCR](cases/08-openhuman-coverage-8/output.txt) - [diff](cases/08-openhuman-coverage-8/compression.diff)
- [Output without CCR](cases/08-openhuman-coverage-8/output-noccr.txt) - [diff](cases/08-openhuman-coverage-8/compression-noccr.diff)

Input excerpt:

```html

<!doctype html>
<html lang="en">

<head>
    <title>Code coverage report for src/chat</title>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="../../prettify.css" />
    <link rel="stylesheet" href="../../base.css" />
    <link rel="shortcut icon" type="image/x-icon" href="../../favicon.png" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type='text/css'>
        .coverage-summary .sorter {
            background-image: url(../../sort-arrow-sprite.png);
        }
    </style>
</head>
    
<body>
<div class='wrapper'>
    <div class='pad1'>
        <h1><a href="../../index.html">All files</a> src/chat</h1>
        <div class='clearfix'>
            
            <div class='fl pad1y space-right2'>
                <span class="strong">55.55% </span>
                <span class="quiet">Statements</span>
                <span class='fraction'>25/45</span>
            </div>
        
            
            <div class='fl pad1y space-right2'>
                <span class="strong">46.34% </span>
                <span class="quiet">Branches</span>
                <span class='fraction'>19/41</span>
            </div>

```

Output excerpt:

```text
Code coverage report for src/chat

All files src/chat

55.55%
Statements
25/45

46.34%
Branches
19/41

80%
Functions
4/5

54.54%
Lines
24/44

Press n or j to go to the next uncovered block, b, p or k for the previous block.

Filter:

File

Statements

Branches

Functions

Lines

chatSendError.ts


```

### `03-noisy-hacker-news`

- [Full input](cases/03-noisy-hacker-news/input.html)
- [Output with CCR](cases/03-noisy-hacker-news/output.txt) - [diff](cases/03-noisy-hacker-news/compression.diff)
- [Output without CCR](cases/03-noisy-hacker-news/output-noccr.txt) - [diff](cases/03-noisy-hacker-news/compression-noccr.diff)

Input excerpt:

```html
<html lang="en" op="news"><head><meta name="referrer" content="origin"><meta name="viewport" content="width=device-width, initial-scale=1.0"><link rel="stylesheet" type="text/css" href="news.css?Ug1GY3B6Kr5c7uonNem9"><li...
<center><span class="yclinks"><a href="newsguidelines.html">Guidelines</a> | <a href="newsfaq.html">FAQ</a> | <a href="lists">Lists</a> | <a href="https://github.com/HackerNews/API">API</a> | <a href="security.html">Secu...
<form method="get" action="//hn.algolia.com/">Search: <input type="text" name="q" size="17" autocorrect="off" spellcheck="false" autocapitalize="off" autocomplete="off"></form></center></td></tr></table></center></body><...

```

Output excerpt:

```text
Hacker News
Hacker Newsnew | past | comments | ask | show | jobs | submit login
1.
Shadcn/UI now defaults to Base UI instead of Radix (shadcn.com)
176 points by dabinat 7 hours ago | hide | 67 comments
2.
If you're a button, you have one job (aresluna.org)
302 points by nozzlegear 10 hours ago | hide | 153 comments
3.
Claude Design System Prompt (github.com/trystan-sa)
40 points by handfuloflight 3 hours ago | hide | 7 comments
4.
Fast Software, the Best Software (2019) (craigmod.com)
51 points by ustad 5 hours ago | hide | 22 comments
5.
GPT-5.5 Codex reasoning-token clustering may be leading to degraded performance (github.com/openai)
313 points by maille 14 hours ago | hide | 117 comments
6.
Functional Programming in hica (hica.dev)
11 points by cladamski79 2 hours ago | hide | 2 comments
7.
Educators disciplined over Charlie Kirk posts are securing big payouts (nbcnews.com)
10 points by Anon84 10 minutes ago | hide | 1 comment
8.
Pandoc Lua Filters (pandoc.org)
85 points by ankitg12 7 hours ago | hide | 5 comments
9.
Scientist who cleaned space toilet on work now leading Mars exploration (bbc.com)
16 points by saikatsg 2 hours ago | hide | 4 comments
10.
Jellyfish can heal wounds in minutes. Scientists want their secrets (mbl.edu)
149 points by hhs 14 hours ago | hide | 31 comments
11.
Knowledge Should Not Be Gated (formaly.io)
20 points by nezhar 4 hours ago | hide | 2 comments
12.

```

### `07-openhuman-coverage-7`

- [Full input](cases/07-openhuman-coverage-7/input.html)
- [Output with CCR](cases/07-openhuman-coverage-7/output.txt) - [diff](cases/07-openhuman-coverage-7/compression.diff)
- [Output without CCR](cases/07-openhuman-coverage-7/output-noccr.txt) - [diff](cases/07-openhuman-coverage-7/compression-noccr.diff)

Input excerpt:

```html

<!doctype html>
<html lang="en">

<head>
    <title>Code coverage report for src/chat/chatSendError.ts</title>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="../../prettify.css" />
    <link rel="stylesheet" href="../../base.css" />
    <link rel="shortcut icon" type="image/x-icon" href="../../favicon.png" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type='text/css'>
        .coverage-summary .sorter {
            background-image: url(../../sort-arrow-sprite.png);
        }
    </style>
</head>
    
<body>
<div class='wrapper'>
    <div class='pad1'>
        <h1><a href="../../index.html">All files</a> / <a href="index.html">src/chat</a> chatSendError.ts</h1>
        <div class='clearfix'>
            
            <div class='fl pad1y space-right2'>
                <span class="strong">100% </span>
                <span class="quiet">Statements</span>
                <span class='fraction'>2/2</span>
            </div>
        
            
            <div class='fl pad1y space-right2'>
                <span class="strong">100% </span>
                <span class="quiet">Branches</span>
                <span class='fraction'>0/0</span>
            </div>

```

Output excerpt:

```text
Code coverage report for src/chat/chatSendError.ts

All files / src/chat chatSendError.ts

100%
Statements
2/2

100%
Branches
0/0

100%
Functions
1/1

100%
Lines
1/1

Press n or j to go to the next uncovered block, b, p or k for the previous block.

Filter:

1
2
3
4
5
6
7
8
9
10
11
12

```

### `10-openhuman-coverage-10`

- [Full input](cases/10-openhuman-coverage-10/input.html)
- [Output with CCR](cases/10-openhuman-coverage-10/output.txt) - [diff](cases/10-openhuman-coverage-10/compression.diff)
- [Output without CCR](cases/10-openhuman-coverage-10/output-noccr.txt) - [diff](cases/10-openhuman-coverage-10/compression-noccr.diff)

Input excerpt:

```html

<!doctype html>
<html lang="en">

<head>
    <title>Code coverage report for src/components/AppBackground.tsx</title>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="../../prettify.css" />
    <link rel="stylesheet" href="../../base.css" />
    <link rel="shortcut icon" type="image/x-icon" href="../../favicon.png" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type='text/css'>
        .coverage-summary .sorter {
            background-image: url(../../sort-arrow-sprite.png);
        }
    </style>
</head>
    
<body>
<div class='wrapper'>
    <div class='pad1'>
        <h1><a href="../../index.html">All files</a> / <a href="index.html">src/components</a> AppBackground.tsx</h1>
        <div class='clearfix'>
            
            <div class='fl pad1y space-right2'>
                <span class="strong">0% </span>
                <span class="quiet">Statements</span>
                <span class='fraction'>0/1</span>
            </div>
        
            
            <div class='fl pad1y space-right2'>
                <span class="strong">0% </span>
                <span class="quiet">Branches</span>
                <span class='fraction'>0/1</span>
            </div>

```

Output excerpt:

```text
Code coverage report for src/components/AppBackground.tsx

All files / src/components AppBackground.tsx

0%
Statements
0/1

0%
Branches
0/1

0%
Functions
0/1

0%
Lines
0/1

Press n or j to go to the next uncovered block, b, p or k for the previous block.

Filter:

1
2
3
4
5
6
7
8
9
10
11
12

```

### `05-openhuman-coverage-5`

- [Full input](cases/05-openhuman-coverage-5/input.html)
- [Output with CCR](cases/05-openhuman-coverage-5/output.txt) - [diff](cases/05-openhuman-coverage-5/compression.diff)
- [Output without CCR](cases/05-openhuman-coverage-5/output-noccr.txt) - [diff](cases/05-openhuman-coverage-5/compression-noccr.diff)

Input excerpt:

```html

<!doctype html>
<html lang="en">

<head>
    <title>Code coverage report for src/assets/icons/GoogleIcon.tsx</title>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="../../../prettify.css" />
    <link rel="stylesheet" href="../../../base.css" />
    <link rel="shortcut icon" type="image/x-icon" href="../../../favicon.png" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type='text/css'>
        .coverage-summary .sorter {
            background-image: url(../../../sort-arrow-sprite.png);
        }
    </style>
</head>
    
<body>
<div class='wrapper'>
    <div class='pad1'>
        <h1><a href="../../../index.html">All files</a> / <a href="index.html">src/assets/icons</a> GoogleIcon.tsx</h1>
        <div class='clearfix'>
            
            <div class='fl pad1y space-right2'>
                <span class="strong">0% </span>
                <span class="quiet">Statements</span>
                <span class='fraction'>0/2</span>
            </div>
        
            
            <div class='fl pad1y space-right2'>
                <span class="strong">0% </span>
                <span class="quiet">Branches</span>
                <span class='fraction'>0/1</span>
            </div>

```

Output excerpt:

```text
Code coverage report for src/assets/icons/GoogleIcon.tsx

All files / src/assets/icons GoogleIcon.tsx

0%
Statements
0/2

0%
Branches
0/1

0%
Functions
0/1

0%
Lines
0/2

Press n or j to go to the next uncovered block, b, p or k for the previous block.

Filter:

1
2
3
4
5
6
7
8
9
10
11
12

```

### `09-openhuman-coverage-9`

- [Full input](cases/09-openhuman-coverage-9/input.html)
- [Output with CCR](cases/09-openhuman-coverage-9/output.txt) - [diff](cases/09-openhuman-coverage-9/compression.diff)
- [Output without CCR](cases/09-openhuman-coverage-9/output-noccr.txt) - [diff](cases/09-openhuman-coverage-9/compression-noccr.diff)

Input excerpt:

```html

<!doctype html>
<html lang="en">

<head>
    <title>Code coverage report for src/chat/promptInjectionGuard.ts</title>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="../../prettify.css" />
    <link rel="stylesheet" href="../../base.css" />
    <link rel="shortcut icon" type="image/x-icon" href="../../favicon.png" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type='text/css'>
        .coverage-summary .sorter {
            background-image: url(../../sort-arrow-sprite.png);
        }
    </style>
</head>
    
<body>
<div class='wrapper'>
    <div class='pad1'>
        <h1><a href="../../index.html">All files</a> / <a href="index.html">src/chat</a> promptInjectionGuard.ts</h1>
        <div class='clearfix'>
            
            <div class='fl pad1y space-right2'>
                <span class="strong">53.48% </span>
                <span class="quiet">Statements</span>
                <span class='fraction'>23/43</span>
            </div>
        
            
            <div class='fl pad1y space-right2'>
                <span class="strong">46.34% </span>
                <span class="quiet">Branches</span>
                <span class='fraction'>19/41</span>
            </div>

```

Output excerpt:

```text
Code coverage report for src/chat/promptInjectionGuard.ts

All files / src/chat promptInjectionGuard.ts

53.48%
Statements
23/43

46.34%
Branches
19/41

75%
Functions
3/4

53.48%
Lines
23/43

Press n or j to go to the next uncovered block, b, p or k for the previous block.

Filter:

1
2
3
4
5
6
7
8
9
10
11
12

```

### `02-rss-hacker-news`

- [Full input](cases/02-rss-hacker-news/input.xml)
- [Output with CCR](cases/02-rss-hacker-news/output.txt) - [diff](cases/02-rss-hacker-news/compression.diff)
- [Output without CCR](cases/02-rss-hacker-news/output-noccr.txt) - [diff](cases/02-rss-hacker-news/compression-noccr.diff)

Input excerpt:

```xml
<rss version="2.0" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:atom="http://www.w3.org/2005/Atom"><channel><title>Hacker News: Front Page</title><link>https://news.ycombinator.com/</link><description>Hacker News RS...
<p>Article URL: <a href="https://www.nbcnews.com/news/us-news/educators-disciplined-charlie-kirk-posts-are-securing-big-payouts-rcna352568">https://www.nbcnews.com/news/us-news/educators-disciplined-charlie-kirk-posts-ar...
<p>Comments URL: <a href="https://news.ycombinator.com/item?id=48793726">https://news.ycombinator.com/item?id=48793726</a></p>
<p>Points: 5</p>
<p># Comments: 0</p>
]]></description><pubDate>Sun, 05 Jul 2026 12:29:44 +0000</pubDate><link>https://www.nbcnews.com/news/us-news/educators-disciplined-charlie-kirk-posts-are-securing-big-payouts-rcna352568</link><dc:creator>Anon84</dc:crea...
<p>Article URL: <a href="https://yusufaytas.com/the-engineer-in-the-half-space">https://yusufaytas.com/the-engineer-in-the-half-space</a></p>
<p>Comments URL: <a href="https://news.ycombinator.com/item?id=48793716">https://news.ycombinator.com/item?id=48793716</a></p>
<p>Points: 13</p>
<p># Comments: 0</p>
]]></description><pubDate>Sun, 05 Jul 2026 12:28:04 +0000</pubDate><link>https://yusufaytas.com/the-engineer-in-the-half-space</link><dc:creator>yusufaytas</dc:creator><comments>https://news.ycombinator.com/item?id=48793...
<p>Article URL: <a href="https://www.bbc.com/news/articles/c8e2j0j87reo">https://www.bbc.com/news/articles/c8e2j0j87reo</a></p>
<p>Comments URL: <a href="https://news.ycombinator.com/item?id=48792790">https://news.ycombinator.com/item?id=48792790</a></p>
<p>Points: 107</p>
<p># Comments: 119</p>
]]></description><pubDate>Sun, 05 Jul 2026 09:56:10 +0000</pubDate><link>https://www.bbc.com/news/articles/c8e2j0j87reo</link><dc:creator>saikatsg</dc:creator><comments>https://news.ycombinator.com/item?id=48792790</comm...
<p>Article URL: <a href="https://www.bbc.com/news/articles/cz758x04g83o">https://www.bbc.com/news/articles/cz758x04g83o</a></p>
<p>Comments URL: <a href="https://news.ycombinator.com/item?id=48792780">https://news.ycombinator.com/item?id=48792780</a></p>
<p>Points: 16</p>
<p># Comments: 4</p>
]]></description><pubDate>Sun, 05 Jul 2026 09:55:22 +0000</pubDate><link>https://www.bbc.com/news/articles/cz758x04g83o</link><dc:creator>saikatsg</dc:creator><comments>https://news.ycombinator.com/item?id=48792780</comm...
<p>Article URL: <a href="https://github.com/Trystan-SA/claude-design-system-prompt">https://github.com/Trystan-SA/claude-design-system-prompt</a></p>
<p>Comments URL: <a href="https://news.ycombinator.com/item?id=48792399">https://news.ycombinator.com/item?id=48792399</a></p>
<p>Points: 36</p>
<p># Comments: 4</p>
]]></description><pubDate>Sun, 05 Jul 2026 08:43:37 +0000</pubDate><link>https://github.com/Trystan-SA/claude-design-system-prompt</link><dc:creator>handfuloflight</dc:creator><comments>https://news.ycombinator.com/item?...
<p>Article URL: <a href="https://0dd.company/galleries/triumph/1.html">https://0dd.company/galleries/triumph/1.html</a></p>
<p>Comments URL: <a href="https://news.ycombinator.com/item?id=48792352">https://news.ycombinator.com/item?id=48792352</a></p>
<p>Points: 26</p>
<p># Comments: 5</p>
]]></description><pubDate>Sun, 05 Jul 2026 08:32:09 +0000</pubDate><link>https://0dd.company/galleries/triumph/1.html</link><dc:creator>scaglio</dc:creator><comments>https://news.ycombinator.com/item?id=48792352</comment...
<p>Article URL: <a href="https://www.formaly.io/blog/knowledge-should-not-be-gated">https://www.formaly.io/blog/knowledge-should-not-be-gated</a></p>
<p>Comments URL: <a href="https://news.ycombinator.com/item?id=48792195">https://news.ycombinator.com/item?id=48792195</a></p>
<p>Points: 18</p>
<p># Comments: 1</p>
]]></description><pubDate>Sun, 05 Jul 2026 07:59:48 +0000</pubDate><link>https://www.formaly.io/blog/knowledge-should-not-be-gated</link><dc:creator>nezhar</dc:creator><comments>https://news.ycombinator.com/item?id=48792...

```

Output excerpt:

```text
Hacker News: Front Page
https://news.ycombinator.com/ Hacker News RSS https://hnrss.org/ hnrss v2.1.1 Sun, 05 Jul 2026 12:36:38 +0000
Educators disciplined over Charlie Kirk posts are securing big payouts

Article URL: https://www.nbcnews.com/news/us-news/educators-disciplined-charlie-kirk-posts-are-securing-big-payouts-rcna352568

Comments URL: https://news.ycombinator.com/item?id=48793726

Points: 5

# Comments: 0

Sun, 05 Jul 2026 12:29:44 +0000 https://www.nbcnews.com/news/us-news/educators-disciplined-charlie-kirk-posts-are-securing-big-payouts-rcna352568 Anon84 https://news.ycombinator.com/item?id=48793726 https://news.ycombina...
The Engineer in the Half-Space

Article URL: https://yusufaytas.com/the-engineer-in-the-half-space

Comments URL: https://news.ycombinator.com/item?id=48793716

Points: 13

# Comments: 0

Sun, 05 Jul 2026 12:28:04 +0000 https://yusufaytas.com/the-engineer-in-the-half-space yusufaytas https://news.ycombinator.com/item?id=48793716 https://news.ycombinator.com/item?id=48793716
Europe's new climate in seven charts

Article URL: https://www.bbc.com/news/articles/c8e2j0j87reo

Comments URL: https://news.ycombinator.com/item?id=48792790

Points: 107

# Comments: 119

Sun, 05 Jul 2026 09:56:10 +0000 https://www.bbc.com/news/articles/c8e2j0j87reo saikatsg https://news.ycombinator.com/item?id=48792790 https://news.ycombinator.com/item?id=48792790
Scientist who cleaned space toilet on work now leading Mars exploration

```

### `01-rss-rust-blog`

- [Full input](cases/01-rss-rust-blog/input.xml)
- [Output with CCR](cases/01-rss-rust-blog/output.txt) - [diff](cases/01-rss-rust-blog/compression.diff)
- [Output without CCR](cases/01-rss-rust-blog/output-noccr.txt) - [diff](cases/01-rss-rust-blog/compression-noccr.diff)

Input excerpt:

```xml
<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom" xml:lang="en">
    <generator uri="https://blog.rust-lang.org/" version="0.1.0">Rust Blog</generator>
    <link href="https://blog.rust-lang.org/feed.xml" rel="self" type="application/atom+xml" />
    <link href="https://blog.rust-lang.org/" rel="alternate" type="text/html" />
    <id>https://blog.rust-lang.org/</id>
    <title>Rust Blog</title>
    <subtitle>Empowering everyone to build reliable and efficient software.</subtitle>
    <author>
        <name>Maintained by the Rust Teams.</name>
        <uri>https://github.com/rust-lang/blog.rust-lang.org/</uri>
    </author>
    <updated>2026-07-03T16:22:30+00:00</updated>

    
    <entry>
        <title>Announcing Rust 1.96.1</title>
        <link rel="alternate" href="https://blog.rust-lang.org/2026/06/30/Rust-1.96.1/" type="text/html" title="Announcing Rust 1.96.1" />
        <published>2026-06-30T00:00:00+00:00</published>
        <updated>2026-06-30T00:00:00+00:00</updated>
        <id>https://blog.rust-lang.org/2026/06/30/Rust-1.96.1/</id>
        <content type="html" xml:base="https://blog.rust-lang.org/2026/06/30/Rust-1.96.1/">&lt;p&gt;The Rust team has published a new point release of Rust, 1.96.1. Rust is a programming language that is empowering every...
&lt;p&gt;If you have a previous version of Rust installed via rustup, getting Rust 1.96.1 is as easy as:&lt;/p&gt;
&lt;pre class&#x3D;&quot;giallo z-code&quot;&gt;&lt;code data-lang&#x3D;&quot;plain&quot;&gt;&lt;span class&#x3D;&quot;giallo-l&quot;&gt;&lt;span&gt;rustup update stable&lt;/span&gt;&lt;/span&gt;&lt;/code&gt;&lt;/pre&gt;
&lt;p&gt;If you don&#x27;t have it already, you can &lt;a rel&#x3D;&quot;external&quot; href&#x3D;&quot;https://www.rust-lang.org/install.html&quot;&gt;get &lt;code&gt;rustup&lt;/code&gt;&lt;/a&gt; from the appropriate p...
&lt;h2 id&#x3D;&quot;what-s-in-1-96-1&quot;&gt;&lt;a class&#x3D;&quot;anchor&quot; href&#x3D;&quot;#what-s-in-1-96-1&quot; aria-hidden&#x3D;&quot;true&quot;&gt;&lt;/a&gt;
What&#x27;s in 1.96.1&lt;/h2&gt;
&lt;p&gt;Rust 1.96.1 fixes:&lt;/p&gt;
&lt;ul&gt;
&lt;li&gt;&lt;a rel&#x3D;&quot;external&quot; href&#x3D;&quot;https://github.com/rust-lang/cargo/pull/17131&quot;&gt;Missing retries / timeouts in Cargo&#x27;s HTTP client&lt;/a&gt;&lt;/li&gt;
&lt;li&gt;&lt;a rel&#x3D;&quot;external&quot; href&#x3D;&quot;https://github.com/rust-lang/rust/pull/158214&quot;&gt;Miscompilation in a MIR optimization&lt;/a&gt;&lt;/li&gt;
&lt;/ul&gt;
&lt;p&gt;It also &lt;a rel&#x3D;&quot;external&quot; href&#x3D;&quot;https://github.com/rust-lang/cargo/pull/17140&quot;&gt;fixes&lt;/a&gt; three CVEs
affecting libssh2 (which is compiled into Cargo):&lt;/p&gt;
&lt;ul&gt;
&lt;li&gt;&lt;a rel&#x3D;&quot;external&quot; href&#x3D;&quot;https://www.cve.org/CVERecord?id&#x3D;CVE-2025-15661&quot;&gt;CVE-2025-15661&lt;/a&gt;&lt;/li&gt;

```

Output excerpt:

```text
Rust Blog

https://blog.rust-lang.org/

Rust Blog

Empowering everyone to build reliable and efficient software.

Maintained by the Rust Teams.
https://github.com/rust-lang/blog.rust-lang.org/

2026-07-03T16:22:30+00:00

Announcing Rust 1.96.1

2026-06-30T00:00:00+00:00
2026-06-30T00:00:00+00:00
https://blog.rust-lang.org/2026/06/30/Rust-1.96.1/
<p>The Rust team has published a new point release of Rust, 1.96.1. Rust is a programming language that is empowering everyone to build reliable and efficient software.</p>
<p>If you have a previous version of Rust installed via rustup, getting Rust 1.96.1 is as easy as:</p>
<pre class="giallo z-code"><code data-lang="plain"><span class="giallo-l"><span>rustup update stable</span></span></code></pre>
<p>If you don't have it already, you can <a rel="external" href="https://www.rust-lang.org/install.html">get <code>rustup</code></a> from the appropriate page on our website.</p>
<h2 id="what-s-in-1-96-1"><a class="anchor" href="#what-s-in-1-96-1" aria-hidden="true"></a>
What's in 1.96.1</h2>
<p>Rust 1.96.1 fixes:</p>
<ul>
<li><a rel="external" href="https://github.com/rust-lang/cargo/pull/17131">Missing retries / timeouts in Cargo's HTTP client</a></li>
<li><a rel="external" href="https://github.com/rust-lang/rust/pull/158214">Miscompilation in a MIR optimization</a></li>
</ul>
<p>It also <a rel="external" href="https://github.com/rust-lang/cargo/pull/17140">fixes</a> three CVEs
affecting libssh2 (which is compiled into Cargo):</p>
<ul>
<li><a rel="external" href="https://www.cve.org/CVERecord?id=CVE-2025-15661">CVE-2025-15661</a></li>
<li><a rel="external" href="https://www.cve.org/CVERecord?id=CVE-2026-55199">CVE-2026-55199</a></li>
<li><a rel="external" href="https://www.cve.org/CVERecord?id=CVE-2026-55200">CVE-2026-55200</a></li>
</ul>

```

