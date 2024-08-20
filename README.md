# frisbee
Firefox automation to send URLs to a local server.

> [!NOTE]  
> Development is early and wild. You probably don't want to use this yet.

## Overview

There are things I'd like to do with URLs I visit in Firefox. I would like
to feed some URLs to a background local process for processing. I don't know
of a good, simple way to do this currently. There are several automation
extensions for firefox, but they are complex and more than I need.

All I want is a button in Firefox that will send the URL of the current tab
to some local server:

- running on `http://localhost:5800`.
- via `POST`
- with a JSON body like this:
```
{
    "url": "https://www.example.com"
}
```

The local server can then do "something" with the URL.

It is important to me that the local server can be switched out for different
servers that do different things. I don't want to have to change the extension
to do this. So for example, you could use this Firefox extension and use your
own server to do whatever you want with the received URL.

## Installation

### Install the Firefox extension

The Firefox plugin is installed like this:

1. Open Firefox and go to about:debugging.
2. Click on "This Firefox" (in newer versions).
3. Click "Load Temporary Add-on..."
4. Select your manifest.json file.

### Start the localhost server (port 5800)

If you have Rust installed, you can build it like this:

```bash
$ cd frisbee/frisbee-server
$ cargo run
```

Otherwise you'll need the binary for your system. I will
eventually build them and provide binaries in the Releases
section.
