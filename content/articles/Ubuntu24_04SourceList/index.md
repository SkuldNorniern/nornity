---
title: "Ubuntu 24.04 LTS (Noble Numbat) Source list"
slug: ubuntu24_04sourceslist
image: "/images/tech-future.jpg"
language: "English"
tags: ["Linux"]
date: "2024-09-27"
author: "Skuld Norniern"
---

Starting from 24.04, `/etc/apt/sources.list.d/ubuntu.sources` is the new configuration file for the sources.
> it's a bit odd :(

Here is the full list of the sources for Ubuntu 24.04 LTS (Noble Numbat)

```
Types: deb deb-src
URIs: http://us.archive.ubuntu.com/ubuntu/
Suites: noble noble-updates noble-backports noble-proposed
Components: main restricted universe multiverse
Signed-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg
Types: deb deb-src 
URIs: http://security.ubuntu.com/ubuntu/
Suites: noble-security
Components: main restricted universe multiverse
Signed-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg
```