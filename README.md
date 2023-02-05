# ChatGPT GUI

![cover image](https://github.com/teunissenstefan/chatgpt-gui/blob/master/cover.png?raw=true)

## Building

```shell
git clone git@github.com:teunissenstefan/chatgpt-gui.git
cd chatgpt-gui
cargo build --release
```

## Todo

* Connect insert_text to only allow digits in certain fields in preferences
* Show confirmation dialog when pressing the X button
* Prefer finish_reason "stop", otherwise, notify the user
* Copy message when double clicking on it, maybe context menu
* Automatically scroll down when a new message has been added
* Continue conversation