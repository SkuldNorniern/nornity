---
title: "Linux how to update kernel"
excerpt: "A comprehensive guide to updating the Linux kernel manually using Debian packages from the kernel PPA."
author: "Eira"
tags: ["linux", "kernel", "debian"]
published_at: "2025-07-26 21:15:54"
draft: false
---

# Updating Kernel is Simpler than you think

First of all you need to go to the site where you can download the kernel file of Linux:

**[Linux Kernel PPA](https://kernel.ubuntu.com/mainline/?C=N;O=D)**

What you can see is the list of available Linux Kernel Version. What you need to do is pick the kernel that you want to use (for me I used v5.12-rc6).

## Choosing the Right Kernel Files

You can opt between low-latency and generic files.

### For Low-Latency Kernel
If you want the low-latency kernel then download these files:

- `linux-headers-x.x.x * low-latency`
- `linux-headers-x.x.x *_x.x.x_all.deb`
- `linux-image-unsigned-x.x.x * low-latency`
- `linux-modules-x.x.x * low-latency`

### For Generic Kernel
For generic download the generic Debian packages for the kernel:

- `linux-headers-x.x.x * generic`
- `linux-headers-x.x.x *_x.x.x_all.deb`
- `linux-image-unsigned-x.x.x * generic`
- `linux-modules-x.x.x * generic`

**Note:** One file will be common in both generic and low-latency is the `linux-headers-x.x.x*_all.deb`. The x.x.x in this case will be 6.15

## Installation Process

Because we have downloaded from the browser:

```bash
$ cd Downloads
```

And while avoiding to install the deb files one by one we will use the command:

```bash
$ sudo dpkg -i *.deb
```

After this is done and your system rebooted:

```bash
$ uname -sr
```

You will be able to see the updated kernel version.

## Summary

Updating the Linux kernel manually might seem daunting at first, but with the right approach and understanding of the different package types (generic vs low-latency), it becomes a straightforward process. The key is to download the appropriate packages for your needs and use the `dpkg` command to install them all at once.

Remember to always backup your system before performing kernel updates, and ensure you have the necessary packages for your specific use case - whether that's the standard generic kernel or the low-latency variant for real-time applications.

