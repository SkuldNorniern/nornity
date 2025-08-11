---
title: "Ubuntu 24.04 LTS (Noble Numbat) Source List Configuration"
excerpt: "Complete guide to configure package sources for Ubuntu 24.04 Noble Numbat"
author: "Eira"
tags: ["linux"]
published_at: "2025-08-11 14:16"
draft: false
---

## Ubuntu 24.04 LTS (Noble Numbat) Source List Configuration

Ubuntu 24.04 LTS (Noble Numbat) is using a new format for package sources. Unlike previous versions, the sources are now stored in `/etc/apt/sources.list.d/ubuntu.sources` instead of the traditional `/etc/apt/sources.list` file.


### Complete Source Configuration

edit the file `/etc/apt/sources.list.d/ubuntu.sources` with the following content:

```
Types: deb
URIs: http://kr.archive.ubuntu.com/ubuntu
Suites: noble noble-updates noble-backports
Components: main restricted universe multiverse
Signed-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg

Types: deb
URIs: http://security.ubuntu.com/ubuntu
Suites: noble-security
Components: main restricted universe multiverse
Signed-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg
```