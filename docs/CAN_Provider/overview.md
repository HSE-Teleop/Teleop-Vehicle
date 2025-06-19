# CAN Provider

The can-provider functions like the [Zenoh-Kuksa-Provider](../Zenoh-Kuksa-Provider) and handles the communication on the other side between subsystem and kuksa databroker.
It receives and writes CAN messages to the subsystem and publishes them also in the kuksa databroker so that the data can then be fed into the zenoh network.