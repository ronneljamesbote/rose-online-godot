ROSE server for Windows
=======================

Run it on the PC that hosts the game. Everyone else only needs ROSE-windows.zip.

First time
1. Unzip this folder anywhere, for example C:\Games\ROSE-server.
2. Double-click start-server.bat. If Windows asks whether to allow network access,
   allow it on private and public networks. Leave the window open.
3. Double-click install-game.bat once. It should end with
   "Created new database with name: rose".
4. On your router, forward TCP port 3000 to this PC (the router's "port forwarding"
   page; give the PC a fixed local address first).
5. Find your public address, for example at https://ifconfig.me, and give your
   players:   ws://YOUR-PUBLIC-ADDRESS:3000
   On this PC itself you can use ws://127.0.0.1:3000.

Every time after that, just start start-server.bat. Characters, HP and positions are kept
in the data folder; delete it to start a fresh world (then run install-game.bat again).

Keep the keys folder private: it signs the players' identities. If it is lost, every
player gets a new character.
