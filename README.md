# Pico - Project Task Management CLI

Pico is a lightweight and efficient command-line tool designed to help you manage your project tasks with ease. Annoyed that you can't track your projects tasks without having to leave your trusty terminal? With Pico you can check out your tasks, who is working on what right from your terminal.

## Features

- **Effortless Task Tracking**: Quickly create, update, and organize tasks from the command line, ensuring you never lose sight of your project's progress.
- **Tied to Git repos**: Have the project tasks located directly in the repository where they are relevant. Pico let's you populate its user list by scraping users from git commits.
- **Interactive Commands**: Pico makes use of interactive prompting to make it easy to manage the tasks of your project without having to remember what the ids are for tasks or users.

## Installation

To install Pico, follow these simple steps:

1. **Prerequisites**: Make sure you have Rust installed on your system.
2. **Install Pico**: Open your terminal and run the following command:

```shell
cargo install pico-cli
```

## Quick Guide

Navigate to the git repository and run the following command to initialize the project:
```shell
pico init
```
This command takes you through setting up your initial task categories, tasks and users.

To see all tasks of the project
```shell
pico list (--json for json output) (--detailed for more details)
```

To see all of the tasks assigned to you
```
pico me
```















