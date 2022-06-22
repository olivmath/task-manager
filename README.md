# 🕹 Task Manager CLI

## How to Install

### Linux
1. download the `tasker-linux` [here](https://github.com/olivmath/task-manager/releases)
2. `mv tasker-linux /usr/bin/tasker`
3. `chmod +x /usr/bin/taker`

### Windows
1. download the `tasker-windows.exe` [here](https://github.com/olivmath/task-manager/releases)
2. `rename tasker-windowns.exe tasker.exe`
3. move `tasker.exe` to `C:\Windows\tasker.exe`
4. open start menu:
    - search for **edit environment variables** and open
    - click in **environment variables** > **system variables** > **new**
    - **variable name:** `tasker`
    - **variable value:** `C:\Windows\tasker.exe`
5. **restart the command prompt**

## How to use

- You can write new task:
  - `tasker add "my new task"`
- You can done tasks:
  - `tasker done <index_of_task>`
- Finally you can list all tasks:
  - `tasker list`
