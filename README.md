# audio-engine

A headless, networked audio server written in Rust that uses GStreamer to stream and receive audio across devices. Designed for low-latency audio control in distributed home systems like Raspberry Pi clusters, laptops, or PCs.

---

## Dependencies
- Rust / Cargo
- GStreamer

---

## Set up

On linux, first install GStreamer

```
sudo apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 gstreamer1.0-qt5 gstreamer1.0-pulseaudio gstreamer1.0-tools gstreamer1.0-plugins-base-apps
```

and if cargo is installed, just run

```
cargo run
```

---

## Current Capabilities

- One-way audio streaming from one device to another
    - Plays all output of one device on another 
- Simple Axum-based HTTP server to start sender or receiver on request

---

## TO-DO

- [ ] dicover devices running engine
- [ ] play audio locally
- [ ] play audio remote (files, streams, etc.)
- [ ] mic broadcast / sending
- [ ] spotify

## How to use

There are 4 end points:

- `send` which takes as input the source (pulsesrc for example), device (can be found with `pactl list sources short`) a host to send audio to and a port. Returns a success message and `session_id` which is needed to terminate the pipe if needed.
    - request body example: `{"source":"pulsesrc","device":"alsa_output.pci-0000_00_1f.3.analog-stereo.monitor","host":"192.168.1.69","port":"5004"}`
    - return example: `{"host":"192.168.1.69","port":"5004","status":"success","session_id":"e6ad85f3-391f-4d60-b883-7172324deab5"}`
- `receive` which takes as input just the port (the same as the one used on a send request). Returns a success message and `session_id` which is needed to terminate the pipe if needed.
    - request body example: `{"port":"5004"}`
    - return example: `{"status":"success","session_id":"149bcfbe-cdf0-41b8-afc6-86704d04d1da"}`
- `stop_send` which takes as input the session id, and then terminates the send session. Returns a success message if ok.
    - request body example: `{"session_id":"e6ad85f3-391f-4d60-b883-7172324deab5"}`
    - return example: `{"status":"success","session_id":null"}`
- `stop_receive` which takes as input the session id, and then terminates the send session. Returns a success message if ok.
    - request body example: `{"session_id":"149bcfbe-cdf0-41b8-afc6-86704d04d1da"}`
    - return example: `{"status":"success","session_id":null"}`

If running an intance, can use `curl` to send requests, for example:

```bash
curl -H 'Content-Type: application/json' -d '{"port":"5004"}' -X POST 127.0.0.1:3000/receive
curl -H 'Content-Type: application/json' -d '{"source":"pulsesrc","device":"alsa_output.pci-0000_00_1f.3.analog-stereo.monitor","host":"192.168.1.69","port":"5004"}' -X POST 127.0.0.1:3000/send
```
