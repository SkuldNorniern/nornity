---
title: "Fix xcrun unable to find metal"
excerpt: ""
author: ""
tags: ["macos","xcode"]
published_at: "2025-10-30 11:39:54"
draft: false
---
# Fix xcrun Unable to find metal

In this post, we'll talk about the occasional issue with Xcode Metal
> xcrun: error: unable to find utility "metal", not a developer tool or in PATH

I still haven't found out what is causing the problem yet (will update the post if I find it)
But mostly, it seems like Apple's side causes the issue.


Anyway, the most common way to fix this originates from  [gfx-rs/gfs #2309](https://github.com/gfx-rs/gfx/issues/2309#issuecomment-413695219)
- case 1: the Command Line Tools for Xcode are not installed
> xcode-select --install

- case 2: Xcode's path is causing the issue
> xcode-select --switch /Applications/Xcode.app/Contents/Developer


But for me, those two didn't solve my issue
- Case 3: didn't agree to the license
> sudo xcodebuild -license

- case 4: set a system default for the active developer directory
> sudo xcode-select -r

And for me, 4 solved the issue, but I found out that the metal toolchain was installed
- case 5: the metal toolchain is not installed
> xcodebuild -downloadComponent MetalToolchain

and for testing the xcrun metal
> xcrun -sdk macosx metal

With these 5, I'm quite sure that the issue with Xcrun Metal would be mostly solved
