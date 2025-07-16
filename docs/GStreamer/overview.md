# GStreamer

Gstreamer is a multimedia framework that allows creation of streaming media applications. It provides features for building 
streaming pipelines that can handle any kind of data flow.

In this project GStreamer is used to send live video, captured by a camera on the model car, to the operation center.
This should ensure low-latency real time video streaming needed for teleoperation.

## Pipelines
There are two distinct pipelines that will be created and used. One is the Sending/Transmitting pipeline that captures the video from the camera
and sends it to the operation center using UDP. The other is the Receiving pipeline in the operation center that
captures the sent UDP packages and converts them back into video data for display.
GStreamer graciously handles the method of display on its own reducing the amount of manual labour needed for set up.

Pipelines can consist of a number of different components to manipulate the raw data.
In our case the transmitting side first captures the video from a source in a particular format, converts the data to a more suitable format,
wraps the data and sens it over UDP to the operation center. The receiving side then matches the transmitters configuration of the data,
strips any headers, decodes into raw video and then converts raw frames for display automatically picking a display method.

In this project the Data will be sent from one Raspberry Pi on the car to another raspberry pi in the operation center.

## Use in Zenoh
This section will feature documentation about the suitability of gstreamer to be used inside the Zenoh network.