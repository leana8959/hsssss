This is a bodged-together `telnet` server that, when you connect to it, shows you a little animation of a snake flicking its tongue.

# Features
Center the animation when the client-side's window dimension changes.

*Note*: It doesn't actually support any telnet protocal exchange beside window dimension negotiation (`IAC DO NAWS` sequence). I simply made it reject any protocol request to keep the client responded while having the connection going. The `IAC DO TIMING MARK` sequence, which happens to be triggered by ^C, is overloaded with the ability to close the connection.

# Usage
```sh
hsssss --path [your_frame_file] --addr [address_to_bind_to]
```
*Note*: You may need to use `sudo` to run this program, as `telnet` binds to port 23.

Simply do
```sh
telnet [machine_that_runs_hsssss]
```
to connect to this `telnet` server. Use `^C` to quit.

# Frame file
For modularity, I decided that a frame file is a text file that contains some ASCII artwork, where each frame is separated by a `>\n` (a `>` and then a new line character).
Any other file of the same format should work just fine.

# Credits & stuff used
- [GrizzlT](https://github.com/GrizzlT) for the help with `tokio`
- [Hykilpikonna](https://github.com/hykilpikonna) for the idea
- [telnetlib](https://github.com/python/cpython/blob/main/Lib/telnetlib.py)
- [RFC 854](https://www.rfc-editor.org/rfc/rfc854.html)
