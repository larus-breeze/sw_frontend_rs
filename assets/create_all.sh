#!/bin/sh

# stop script on error
set -e

python assets/create_colors.py
python assets/create_items.py
python assets/create_polar_idx.py
python assets/create_sound_samples.py
python assets/create_sprite_coords.py
python assets/create_wallpaper_editor.py
python assets/create_wallpaper_horizon.py
python assets/create_wallpaper_vario.py
python assets/convert_pictures.py
