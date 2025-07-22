# CAN-Provider

The CAN-Provider acts as a bridge between automotive CAN networks and the Kuksa Databroker.
It receives CAN messages either live from the subsystem of the Car or from a recorded candump file.
CAN messages are then decoded using a DBC file and mapped to VSS. Once they are mapped, the values can be written into the Databroker.
These values can subsequently be further transformed into Zenoh messages by the [Zenoh-Kuksa-Provider](../Zenoh-Kuksa-Provider).

## Testing
In this project the Kuksa CAN-Provider was first used as confirmation for the system. It was used with a live replay of a Tesla Model 3
test drive using candump, dbc and dbc-vss mapping files. This way the custom [Zenoh-Kuksa-Provider](../Zenoh-Kuksa-Provider)
that handles the communication between the Kuksa Databroker and Eclipse Zenoh could be tested and confirmed.
For confirmation the Databroker, the Databroker-CLI, Zenoh, the [Zenoh-Kuksa-Provider](../Zenoh-Kuksa-Provider), the CAN-Provider
and a custom Zenoh Subscriber were started. The CAN-Provider then fed the values of the replay into the Databroker.
This could be confirmed with the Databroker-CLI.

Using the Command: `````subscribe Vehicle.Speed````` inside the CLI, one could see the values of the signal be updated
by the CAN-Provider. In the next step the [Zenoh-Kuksa-Provider](../Zenoh-Kuksa-Provider) was used to transform VSS signals into Zenoh messages.
Similar to what was done in the CLI the [Zenoh-Kuksa-Provider](../Zenoh-Kuksa-Provider) subscribed to the ```Vehicle.Speed``` signal inside the Databroker.
The now transformed messages are sent to the Zenoh router which then distributes the messages to subscribers.
We used a custom subscriber that subscribes to all Keys inside Zenoh to confirm that the [Zenoh-Kuksa-Provider](../Zenoh-Kuksa-Provider) successfully
received the VSS signals, transformed them into Zenoh messages and sent them to the router.

## Live Use
This section will feature documentation of how the CAN-Provider will be used with live signals from the model Car.