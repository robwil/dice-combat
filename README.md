# dice-combat

Experimental dice combat game.

- Server is responsible for maintaining the game state, and accepting inputs: 1) player input via WebSocket, 2) AI input.
- Client is responsible for displaying the current game state, as well as giving 'local player' choices.

## How to deploy

**Server**

For faster Docker builds, we leverage [cargo-chef](https://github.com/LukeMathWalker/cargo-chef).

Only when dependencies changed:
```
cargo chef prepare --recipe-path recipe.json
```

Building the Docker image:
```
docker build . --tag gcr.io/robwil-io/dice-combat
docker push gcr.io/robwil-io/dice-combat
```

Edit the `server/service.yaml` file:
- Set spec.template.metadata.name to a new revision name
- Set spec.traffic[0].tag to the new traffic tag. This will be used in the new URL: `https://<tag>---dice-combat-sxrrowqjgq-uk.a.run.app/`

Deploying the new Docker image:
```
gcloud beta run deploy --image gcr.io/robwil-io/dice-combat --platform managed --port 9000 --tag <checkpoint name>
  -- pick Fully Managed (1) and us-east4 (20)
```

Every time we need to rebuild, can test the cargo-chef release build using this command:
( NOTE: This currently screws up Cargo.toml and src/main.rs due to https://github.com/LukeMathWalker/cargo-chef/issues/27 )
```
cargo chef cook --release --recipe-path recipe.json
```

**Client**

First, make sure to update the WS_URL constant in `client/src/client.rs` to point to the proper Cloud Run revision of a compatible server deployment.

Example: `wss://initial---dice-combat-sxrrowqjgq-uk.a.run.app/ws` for tag `initial`

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