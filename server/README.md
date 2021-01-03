# dice-combat

Experimental dice combat game.

## TODO

Alternate stream
- Rather than continue down the current path, I could also try to break up the game into "server" and "client".
- Server would be responsible for maintaining the game state, and accepting inputs: 1) local player input (or is this just another "remote" player?), 2) remote player input, 3) AI input.
- Client would be responsible for displaying the current game state, as well as giving 'local player' choices.

There are a number of options for how to build this client/server:
- Client: I could go down the road of the multiplayer Snake game (https://youtube.com/watch?v=Yb-QR3Vm3sk) which uses a JS frontend with websocket communication to backend Rust server.
- Client: I could build frontend using Yew or Seed, to further learn Rust ecosystem.
- Client: Stick to my strengths and just build it in React
- Server: Keep the code basically as-is, but add incoming web hooks to replace and/or supplement the MegaUI inputs.
- Server: Totally redo everything to have a UI-less game state, optimizing for state management. Unclear if Specs would still be helpful here.

Tutorials that could help:
- https://blog.logrocket.com/websockets-tutorial-how-to-go-real-time-with-node-and-react-8e4693fbf843/
- https://blog.logrocket.com/how-to-build-a-websocket-server-with-rust/

Imagining the client/server contract:
- server sends event when it is client's turn: include the valid actions they can take
    server updates current state (i.e. "Player 1 is choosing action") for all clients
- client sends event when choosing action
    server updates combat log for all clients
- server sends event when client needs to choose target
    server updates current state (i.e. "Player 1 is choosing target") for all clients
- client sends event of which target was chosen
    server updates combat log for all clients
- server sends new game state; combatant state including any new damage/dice/status, plus new combat log entries
    (This could be dumb initially, just sending back entire state, since it should serialize to be pretty small.)

Original approach TODO
- [ ] implement Defend mechanics in `action_system`
- [ ] experiment with different ways of calculating bonus damage in heavy attack
- [ ] build an AI System can drive the enemies (starting with randomly selecting valid actions)
- [ ] multiplayer support (how would this even work?)