# Zenoh-Kuksa-Provider

The Zenoh-Kuksa-Provider is a custom interface acting as the bridge between the zenoh network and the Kuksa Databrokers vehicle signals.
The provider uses the databrokers gRPC API to subscribe to specific signals, observing changes in value, and publish new values to them.
It also uses the Zenoh API to subscribe/publish to specific zenoh keys.

Internally every change to a vehicle signal in the databroker or to a key in the zenoh network will result in the provider
transforming them into the other and publishing the new message in the corresponding "network" thus acting as the bridge between
the two technologies.