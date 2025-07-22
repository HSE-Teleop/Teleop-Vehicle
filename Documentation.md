# Documentation Notes

## Technologies
- Eclipse Zenoh
- Eclipse Kuksa-Databroker & CLI
- Covesa VSS
- GStreamer
- Rust
- Kuksa-Can-Provider
- Raspberry PI 5 + HQ Camera
- Zenoh-Kuksa-Provider built ourselves

## What did we already do?
- Began with looking at all the Technologies
- In a WSL 2 testing environment tried to connect the technologies together
  - Zenoh, Databroker, Databroker-CLI, Zenoh-Kuksa-Provider (official)
- Tried out Gstreamer for the first time in WSL
- Came to the conclusion that the official Zenoh-Kuksa-Provider did not work for our purposes (did not correctly connect Zenoh to the Databroker)
- Started implementing all of the above in a Developement Environment
- Zenoh, Databroker, Databroker CLI all run in Docker Containers
- Added Zenoh-Publisher and Zenoh-Subscriber to the project also running inside Containers
- Started implementation of our own connection between Zenoh and Databroker
- Tested own Zenoh-Kuksa-Provider
- Started getting promising result with own implementation, but a few problems remain
- Added caching of messages to Provider to break infinite feedback loop
- Testing of whole system using Kuksa-Can-Provider
- Replay of Tesla Model 3 test drive over can-provider feeding signals into databroker
- Result: VSS-Paths get updated correctly, Provider forwards changes to Zenoh network correctly