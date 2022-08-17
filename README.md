# **blrs**
A simple CLI interface for backlight control on linux-based systems. 
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

