# **blrs**
A simple CLI interface for backlight control on linux-based systems. 
# Subcommands
## get <value>
Returns the current backlight value
## set <value>
Sets the current backlight value
## inc <value>
Increments the backlight by value  
*Will fail if result is larger than 100*
## dec <value>
Decrements the backlight by value
*Will fail if result is less than 0*
## smooth <value> <time (ms)>
Smoothly transitions to value in given time
# Running
## Download Sources
`git clone https://github.com/wikki1/blrs.git`  
`cd blrs`
## Build
Check the output of the following command to see which section to follow.  
`ls /sys/class/backlight/`
## acpi_video0
`cargo build --release`
## intel_backlight
`cargo build --release --features intel`
## Run
`./target/release/blrs help`

