
# Contribution Rules

## Code Review & Contributions

1. This document is the final source of truth on acceptable format, styles, and decisions.
2. This document is a living document. If you have suggestions, please make a pull request.
3. This document is a guideline. If you have a good reason to break a rule, do it. Just make sure you have a good reason.
4. Architecture decisions should be documented in the github discussions or in the code itself.
5. Be kind, be constructive, and be helpful. Ain't no godkings here :).

## Code Guidelines

1. **Use `rustfmt`** to format your code. You can use the `cargo fmt` command to format your code.
2. **Eliminate warnings** from your code to the best of your ability. Warning-free code is preferred.
3. **Follow the patterns** you see. Innovation and deviation are welcome, but consistency is key.
3. Try to **keep your code clean** and readable. That means using whitespace, comments, and newlines - avoid code golfing & overly compact code.
4. **Use descriptive variable names**. Avoid single-letter & short variable names. (i.e. `let trunc_desc = "This is a truncated description";` is less preferred to `let truncated_desc = "This is a truncated description";`)
5. **Use descriptive function names**. Avoid single-word & short function names. (i.e. `fn add(a: i32, b: i32) -> i32 { a + b }` is less preferred to `fn add_two_numbers(number_one: i32, number_two: i32) -> i32 { number_a + number_b }`)
6. When you finalize your interfaces, make sure they are well-documented. **Use `///` for documentation comments, `//!` for module-level documentation, and `//` for regular comments throughout.**
7. **Modularize your code** and assume robust reuse. Break your code into smaller, reusable components which can respond dynamically to input.

## Style Guide
1. Propagate errors up the call stack or return them. Follow the Hitchhiker's Guide to the Galaxy! *Don't panic!* Don't swallow errors!
2. No `unwrap()` or `expect()` in production code. Use `match`,`if let`, or solid error-handling instead.
3. No `clone()`ing to run from the borrow checker :)
4. If you're returning a value after long and complicated code, use an explicit `return` statement so it's not lost in the sauce.
5. Pull constants out of your code and into a `constants.rs` in the module. This makes it easier to change hard-coded values later.
6. Use `anyhow` crate for error handling. It's a great library that makes error handling easier.

## Branching & Merging Rules

1. Create a new branch for each feature or bug fix.
2. Small changes can be made directly to the `main` branch.
2. Branch names should be descriptive and use kebab case. For example, `feat-sound-effects`.
3. Branch names should start with the type of change (add, bug, fix, feat, etc.).
4. Branches should be created from the `main` branch.
5. Branches should be merged into the `main` branch using a pull request.
6. To squash or not to squash? That is the question. Squashing is preferred for small changes, but not necessary for larger changes. Use your best judgment.
6. Branches which are to be merged should have WORKING code. No errors and minimal warnings.
6. Branches should be destroyed after merging.


## Pull Requests
1. Create a pull request when you're ready to merge your changes into the `main` branch.
2. Pull request titles should be descriptive and use title case (e.g. Capitalization). Emojis are welcome ;).
3. Pull request descriptions should include a short summary of changes and what's still WIP (Work In Progress).
4. Wait for at least one review before merging a pull request to main.


## Commits

1. Create a commit to track one unit of work. Atomic commits are preferred. Ideally, each commit should represent a "single" change (i.e. adding a new file, making a new component, refactoring an interface, etc.). Obviously, this is subjective, but the idea is to keep commits small and focused enough to follow.
2. Use the gitmoji standard to add appropriate icons to each commit: https://gitmoji.dev/
You can use the [gitmoji-cli tool written in js](https://github.com/carloscuesta/gitmoji) or [gimoji cli tool written in rust](https://github.com/carloscuesta/gitmoji) for speed and performance
3. Use the following format for commit messages:
    - :emoji: [area-of-focus] message
    - Example: :sparkles: [lex] Add `parse_in_full()` func
4. Use the present tense ("Add feature" not "Added feature").
5. Use the imperative mood ("Move cursor to..." not "Moves cursor to...").
6. Use capitalization
7. When referencing a file or function or variable, use backticks. For example, `main.rs` or `add()`.
8. Try to summarize specifics instead of generalizing (i.e. "Add `parse_func()` to context module" instead of "Update context module").
9. Messages are welcome! They can add context to the commit message.
10. Be fun :)
