# Weebmachine

A simple bot to send new episode posts from /r/anime to a discord channel, via webhooks

## Release

No releases or upload to crates.io for now

## How to run

Simply run the binary with WEBHOOK_URL as env var e.g.:

    WEBHOOK_URL=https://discord.com/api/webhooks/123123/asdfasdf ./weebmachine

## What it does

1. Check /r/anime for the hundred newest posts
2. Check if a post has flair "Episode"
3. Get an image from MAL if possible
4. Post to discord