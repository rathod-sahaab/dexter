# Dexter

Dexter is an electronic lock. The way to open and close this lock is what makes it unique. Instead of pressing one key at a time you push multiple.

Normal pass-code/pin locks use a mechanism where you can activate one key/switch at a time, for this you need a lot more keys/switches for given range.

Example:

```
1   2   3
4   5   6
7   8   9
    0
```

Here you can have 10 keys giving you 0-9 and then you can chain those into some thing like `0-9-1-8-2`. This is waste of resources.

Dexter takes a better approach where you can press multiple keys a once to create a combination. Example with 5 keys your key pad will look like.

```
 1 1 1 1 1
```

_--- or ---_

```
16 8 4 2 1
```

Now you can enter binary number between `1-30` for example

| To enter | Combination |
| -------- | ----------- |
| 10       | `01010`     |
| 15       | `01111`     |
| 27       | `11011`     |

5 bit range is `0-31` Why not 0? You can't press no keys and register it as a key press. To enter 0 you need complex timing mechanism or extra key which you can use better to double the range.

You need significantly less number of keys but more **Dexterity** , hence the name **Dexter**.

So with same number of keys you can achieve more range of number, mathematically.

$$
range\ increased = \frac{\left(2^n - 2\right)}{n}\ times
$$

Which is near exponential increase.

## Usage

### Philosophy

Dexter user experience is designed keeping few things in mind

1. Humans make mistakes.
2. Humans mostly realise they have made a mistake halfway through.
3. Humans try to course correct the mistake.

### Structure

```
(3) (4)
(5) (6) (7) (8) (9)...

(1) (2) [ 1 ] [ 2 ] [ 3 ] [ 4 ] [ 5 ]
```

**LED indicators:**

- `(1)` Activity Indicator
- `(2)` Digit Registered Indicator
- `(3)` Success Indicator
- `(4)` Wrong Password Indicator
- `(5+)` Digits entered Indicator

**Keys:**

- `[1-5]` Keys ordered in MSB order

### Workflows

**Login Flow:**

1. Initially the lock is in standby, `(1)` is off, to activate it press first key then `(1)` pops up.
2. Start with your combination first digit in binary say `01101` when your are holding down given combination `(2)` is blinking indicating you can press more keys.
3. When you lift your hand up, `(2)` stops blinking momentarily indicating your digit input has been recorded. One more LED lights up in progress bar made up of `(5+)`
4. Once you enter correct password, lock opens and `(3)` stays lit up.
5. Incase of wrong password, `(4)` lights up for a few seconds.

**Password Change flow:**

1. Once lock is unlocked, press all keys, `(3)` starts blinking, now you can enter combination like in the login flow.
2. Fill all the digits indicated by `(5+)` progress bar.
3. **NOTE:** You can't use `00000` and `11111` in this passcode

**Forgot Password:**
1. Break the lock, if you were smart enough to created a reset button press that enter default password.

## Dev Containers
This repository offers Dev Containers supports for:
-  [VS Code Dev Containers](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-an-existing-folder-in-a-container)
-  [GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-codespaces/creating-a-codespace)
> **Note**
>
> In [order to use GitHub Codespaces](https://github.com/features/codespaces#faq)
> the project needs to be published in a GitHub repository and the user needs
> to be part of the Codespaces beta or have the project under an organization.

If using VS Code or GitHub Codespaces, you can pull the image instead of building it
from the Dockerfile by selecting the `image` property instead of `build` in
`.devcontainer/devcontainer.json`. Further customization of the Dev Container can
be achieved, see [`.devcontainer.json` reference](https://code.visualstudio.com/docs/remote/devcontainerjson-reference).

When using Dev Containers, some tooling to facilitate building, flashing and
simulating in Wokwi is also added.
### Build
- Terminal approach:

    ```
    scripts/build.sh  [debug | release]
    ```
    > If no argument is passed, `release` will be used as default


-  UI approach:

    The default build task is already set to build the project, and it can be used
    in VS Code and GH Codespaces:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Build Task` command.
    - `Terminal`-> `Run Build Task` in the menu.
    - With `Ctrl-Shift-B` or `Cmd-Shift-B`.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build`.
    - From UI: Press `Build` on the left side of the Status Bar.

### Flash

> **Note**
>
> When using GitHub Codespaces, we need to make the ports
> public, [see instructions](https://docs.github.com/en/codespaces/developing-in-codespaces/forwarding-ports-in-your-codespace#sharing-a-port).

- Terminal approach:
  - Using `flash.sh` script:

    ```
    scripts/flash.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

- UI approach:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Flash`.
    - From UI: Press `Build & Flash` on the left side of the Status Bar.
- Any alternative flashing method from host machine.


### Wokwi Simulation

#### VS Code Dev Containers and GitHub Codespaces

The Dev Container includes the Wokwi Vs Code installed, hence you can simulate your built projects doing the following:
1. Press `F1`
2. Run `Wokwi: Start Simulator`

> **Note**
>
>  We assume that the project is built in `debug` mode, if you want to simulate projects in release, please update the `elf` and  `firmware` proprieties in `wokwi.toml`.

For more information and details on how to use the Wokwi extension, see [Getting Started] and [Debugging your code] Chapter of the Wokwi documentation.

[Getting Started]: https://docs.wokwi.com/vscode/getting-started
[Debugging your code]: https://docs.wokwi.com/vscode/debugging

> **Warning**
>
>  ESP32-C2 is not, yet, not supported in Wokwi.


