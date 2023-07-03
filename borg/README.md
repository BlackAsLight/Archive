# Borg
This is a small script to automate the backing up of files to a remote repo through SSH with [BorgBackup](https://www.borgbackup.org/). Refer to their docs on how to install, use, and setup Borg.

## SSH Config
Your ssh config should look something like this
```
Host Borg
    Hostname <Remote URL>
    User <Username>
    Port 22
    IdentityFile ~/.ssh/<Key>
```

## Compile Time Environment Variables
There are several compile time environment variables that need to be provided when compiling this project.
- NAME
  - The name you chose for your BorgBackup Repo
- PASS
  - The password required to decrypt it
- IN_DIR
  - The directory you'd like to backup
- OUT_DIR
  - The directory, on the remote device, that your BorgBackup Repo is located (Excluding the Repo name)
- BORG_LOC
  - The location the `borg` command is installed on this system. Use `which borg` to find out

## Run Time Environment Variables
There are two run time environment variables that allow you to configure some of the dynamic behaviour it does.
- EXCLUDE (Optional)
  - A text file separated by new lines with each line containing a pattern, by borg's standards, that you'd like to exclude from the backups.
- LOGS (Optional)
  - A directory to save the logs that it prints out.
