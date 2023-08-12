# Piqo - Project Task Management CLI

Piqo is a lightweight and efficient command-line tool designed to help you manage your project tasks with ease. Annoyed that you can't track your projects tasks without having to leave your trusty terminal? With Piqo you can check out your tasks, who is working on what right from your terminal.

## Features

- **Effortless Task Tracking**: Quickly create, update, and organize tasks from the command line, ensuring you never lose sight of your project's progress.
- **Tied to Git repos**: Have the project tasks located directly in the repository where they are relevant. Piqo let's you populate its user list by scraping users from git commits.
- **Interactive Commands**: Piqo makes use of interactive prompting to make it easy to manage the tasks of your project without having to remember what the ids are for tasks or users.

## Installation

To install Piqo, follow these simple steps:

1. **Prerequisites**: Make sure you have Rust installed on your system.
2. **Install Piqo**: Open your terminal and run the following command:

```shell
cargo install piqo
```

## Quick Guide

Navigate to the git repository and run the following command to initialize the project:
```shell
piqo init
```
This command takes you through setting up your initial task categories, tasks and users.

To see all tasks of the project
```shell
piqo list (--json for json output) (--detailed for more details)
```

To see all of the tasks assigned to you
```
piqo me
```















