# Notifier-rust
Custom notification tool based on rust

## Clone the repository
```
https://github.com/FASTSHIFT/Notifier-rust.git
```

## Build the project
```
cargo build --release
```

## Run the tool
```
./target/release/Notifier
```

Use the `-c` and `-d` options to customize the configuration and duration of the notifications.
```
./target/release/Notifier -c <config_file_path> -d <duration>
```
to customize parameters.

## Send notifications

Create a configuration file in the `/tmp` directory with the following content:
```
echo -e "[notification]\nsummary = This is the title of the notification\nbody = This is the content of the notification\nsound_file = /usr/share/sounds/freedesktop/stereo/complete.oga" > /tmp/notification-rust.ini
```

