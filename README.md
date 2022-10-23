# RUSTADB #

It has implementaion of setting up and running adb related tools in one command. Checkout the added rustadb binary.


## Availble Feature ##
```
showinfo       : to show the all commands and their used.

start          : to start adb server. This is equivalent to adb start-server.
stop           : to stop adb server. This is equivalent to adb kill-server.
ls             : to show the list of connected devices. This is equivalent to adb devices.
ip             : to show ip of connected device.
connect        : to connect the device with wifi.
        e.g. connect IP:PORT.
        You can get the IP of phone in Settings > Wifi Settings > Advance > IP Address.
        Or Search IP address in Setting. Or Use ip command.
        Make sure phone is connect through USB.
logcat         : to record logcat in file, after this command add file name where logcat should be stored.
        eg. logcat demo.

download       : to download android command line tools and platform tools in home dir.
setpath        : to set the downloaded tools enviromental path.

capture        : to capture the screen of connected devices.
        eg. capture filename.
record         : to record the screen of connected devices.
        eg. record filename.
dump           : to get dump of system or application.
        eg. dump packagename.
listpackage    : to print all installed applications packagename.
pull           : to pull the installed application from phone to computer.
        eg. pull packagename.
install        : to install apk in connected phone.
        eg. install location/of/your/apk.

exit: to close radb app.
```
#### Above option is shown when app starts ####

### Requirements ###
* Lunix OS.
* Java should installed for adb.
* Git to clone jar files.