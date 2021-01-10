# dice-combat

Experimental dice combat game.

- Server is responsible for maintaining the game state, and accepting inputs: 1) player input via WebSocket, 2) AI input.
- Client is responsible for displaying the current game state, as well as giving 'local player' choices.

## How to deploy

Client
```
cargo make deploy <checkpoint name>
```

## How to run locally

Server (listens on port 9000)
```
cargo run
```

Client (listens on port 8000)
```
cargo make serve
cargo make watch
```

## TODO

Client/server TODO
- [ ] Client side: Display the combat log
- [ ] Implement the rest of the basic phases: selecting actions and targets.
- [ ] Figure out how to deploy this. Would be cool if WebSocket part could be deployed to CDN somehow? Cloudflare workers?

Actual game TODO
- [ ] implement Defend mechanics in `action_system`
- [ ] experiment with different ways of calculating bonus damage in heavy attack
- [ ] build an AI System can drive the enemies (starting with randomly selecting valid actions)
- [ ] multiplayer support (how would this even work?)