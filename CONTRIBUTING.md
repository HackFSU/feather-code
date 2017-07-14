# Contributing

:+1::tada: First off, thanks for taking the time to contribute! :tada::+1:

# Your First Contribution

We welcome issues with questions, suggested enhancements, or bugs.  Please provide as much information as possible in your issue.

## How Do I Submit A (Good) Issue for Suggesting an Enhancement or Identifying a Bug?

Enhancement suggestions are tracked as GitHub issues. Create an issue and try to do as many of the following as possible:

- Use a clear and descriptive title for the issue to identify the suggestion
- If you've found a bug, describe the behavior you observed after following the steps and point out what exactly is the problem with that behavior.
- Provide a step-by-step description of the suggested enhancement or bug replication in as many details as possible
- Provide specific examples to demonstrate the steps. Include any copy/pasteable snippets which you use in those examples as Markdown code blocks
- Describe the current behavior and explain which behavior you expected to see instead and why
- Include screenshots and animated GIFs that show bugged or poor behavior that should be improved; a picture is worth a thousand words
- Specify the name and version of the OS you're using

# Style Guide

## Git Commit Messages

- Use present tense, imperative mood for commit messages as in:
    - > Add test for `poly_eval` covering edge case
    - > Refactor inner loop with borrows for improved performance
    - > Add example to elucidate documentation for matrix type
- Treat commit messages like emails, with subject and body (if necessary)
- Limit the frist line to 72 characters or less
- When changing only documentation (that doesn't require doc tests) add `[ci skip]` to the commit description
- Reference pull requests and issues liberally after the first line

# Hooks

## pre-commit

We use a pre-commit hook to automate testing on each commit.  To enable the hook on a UNIX-like OS with a [symbolic](https://erictleung.com/the-ln-command) link simply type:

```
$ ln -s -f ../../hooks/pre-commit.sh .git/hooks/pre-commit
```

Otherwise, you can copy the pre-commit hook into your .git folder with:

```
$ cp hooks/pre-commit.sh .git/hooks/pre-commit
```

or by manually copying the file with your file explorer utility.

> Note: if you copy the pre-commit hook it won't get updates from upstream, so a symbolic link is preferred if at all possible.
