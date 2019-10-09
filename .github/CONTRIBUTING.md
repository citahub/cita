# Contributing to CITA

First off, thanks for taking the time to contribute!

The following is a set of guidelines for contributing to CITA and its packages. These are mostly guidelines, not rules. Use your best judgement, and feel free to propose changes to this document in a pull request.

## Table of Contents

[Code of Conduct](CODE_OF_CONDUCT.md)

[How Can I Contribute?](#how-can-i-contribute)

- [Reporting Bugs](#reporting-bugs)
- [Pull Request](#pull-request)

[Styleguides](#Styleguides)

- [Git Commit Messages](#git-commit-messages)
- [Rust Styleguide](#rust-styleguide)
- [Documentation Styleguide](#documentation-styleguide)

## How Can I Contribute'?'

### Reporting Bugs

This section guides you through submitting a bug report for CITA. Following these guidelines helps maintainers understand your report :pencil:, reproduce the behavior :computer:, and find the related reports :mag_right:.

When creating a bug report, please [include as many details as possible](#how-do-i-submit-a-good-bug-report). Fill out [the required template](ISSUE_TEMPLATE.md), the information it asks for helps others resolve issues faster.

#### How Do I Submit a Good Bug Report'?'

After you’ve determined which repository your bug is related to, create an issue on that repository and provide the following information by filling in [the template](ISSUE_TEMPLATE.md).

Explain the problem and include additional details to help maintainers reproduce the problem:

- **Use a clear and descriptive title** for the issue to identify the problem.
- **Describe the exact steps in which reproduce the problem** in as many details as possible. For example, start by explaining how you started CITA, e.g. which command exactly you used in the terminal.
- **Provide specific examples to demonstrate the steps**. For example, links to files that you use. If you’re providing snippets in the issue, use Markdown code blocks.
- **Describe the behavior you observed after following the steps above** and point out what exactly is the problem with that behavior.
- **Explain which behavior you expected to see instead and why**

Include details about your configuration and environment:

- **Which version of CITA**?
- **Which version of the OS**?
- **Are you running CITA in a virtual machine?** If so, which VM software (including docker) and which operating systems and versions are used for the host and the guest?

### Pull Request

- Fill in [the required templates](PULL_REQUEST_TEMPLATE.md)
- Document new code based on the [Code Documentation Styleguide](#code-docs-style)

## Styleguides

### Git Commit Messages

- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line
- When only changing documentation, include `[skip ci]` in the commit title
- Consider starting the commit message with an applicable emoji:
    - :art: `:art:` when improving the format/structure of the code
    - :racehorse: `:racehorse:` when improving performance
    - :scroll: `:scroll:` when writing docs
    - :penguin: `:penguin:` when fixing something on Linux
    - :apple: `:apple:` when fixing something on macOS
    - :checkered_flag: `:checkered_flag:` when fixing something on Windows
    - :bug: `:bug:` when fixing a bug
    - :fire: `:fire:` when removing code or files
    - :green_heart: `:green_heart:` when fixing the CI build
    - :white_check_mark: `:white_check_mark:` when adding tests
    - :arrow_up: `:arrow_up:` when upgrading dependencies
    - :arrow_down: `:arrow_down:` when downgrading dependencies
    - :shirt: `:shirt:` when removing linter warnings

### Rust Styleguide

All Rust must adhere to [Rust Styleguide](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md)

Use [Rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) to improve your Rust codes by catching common mistakes.

Todo List

- [ ] Use [Highfive](https://github.com/rust-lang-nursery/highfive) to assign pull requests to users based on rules in configuration files.
- [ ] Read [Rust API guidelines](https://github.com/rust-lang-nursery/api-guidelines).

#### Code Docs Style

Here we use [rustdoc](https://doc.rust-lang.org/book/first-edition/documentation.html) as our code docs style standard.

### Documentation Styleguide

Documents are essential parts to communicate with our users. We need to make our documents well-structured and readable. Everyone who contributes to the documents should have a **strong responsibility** for creating the great experience.

- Everyone who modifies the documents needs to visualize the documents before making a pull request, in case there are some mistakes.

- Separate Chinese and English characters by a blank space. [Chinese Version Only]
    - Normally we need to add a blank space before the first character of English words and after the last character of English words.
    - If the character before the first character of English words is punctuation mark, then we don’t need to add a blank space.
    - If the character after the last character of English words is punctuation mark, then we don’t need to add a blank space.

    Example:

    - Bad Style：

        我们采用 CITA 作为我们的blockchain基础设施服务。

    - Good Style：

        我们采用 CITA 作为我们的 blockchain 基础设施服务。

- Separate Chinese characters and numbers by a blank space. [Chinese Version Only]
    - Detailed rules are same as above.

    Example:

    - Bad Style:

        CITA 的速度超乎想象，比我用过的其他10几个区块链服务都要快。

    - Good Style:

        CITA 的速度超乎想象，比我用过的其他 10 几个区块链服务都要快。

- Please do remember to indent the contents below any ordered list or unordered list.

    Example:

    ![Bad Style](https://ws4.sinaimg.cn/large/006tKfTcly1frtlvhnbfdj31kw0zkjvu.jpg)

    ![Good Style](https://ws3.sinaimg.cn/large/006tKfTcly1frtlwl2hq9j31kw0zktd5.jpg)
