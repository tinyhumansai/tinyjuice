# Security Policy

TinyJuice is a token compression library. Security-sensitive areas include
prompt and context handling, sensitive data retention, accidental disclosure in
reports or logs, policy-preserved instructions, and credentials included in
model-facing context.

Report concerns to `contact@tinyhumans.ai`.

## Supported Versions

TinyJuice is pre-1.0. Security fixes target the `main` branch until the project
starts maintaining release branches.

## Reporting A Vulnerability

Please do not open a public issue for a suspected vulnerability.

Report security concerns by emailing `security@tinyhumans.ai` with:

- a description of the issue
- affected versions or commits
- reproduction steps or proof of concept
- impact assessment
- any suggested fix or mitigation

We will acknowledge reports as quickly as practical and coordinate disclosure
before publishing details.

## Scope

Examples of in-scope issues:

- leaking raw prompt, context, or credential material through reports or logs
- dropping system or safety instructions despite preservation policy
- cross-conversation context leakage in adapter code
- unsafe handling of credentials or secrets embedded in model input
- dependency vulnerabilities with a practical exploit path in TinyJuice

Examples generally out of scope:

- model hallucination or low-quality model output by itself
- expected information loss from an explicitly selected lossy strategy
- vulnerabilities in downstream applications that use TinyJuice incorrectly

## Security Design Direction

TinyJuice should treat prompt and context input as sensitive. Compression
strategies should avoid logging raw text, make lossy behavior explicit, and
preserve caller-marked instructions unless the caller opts into a different
policy.
