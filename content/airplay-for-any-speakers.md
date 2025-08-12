---
title: "Airplay for Any Speakers"
excerpt: "Turn any speakers into AirPlay targets using a Raspberry Pi, NQPTP, and Shairport Sync—with setup steps, firewall rules, and verification."
author: "Eira"
tags: ["airplay", "iot"]
published_at: "2025-08-12 23:20:14"
draft: false
---

I use Apple Home as my primary home ecosystem and have a HomePod mini, which is fine-but my Kanto YU2s sound better. Since I have a Raspberry Pi 4 already hosting this blog, I'll use it to add AirPlay to any speakers via Shairport Sync (+ NQPTP for proper timing).

> Note: My Raspberry Pi uses wired Ethernet. If you're on Wi-Fi, consider disabling Wi-Fi power saving to avoid timeouts.

## Prerequisites

Update packages and install build requirements:

```bash
sudo apt update
sudo apt upgrade -y   # optional but recommended

# Build tools and dependencies used by NQPTP and Shairport Sync
sudo apt install -y \
  build-essential git autoconf automake libtool pkg-config \
  libpopt-dev libconfig-dev libasound2-dev avahi-daemon libavahi-client-dev \
  libssl-dev libsoxr-dev libplist-dev libsodium-dev \
  libavutil-dev libavcodec-dev libavformat-dev uuid-dev libgcrypt-dev xxd
```

## Install NQPTP

```bash
git clone https://github.com/mikebrady/nqptp.git
cd nqptp
autoreconf -fi
./configure --with-systemd-startup
make
sudo make install

sudo systemctl enable --now nqptp
```

## Install Shairport Sync

```bash
git clone https://github.com/mikebrady/shairport-sync.git
cd shairport-sync
autoreconf -fi

./configure --sysconfdir=/etc \
  --with-alsa \
  --with-soxr \
  --with-metadata \
  --with-avahi \
  --with-ssl=openssl \
  --with-systemd \
  --with-mqtt-clients \
  --with-airplay-2

sudo make
sudo make install

sudo systemctl enable --now shairport-sync
```

## Firewall (UFW)

If you use UFW, open the following ports used by AirPlay and discovery:

```bash
sudo ufw allow 319:320/udp     # PTP (timing)
sudo ufw allow 3689/tcp        # DACP/DMAP
sudo ufw allow 5353/udp        # mDNS (Bonjour)
sudo ufw allow 5000/tcp
sudo ufw allow 7000/tcp
sudo ufw allow 6000:6009/udp
sudo ufw allow 32768:60999/udp
sudo ufw allow 32768:60999/tcp
```

## Verify

```bash
# Services should be active
systemctl status nqptp --no-pager
systemctl status shairport-sync --no-pager

# Recent logs if you need to troubleshoot
journalctl -u shairport-sync -n 50 --no-pager
```

On iOS or macOS, open the AirPlay picker (Control Center -> AirPlay) and look for your Raspberry Pi device. Select it and play audio.

Here is the image for the rpi(mokapot) detected on the AirPlay list

[[image src="/static/assets/image/posts/airplay-for-any-speakers/airplay-speaker-example.jpg" alt="AirPlay speaker detected in list" caption="Raspberry Pi AirPlay device showing in list"]]

