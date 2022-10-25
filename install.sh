#!/usr/bin/env bash
mkdir -p ~/.config/vlc-autoskip
mkdir -p ~/.local/bin

cargo build --profile release

sudo chmod +x target/release/vlc-autoskip
cp target/release/vlc-autoskip ~/.local/bin/vlc-autoskip
cp settings.json ~/.config/vlc-autoskip/settings.json
cp VLCAutoskip.desktop ~/.local/share/applications/VLCAutoskip.desktop