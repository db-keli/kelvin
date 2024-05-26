# Contributing to Kelvin

Thank you for considering contributing to [Your Project Name]! Your help is appreciated and necessary for the growth and improvement of this project. Please follow the guidelines below to make the process smooth and effective.

## Table of Contents

1. [How to Contribute](#how-to-contribute)
2. [Reporting Issues](#reporting-issues)
3. [Submitting Pull Requests](#submitting-pull-requests)
4. [Code Style and Guidelines](#code-style-and-guidelines)
5. [Commit Messages](#commit-messages)
6. [Contact](#contact)

## How to Contribute

1. **Fork the repository**: Click the "Fork" button on the top right of the repository page.
2. **Clone your fork**: 
    ```bash
    git clone https://github.com/db-keli/kelvin.git
    ```
3. **Create a branch**: 
    ```bash
    git checkout -b feature/your-feature-name
    ```
4. **Make your changes**: Implement your feature, bug fix, or documentation improvement.
5. **Commit your changes**: 
    ```bash
    git commit -m "Add your commit message here"
    ```
6. **Push to your fork**: 
    ```bash
    git push origin feature/your-feature-name
    ```
7. **Create a pull request**: Go to the original repository and click the "New Pull Request" button. Fill in the necessary details and submit.

## Reporting Issues

If you find any bugs, please open an issue on GitHub with the following details:

- A clear and descriptive title.
- A detailed description of the problem.
- Steps to reproduce the issue.
- Any relevant logs or screenshots.
- The expected and actual results.

## Submitting Pull Requests

When submitting a pull request:

1. Ensure that your changes are well-documented.
2. Ensure that your code follows the project's coding standards.
3. Ensure that your code passes all existing tests and includes new tests for new features.
4. Provide a clear and descriptive title for your pull request.
5. Provide a detailed description of your changes in the pull request description.
6. Link the issue that your pull request resolves (if applicable).

## Code Style and Guidelines

Please follow these guidelines to maintain code consistency:

- Follow the existing code style in the project.
- Write clear, concise, and well-documented code.
- Ensure that your code is tested.
- Use meaningful commit messages (see below).

## Commit Messages

Please follow these conventions for commit messages:

- Use the present tense ("Add feature" not "Added feature").
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...").
- Limit the first line to 72 characters or less.
- Reference issues and pull requests liberally.

Example commit message:
```markdown
Add feature to convert Markdown to HTML

- Implement Marked.js library for Markdown conversion.
- Add functionality to fetch Markdown files from the server.
- Update HTML and CSS to display converted content.

Resolves #42
