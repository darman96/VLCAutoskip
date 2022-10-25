#!/usr/bin/env bash
mkdir -p ~/.config/vlc-autoskip
mkdir -p ~/.local/bin

cargo build --profile release

sudo chmod +x target/release/vlc-autoskip
cp target/release/vlc-autoskip ~/.local/bin/vlc-autoskip
cp settings.json ~/.config/vlc-autoskip/settings.json

rm ~/.local/share/applications/VLCAutoskip.desktop
cat <<EOT >> ~/.local/share/applications/VLCAutoskip.desktop
[Desktop Entry]
Name=VLCAutoskip
Exec=vlc-autoskip
Icon=vlc
Terminal=true
Type=Application
EOT
