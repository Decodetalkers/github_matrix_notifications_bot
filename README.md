# Cris Spam bot

Spam bot for matrix users.

## Usege

```bash
cris_spam_bot -o /path/to/config.toml
```

toml example is under misc

### step

* create a new github account
* set Notification to github not email
* watch the repo which you want to be send notify
* get a token to receive notifications
* create a new matrix account
* write token, account, passward, botname, and homeserver to toml
* run it, and click `it is me` on matrix client applications
* try open a issue under the repo is watched with another account

## From docker

```
sudo docker run -v /host/settings/dir:/settings/ ${imageid} config /settings/setting.toml
```
