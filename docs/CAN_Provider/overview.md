# CAN-Provider

The CAN-Provider acts as a bridge between automotive CAN networks and the Kuksa Databroker.
It receives CAN messages either live from the subsystem of the Car or from a recorded candump file.
CAN messages are then decoded using a DBC file and mapped to VSS. Once they are mapped, the values can be written into the Databroker.
These values can subsequently be further transformed into Zenoh messages by the [Zenoh-Kuksa-Provider](../Zenoh-Kuksa-Provider).

## Custom .dbc and vss_dbc mapping files
To use the CAN-Provider for our specific car a custom .dbc file had to be created. This was done by analyzing current candumps of the car
for CAN frames that would be of importance to us. For our purposes we concluded that frames with the IDs of 160 and 161 would be the most
usefull for us, as they are contain data about Speed/Acceleration, Steering and some time information. Frames with CAN-ID 160 are sent
by the car to the CAN-Provider. Frames with ID 161 are sent by the CAN-Provider to the Car. So 160 is the cars feedback information and
161 is control/command information initially sent from the operation center.

Apart from the .dbc file a new section in the model cars VSS had to be created, containing information about mapping between vss and dbc
information, so VSS paths can be associated with CAN frames.

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

Furthermore, the CAN-Provider was used to simulate the car for testing purposes. On a raspberry pi with can-utils installed, 
one could issue the following commands to create a virutal can interface:
<pre>
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
</pre>

Once this is done the Kuksa Databroker, the Kuksa Databroker-CLI and the CAN-Provider could be started using the docker compose.
To test the communication from the Databroker down to the car we can now use the Databroker-CLI to change a value of a path the
Can-Provider is subscribed to. For example:

<pre>
actuate Vehicle.Teleop.SteeringAngle 100
</pre>

The logs of the CAN-Provider are now confirming to have received a message on the correct path thus proving that the Kuksa Databroker
can communicate with the Car via the CAN-Provider.

## Live Use
This section will feature documentation of how the CAN-Provider will be used with live signals from the model Car.