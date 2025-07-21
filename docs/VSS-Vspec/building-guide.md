# Building Guide for VSS-Vspec









Follow the installation guide on the [vss-tools repo](https://github.com/COVESA/vss-tools?tab=readme-ov-file#installation).








Final command:

``` cmd
vspec export json -s .\VSS\VehicleSignalSpecification.vspec -o own_vss.json -I VSS --pretty -l .\VSS\overlays\profiles\TeleopVehicle.vspec -l .\VSS\overlays\extansions\Teleop.vspec -l .\VSS\overlays\extansions\Teleop\Teleop.vspec
```
