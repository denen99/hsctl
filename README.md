# Overview
hsctl is a simple CLI for interfacing with the HomeSeer system.  Sometimes you do not want to navigate the UI or sometimes you want to script certain things to happen.  Or you just like viewing your system from the command line.  For me, I just wanted to learn Rust :) 

#Options 
hsctl has global options and subcommand options.  Please remember to use them in the right order.  It is 

```hsctl [GLOBAL OPTIONS] [SUBCOMMAND] [SUBCOMMAND_OPTIONS]```

For more detail on the options of any subcommand you can always type 

```hsctl help SUBCOMMAND``` where subcommand is currently login,control,status.

####Global Options

#####-h / --hostname  

Allows you to set the hostname you want to connect to.  This defaults to `https://connected11.homeseer.com`, but you can always specifify your internal homeseer device instead 

##### -o / --output 

Allows you to specify how you want the output.  The current options are `table` and `json` and it defaults to `table`.  This is so you can have a pretty print output or something you can script against 

#Authentication 
If you have not logged in yet, you need to first execute the `login` subcommand which takes 2 parameters `username` and `password` in that order.  Once you successfully login, hsctl caches the auth token locally in ~/.hsctl.  On startup, it quickly validates that token against the `hsversion` endpoint.

To login use the command 

```hsctl login myusername mypassword```

On successful login you should see something like the following 

```
Warning, no cached credentials found, make sure you login!
Login successful
Cached token directory does not exist, creating /Users/adenenberg/.hsctl/
Login token cached successfully..
```

#Status
The most commom use case for querying HomeSeer is probably the status subcommand.  

Assuming you have already logged in with the login subcommand, you can simply do 

```hsctl status``` 

to get a tabled result of all your devices.  Additionally, hsctl gives you the ability to filter your results by location1, location2 or the ref, using the subcommand options `--loc`, `--loc2`, or `--ref`

#Control
hsctl also gives you the ability to control your devices.  This one is a little bit trickier since different devices take different values.  HomeSeer allows you to control either by label or by value.  Only one of these two can be used and the CLI will not let you use both options.  When you run ```hsctl status``` you can get a better sense of the device values and labels that your system has.  The use cases for label vs value, could be using a label of `On` for a switch or a Dimmer value of `10` to Dim the light.

For example, to turn off a light with ref id 55, something like the following would work 

```hsctl -h https://myhome.example.com control --ref 55 --value 0``` 

Should return a result like the following 

```
Cached token validated successfully.
Success setting status by value
```