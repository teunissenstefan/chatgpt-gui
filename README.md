# chatgpt-gui

![cover image](https://github.com/teunissenstefan/chatgpt-gui/blob/master/cover.png?raw=true)


## Building
```shell
git clone git@github.com:teunissenstefan/chatgpt-gui.git
cd chatgpt-gui
cargo build --release
mkdir -p $HOME/.local/share/glib-2.0/schemas
cp src/org.teunissenstefan.ChatGPT.gschema.xml $HOME/.local/share/glib-2.0/schemas/
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
```