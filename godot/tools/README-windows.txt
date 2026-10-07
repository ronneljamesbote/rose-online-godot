ROSE (Godot client) for Windows
===============================

1. Unzip this folder anywhere, for example C:\Games\ROSE.
2. Put the iROSE 129_129 client next to ROSE.exe, so that this file exists:
       ROSE\iRose_129_129\data.idx
   (Somewhere else is fine too: the game then asks for data.idx once and remembers it.)
3. Start ROSE.exe. Windows may warn that the app is unrecognised: click
   "More info", then "Run anyway".
4. Type the server address you were given, a name (1-20 letters) and pick a weapon,
   then click Connect. "Play offline" walks around Zant alone.

Controls
  Left-click the ground     run there
  Left-click a monster      attack it (Space attacks the nearest one)
  S                         stop
  Right-drag                turn the camera
  Mouse wheel               zoom

Your character is tied to this PC (the key is kept in
%APPDATA%\Godot\app_userdata\ROSE (Godot)\). Copy identity-default.token from there
to keep the same character on another PC.

Needs a graphics card with Vulkan or Direct3D 12 (most PCs from 2016 on).
