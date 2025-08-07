# Building Guide for VSS-vspec

This guide should help to build a VSS based on our own VSS structure.

## VSS Tools installation

The installation of the VSS Tools and the activated python virtual environment are well explained in the guide of the [VSS Tools repo](https://github.com/COVESA/vss-tools?tab=readme-ov-file#installation).</br>
Neverthless it is optimized for the Linux operating system. We use Windows on our local machines so here is a short guide update for Windows users:

1. Have Python installed to create a virtual environment.
2. The `source` command is not available in Windows. Instead use `.\.venv\Scripts\Activate.ps1` (Powershell) or `call .\.venv\Scripts\activate.bat` (CMD) to activate the virtual environment.
3. Follow the guide in the repo (we used the default branch)

## Building the VSS

Now that everything is set up you can either use the CLI for help or just use the final command below to build the VSS.

``` powershell
vspec export json -s .\VSS\VehicleSignalSpecification.vspec -o own_vss.json -I VSS --pretty -l .\VSS\overlays\profiles\TeleopVehicle.vspec -l .\VSS\overlays\extensions\Teleop.vspec -l .\VSS\overlays\extensions\Teleop\Teleop.vspec -l .\VSS\overlays\extensions\DBC\DBC_mapping.vspec
```

This command requires to have the python virtual environment activated on the root level and the VSS Tools installed. The output will be in the file `own_vss.json` which contains the built VSS based on our structure of this repository.
