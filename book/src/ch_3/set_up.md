To ensure we know what is going on in the source code, developers usually:

- `dbg!` or `print!` information to know the value of a variable.
- `assert!` expected values
- Write unit tests

Ocassionally, we need more details. For example, in a mathematical algorithm, to inspect many different variables.

To access that finer details, we can use a *debugger*.

## Software needed

The main programs are:
- GNU Debugger (`gdb`): inspect and stop the program as it runs.
- OpenOCD (`openocd`): handles the communication with the board.

Linux has `gdb` installed; on MacOS, install it with `brew install gdb`.

Download the latest `openocd` for your laptop architecture in the `openocd-esp32` [github repo]. The repo is a fork of `openocd` targetted at `esp32` boards.

Below, there are suggested commands for MacOS-arm64, and Linux-amd64, check the link above for other architectures.

> [!IMPORTANT]
> Please, check the commands before executing them.

- For Linux amd-64:
    ```bash
    # Linux amd64. See releases page for other archs. 
    cd ${HOME} # go to home so we download in a visible place.
    OPENOCD_ZIP_NAME=openocd-esp32-linux-amd64-0.12.0-esp32-20250707.tar.gz
    DATE=v0.12.0-esp32-20250707
    wget https://github.com/espressif/openocd-esp32/releases/download/${DATE}/${OPENOCD_ZIP_NAME}
    tar -xvf ${OPENOCD_ZIP_NAME}
    ```
- For MacOS arm64:
    ```bash
    cd ${HOME}
    OPENOCD_ZIP_NAME=openocd-esp32-macos-arm64-0.12.0-esp32-20250707.tar.gz
    DATE=v0.12.0-esp32-20250707
    wget https://github.com/espressif/openocd-esp32/releases/download/${DATE}/${OPENOCD_ZIP_NAME}
    tar -xvf ${OPENOCD_ZIP_NAME}
    ```

> [!IMPORTANT]
> Below, it is assumed the output of the command above was a directory named `openocd-esp32`.

## Adding it to PATH

`PATH` is list of directories (separated with `:`) where our system searches for binaries.

We will add a location to `PATH`, by editing the `.zshrc` or `.bashrc` file. 

Additionally, the `OPENOCD_SCRIPTS` variable is defined. `openocd` uses that variable to find the board's configuration.

```bash
# The variable names must be PATH and OPENOCD_SCRIPTS.
# add `openocd` to path
export PATH=${HOME}/openocd-esp32/bin:${PATH}
# define the config path
export OPENOCD_SCRIPTS="${HOME}/openocd-esp32/share/openocd/scripts"
```

Then source the profile:

```bash
source ~/.bashrc # or ~/.zshrc
```

Then check it is installed with `openocd --version`. 

Pheww!


[github repo]: https://github.com/espressif/openocd-esp32/releases
