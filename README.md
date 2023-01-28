# winscreenshot
Periodic screen capture in Rust

## Description
Captures a screenshot of the current screen and stores it as jpg-file in the supplied directory.
By default the file is named by the current date and time like so 2024-06-20T10.06.37.jpg.

## Examples

```
winscreenshot
```
Creates a screenshot file in the format 2024-06-20T10.06.37.jpg in the current directory.
```
winscreenshot C:\MyStuff\Screenshots
```
Creates a screenshot file C:\MyStuff\Screenshots\2024-06-20T10.06.37.jpg
```
winscreenshot -Path /Screenshots -Filename screenshot10
```
Creates a screenshot file /Screenshots/screenshot10.jpg
```
winscreenshot -Path /Screenshots -Interval 600 -Times 10
```
Creates screenshot every 10 minutes in /Screenshots as 2024-06-20T10.06.37.jpg,
2024-06-20T20.06.37.jpg, 024-06-30T10.06.37.jpg etc.

```
winscreenshot -Path /Screenshots -Interval 60 -Forever
```
Creates screenshot, one every minute in /Screenshots forever.
