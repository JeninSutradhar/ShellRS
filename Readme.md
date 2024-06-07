# Shell RS
ShellRS is a powerful shell implemented in Rust. It supports advance command execution, piping, redirection, and this enhanced shell properly handle opening applications, executing commands with elevated privileges, and improving the overall user experience.

```
   _____ __         ____   ____  _____    
  / ___// /_  ___  / / /  / __ \/ ___/    
  \__ \/ __ \/ _ \/ / /  / /_/ /\__ \     
 ___/ / / / /  __/ / /  / _, _/___/ /     
/____/_/ /_/\___/_/_/  /_/ |_|/____/      
```

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

![shellfull](https://github.com/JeninSutradhar/ShellRS/assets/111521642/e6558dad-4a77-43b5-9ae8-e46b72b5798c)

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


### History and Tab Completion

- **Command History**:
    Use the up and down arrow keys to navigate through your command history.

- **Tab Completion**:
    Start typing a command or file name and press `Tab` to auto-complete.

To exit the shell, you can type:

```sh
> exit
