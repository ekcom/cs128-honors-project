# Rust Apache Clone

*[Elijah Mock](https://github.com/ekcom) (emock3), [Colin Richardson](https://github.com/crich46) (colinhr3), [Maanas Belambe](https://github.com/maanasbelambe) (belambe2), and [Shreya Rao](https://github.com/Sh-r-eya) (shreya29)*

## Project introduction
A multithreaded HTTP server which serves static files with proper headers and MIME types, similar to a basic Apache server.
This project also has a GUI which can be used to start the server and configure settings.

## Technical overview
This project works as an Apache clone, which will support sending different files as an HTTP server. It will scan the local directory for the requested path from the HTTP GET request and serve that file if found or serve the 404 page if not found. Additionally, this project has a GUI interface from which the server can be started/stopped and settings can be configured. Settings include setting the port to host the server on and which directory to serve static files from. Additionally, this project will be multithreaded, meaning it can handle multiple requests simultaneously from spawned worker threads.

### Task list

#### Checkpoint 1

- [ ] Research crates to use
    - potentially [conrod_core](https://crates.io/crates/conrod-core) (GUI), [http](https://docs.rs/http/latest/http/) (HTTP requests)
- [x] Open a network connection to client
- [x] Parse client's request to determine which resource is requested
- [ ] Send proper headers to client
- [x] Read files and send them to client
- [x] Send a default 404 page if the resource could not be located
- [ ] Create basic GUI for controlling the server
- [ ] Template out a settings menu in the GUI

#### Checkpoint 2

- [ ] Create multiple threads to handle multiple requests at the same time
    - [ ] Have a setting (configurable through the GUI) which determines the number of threads to spawn
- [ ] Create a configurable option to choose the directory to serve files from
- [ ] Create a configurable option to set 404 (not found) page
- [ ] Create a configurable option to change port to run on
- [ ] Persist settings upon restarting the program (store in a file)
- [ ] Write requests to a log file
- [ ] Improve GUI appearance

#### Final submission

- [ ] Wrap up previous checklists
- [ ] Attempt to [cache data](https://httpd.apache.org/docs/current/caching.html) and not resend pages to users
- [ ] Attempt TLS encryption through HTTPS
- [ ] Squash bugs
- [ ] Create <10 min video of project demo, discussion, and brief code overview

## Possible challenges

Some challenges from this project include learning how to interface with a network connection using Rust, splitting up the server across multiple threads without creating any issues with ownership of data and file locks, and learning how to create a GUI with Rust.

## References

### Inspiration

Apache: https://www.apache.org/

XAMPP: https://www.apachefriends.org/

### Guidance

[list any tutorials used here]
