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
sudo apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 gstreamer1.0-qt5 gstreamer1.0-pulseaudio gstreamer1.0-tools
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
