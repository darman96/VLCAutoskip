#!/usr/bin bash

cargo build --profile release
cp target/release/vlc-autoskip ~/.local/bin/vlc-autoskip
chmod +x ~/.local/bin/vlc-autoskip

mkdir ~/.config/vlc-autoskip
cp ./settings.json ~/.config/vlc-autoskip/settings.json

cat <<EOT >> ~/.local/share/applications/VLCAutoskip.desktop
[Desktop Entry]
Name=VLCAutoskip
Exec=vlc-autoskip
Icon=steam_icon_945360
Terminal=true
Type=Application
Categories=Other;
EOT
