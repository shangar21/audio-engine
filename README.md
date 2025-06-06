# audio-engine

A headless, networked audio server written in Rust that uses GStreamer to stream and receive audio across devices. Designed for low-latency audio control in distributed home systems like Raspberry Pi clusters, laptops, or PCs.

---

## Dependencies
- Rust / Cargo
- GStreamer

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
