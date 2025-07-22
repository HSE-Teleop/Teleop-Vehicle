# Own VSS structure

The Vehicle Signal Specification (VSS) is really important because it undermines the structure of communication between most components. It defines how signals are structured, what they mean, and how they can be used in the vehicle's systems.

## Default VSS structure

Our structure is inspired and based on the default catalog of the [COVESA VSS](https://github.com/COVESA/vehicle_signal_specification/tree/master/spec).</br>
The structure begins within the `VSS` folder where the top-level `VehicleSignalSpecification.vspec` file and the necessary quantities, units and overlays are located.
The `units.yaml` file contains the default definitions of units used in the VSS and includes the `quantities.yaml` file which defines the necessary quantities for the units. Those files are copied from the default catalog. As an entrypoint there is the `VehicleSignalSpecification.vspec` file which contains the main structure of our own VSS. Here we define the branches and attributes we want to use from the default catalog.

## Overlays

Because we are using a teleoperated vehicle we have created an overlay under `overlays/profiles/TeleopVehicle.vspec` which contains the signals that are specific to our teleoperated vehicle. The profiles overlay is used to define profiles based on the default catalog. Some attributes may be changed or added to fit our use case.

For a more extensive overlay we used extansions under `overlays/extensions/Teleop.vspec` which contains signals not specified in the default catalog. This allows us to extend the VSS with additional signals that are specific to our teleoperated vehicle. The `Teleop.vspec` file adds the branches and the vspec files under `overlays/extensions/Teleop` contain the additional signals.

Normally a larger overlay is done only using a profile and an extension is used to apply small changes after applying the profile. In our case we considered to split the overlay for the teleoperated vehicle for better maintainability and understanding.

### Overlay structure

Our specification has two main groups. One for the telemetry data coming from the vehicle and one for the teleoperation commands sent to the vehicle.</br>
The telemetry data contains signals that are related to the vehicle's state and can be found in the default catalog. That is why we use the profile overlay to select the certain signals and modify them if needed.</br>
Teleoperation commands are not defined in the default catalog and are therefore implemented in our extension overlay. The teleoperation commands are grouped under the `Teleop` branch and contain signals to control the vehicle.

## Summary

The `VehicleSignalSpecification.vspec` file is the main entrypoint for our own VSS structure. After that our profile overlay (`overlays/profiles/TeleopVehicle.vspec`) is applied which contains the signals for our telemetry data. In addition to the profile we have an extension overlay (`overlays/extensions/Teleop.vspec`) which contains the teleoperation commands and appends the signals via a new branch `Teleop`.
