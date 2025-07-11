# scrabber
Periodic screen capture in Rust

## Description
Captures a screenshot of the current screen and stores it as jpg-file in the supplied directory.
By default the file is named by the current date and time like so 2024-06-20T10.06.37.jpg.

## Examples

```
scrabber
```
Creates a screenshot file in the format 2024-06-20T10.06.37.jpg in the current directory.
```
scrabber C:\MyStuff\Screenshots
```
Creates a screenshot file C:\MyStuff\Screenshots\2024-06-20T10.06.37.jpg
```
scrabber --path /Screenshots -Filename screenshot10
```
Creates a screenshot file /Screenshots/screenshot10.jpg
```
scrabber --path /Screenshots --interval 600 --count 10
```
Creates screenshot every 10 minutes in /Screenshots as 2024-06-20T10.06.37.jpg,
2024-06-20T20.06.37.jpg, 024-06-30T10.06.37.jpg etc.

```
scrabber --path /Screenshots --interval 60
```
Creates screenshot, one every minute in folder /Screenshots.

```
scrabber --interval 10 --forever
```
Creates screenshot, one every 10 seconds forever until stopped with Ctrl-C.

```
scrabber --interval 5 --skip-duplicates --forever
```
Creates screenshot every 5 seconds forever, but skips saving any that are identical to the previous one. 
