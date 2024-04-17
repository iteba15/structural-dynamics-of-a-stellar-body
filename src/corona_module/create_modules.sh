#!/bin/bash

# Create a directory for the modules
mkdir -p corona_module

# Define the modules
modules=(
    "Advanced_Emission_Spectrum"
    "Magnetic_Reconnection"
    "Wave_Heating"
    "Particle_Acceleration"
    "Sophisticated_Solar_Wind_Model"
    "Data_Driven_Models"
)

# Loop through the modules and create files for each
for module in "${modules[@]}"
do
    # Create the file with nano
    nano "corona_module/${module}.rs"
    
    # Print a message indicating file creation
    echo "Created ${module}.rs"
done
