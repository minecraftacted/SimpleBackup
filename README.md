# SimpleBackup

SimpleBackup, as the name suggests, is a simple solution to backup.
Compatible with Windows, Mac, and Linux.
## Installation
You must do only 3 commands:
```
git clone https://github.com/minecraftacted/SimpleBackup.git
cd SimpleBackup
cargo build --release
```

You need git and rust(programming language, not a game) to build.

## Configuration
Open config.yml in SimpleBackup/target/release/.
```yml
#These directories or files will be copied to "directory_to_save_directory" path as backup data.
# NOTE: Relative paths are interpreted from the current directory when the program is run.
#       We recommend specifying an absolute path.
directories_to_backup:
#- ./something_to_backup/
#- ./what_to_backup/
#- ./some_directory/

#Backups will be saved to the directories set here.
#If paths doesn't exist, the program will create new directories.
# NOTE: Relative paths are interpreted from the current directory when the program is run.
#       We recommend specifying an absolute path.
directories_to_save_backup: 
#- ./backups/
#- ./backups2/

#When the program is run,
#folders older than the number set here will be deleted.
storage_period_days: 7
```
The inside of the file should look like this.

This is the configuration file for this program, from which you can change all settings.

### About each item
**directories_to_backup** is paths of file or directory to save as backup.

The program will back up the files or directories specified here.

The path can be specified as a relative path. In that case, relative paths are interpreted relative to the current directory when running the program.

**directories_to_save_backup** is destination of backups.

Inside these directories, a directory named after the date will be created, and the directories_to_backup directories will be backed up in that directory.

It can be specified with a relative path. Relative paths are handled in the same way as before.

**storage_period_days** is storage period.

Backups older than this number of days will be deleted when the program runs.

## Let's backup!
Move the executable file and config.yml to your favorite location and run it.

It is a success if it looks like the following! congratulations!

```
backups
└ 2024-05-05-18-12-dd4a2ee6-3d03-4879-b4a7-e4706624ffb1┐
 └ Data more important than life
```
Idea:

It would be a good idea to use a regular execution tool such as cron to ensure backups.


