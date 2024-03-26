---
title: Security
metaTitle: Metaplex — Security
description: Audits and how to report a vulnerability.
---

## Reporting a Vulnerability

**Please do not open a public GitHub Issue to report the vulnerability**.

Instead, please email security@metaplex.foundation.

You should receive a response within 24-48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the requested information listed below (as much as you can provide) to help us better understand the nature and scope of the possible issue:

- Type of issue (e.g. buffer overflow, missing ownership check, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

You may also be eligible for a bounty. More details can be found [here](https://www.metaplex.com/bounty-program).

## Audits

Ongoing automated and manual security audits are routinely performed by our audit partners [Sec3](https://www.sec3.dev/) and [MadShield](https://www.madshield.xyz). Automated audits are performed for every PR and security issues must be resolved before merging into the main branch. Manual ongoing audits are initiated for changes above a specific threshold and security issues must be resolved before merging into the main branch.

Large one-off audits are also performed when there are large changes to the code or functionality as detailed below.

| Protocol              | Last major one-off audit date |
| --------------------- | ----------------------------- |
| Token Metadata        | 2023-06-24                    |
| Trifle/Fusion         | 2023-04-13                    |
| Bubblegum/Compression | 2022-11-02                    |
| Candy Machine V3      | 2022-11-01                    |
| Candy Machine V2      | 2022-11-01                    |
| Auction House         | 2022-10-24                    |
| Gumdrop               | 2022-05-16                    |

We do not have ongoing automated nor manual security audits that are routinely performed by our audit partners for our developer tools. However, audits may be ordered, facilitated, and paid for by our community of 3rd party Solana ecosystem developers or entities of thier own accord.

| Developer Tools | Last audit date |
| --------------- | --------------- |
| Sugar CLI\*     | 2022-08-26      |

(\*) Audited by [OtterSec](https://osec.io)
