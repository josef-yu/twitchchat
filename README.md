# Twitchchat

**A Twitch Chat reader and scraper**

---

## Commands

 - **Channel**. Use `-c <CHANNEL_NAME>` or `--channel <CHANNEL_NAME>` for reading a twitch channel chat
    even if the stream is offline.
   
 - **Token**. Use `-t <TOKEN>` or `--token <TOKEN>` for setting up the authorization
    token. This can be obtained via https://www.twitchapps.com/tmi
   
 - **Client ID**. Use `-i <CLIENT_ID>` or `--clientid <CLIENT_ID>` for setting up the client ID.
    This can be obtained via https://www.dev.twitch.tv/console/apps.
   
 - **Logging**. Use `-l <FILENAME>` or `--log <FILENAME>` to save the messages of the
    channel's chat being set to read. If `FILENAME` is not provided, it defaults to
    a filename in the format of `<CHANNEL>-<CURRENTDATE>.txt` where current date
    is `YEAR-MONTH-DATE`.
   
## Todo

 - Implement a unique color for each username
   
 - Implement VOD chat scraping

 - Implement silent mode

 - Implement hotkeys

 - Better error handling
