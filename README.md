Rust Echo Daemon
================
This is an implementation of the echo protocol, as described in RFC-862, in the
Rust language.

See docs/rfc862.txt for details.

Installation
------------
The following sections describe install and setup for different systems.

### Linux, using systemd
Copy the following files to the indicated location:

  * `target/debug/echod` to `/usr/sbin/echod`
  * `echod.service` to `/lib/systemd/system/echod.service`

Start the service with:

    $ sudo systemctl start echod

To have the service start automatically at boot:

    $ sudo systemctl enable echod

The current status can be checked as follows:

    $ sudo systemctl status echod
    ● echod.service - TCP and UDP Echo Service
       Loaded: loaded (/lib/systemd/system/echod.service; disabled; vendor preset: enabled)
       Active: active (running) since Fri 2021-09-17 16:15:51 BST; 5min ago
     Main PID: 19535 (echod)
        Tasks: 3 (limit: 4915)
       CGroup: /system.slice/echod.service
               └─19535 /usr/sbin/echod

    Sep 17 16:15:51 <hostname> systemd[1]: Started TCP and UDP Echo Service.

The service can be terminated with the `systemctl stop` command.
