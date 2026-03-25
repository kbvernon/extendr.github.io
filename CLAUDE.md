## Overview

This repo contains code and data required to compile the **Quarto-based 
documentation website** for the [extendr](https://github.com/extendr/extendr) 
project — a framework for writing R packages using Rust. Published at 
https://extendr.github.io.

## Codebase

**extendr** 
repo: https://github.com/extendr/extendr
description: The core Rust library. Read source files there directly when you 
need to understand extendr's API, types, macros, or implementation details.

**rextendr**
repo: https://github.com/extendr/rextendr
description: The R package that scaffolds extendr projects and generates R 
wrappers. Read source files there directly when you need to understand 
`rextendr::use_extendr()`, template generation, or the wrapper generation 
architecture.

## Repository Structure

```
.
├── _quarto.yml       # Site config: navbar, sidebars, theme
├── index.qmd         # Landing page (3 cards: Get Started, Examples, Community)
├── get-started.qmd   # 4-step install guide (Rust, R ≥ 4.2, rextendr, rust-analyzer)
├── changelog.qmd     # Dynamically fetches extendr CHANGELOG.md from GitHub
├── user-guide/       # User guide
├── intro-rust/       # Intro to Rust for R developers
├── contributing/     # Contributor guide (style, colors, scaffolding)
├── blog/             # Blog posts
├── css/              # Custom SCSS
├── images/           # Image files
└── _extensions/      # Quarto extensions
```

## Quarto (`_quarto.yml`)

This is a Quarto website. The main site structure and metadata are defined in 
`_quarto.yml`, including the navbar, sidebars, theme, and page execution 
settings. When adding or reorganizing pages, `_quarto.yml` is the first place to 
look.

## Content Sections

- Get Started (`get-started.qmd`): An installation guide for setting up extendr 
  and rextendr.
- User Guide (`user-guide/`): The main guide for R package development with 
  extendr. 
- Rust Basics (`intro-rust/`): An opinionated introduction to Rust for R 
  developers, covering the concepts needed to use extendr effectively.
- Contributing (`contributing/`): Guidelines for contributing to the extendr
  documentation, including writing style, branding, and scaffolding details.


## Claude Behavior

- DO NOT BE OBSEQUIOUS WITH PRAISE.
- BE CONCISE. Only elaborate when that is requested.
- When editing markdown files, please keep line length to 80 characters.
- DO NOT EDIT CLAUDE.md WITHOUT ASKING PERMISSION.