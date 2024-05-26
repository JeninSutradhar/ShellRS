# Shell RS
ShellRS is a powerful shell implemented in Rust. It supports advance command execution, piping, redirection, and this enhanced shell properly handle opening applications, executing commands with elevated privileges, and improving the overall user experience.


## Features

- **Basic Commands:** Execute common commands like ls, pwd, echo.
- **Directory Navigation:** Change and list directories.
- **Piping and Redirection:** Use pipes and redirections to manipulate command output.
- **Error Handling:** Gracefully handle invalid commands.
- **Environment Variables:** Set and use environment variables.
- **Custom Prompts and Colors:** Enjoy a customizable and colorful prompt.
- **History and Tab Completion:** Navigate through command history and use tab completion for efficiency.


## Usage
Once you start ShellRS, you can begin typing commands just like you would in any other shell.

### Basic Commands

- **List Files (`ls`)**:
    ```sh
    > ls
    ```
    This command lists all files and directories in the current directory.

- **Print Working Directory (`pwd`)**:
    ```sh
    > pwd
    ```
    This command displays the full path of the current directory.

- **Echo Text (`echo`)**:
    ```sh
    > echo Hello, ShellRS!
    ```
    This command outputs the specified text to the terminal.

### Directory Navigation

- **Change Directory (`cd`)**:
    ```sh
    > cd src
    ```
    This command changes the current directory to `src`.

- **List Files in New Directory (`ls`)**:
    ```sh
    > ls
    ```
    After changing the directory, you can list the files in the new directory.

### Piping and Redirection

- **Pipe Commands**:
    ```sh
    > ls | grep Cargo
    ```
    This command lists all files and directories, and then filters the output to show only those containing "Cargo".

### Error Handling

- **Invalid Command**:
    ```sh
    > invalidcommand
    ```
    This command will show an error message indicating that the command was not found.

- **Invalid Directory**:
    ```sh
    > cd nonexistentdirectory
    ```
    This command will show an error message indicating that the directory does not exist.

### Environment Variables

- **Set Environment Variable**:
    ```sh
    > export MYVAR=myvalue
    ```
    This command sets an environment variable `MYVAR` with the value `myvalue`.

- **Use Environment Variable**:
    ```sh
    > echo $MYVAR
    ```
    This command outputs the value of `MYVAR`.

### Custom Prompts and Colors

- **Custom Prompt**:
    When you start ShellRS, you will see a colorful prompt:
    ```sh
    > 
    ```
    This prompt is customizable and adds a visual appeal to your shell experience.

### History and Tab Completion

- **Command History**:
    Use the up and down arrow keys to navigate through your command history.

- **Tab Completion**:
    Start typing a command or file name and press `Tab` to auto-complete.

To exit the shell, you can type:

```sh
> exit