# ToDo

- [x] Output to serial port so we can have more than a screen of data
- [ ] VGA WriteLine should handle bare LF's and make them optionally CRLF
- [ ] Stop hardcoding magic constants including the SATA drive
- [ ] Figure out why we cannot sometimes debug main.rs
  - It's probably too big and getting loaded incorrectly
- [ ] Serial shouldn't wait forever to init which causes hang
- [ ] Clean up hack city memory managment code
- [ ] Read from disk so we can hopefully have faster iteration loop and local storage
  - Almost there...
- [ ] Change resolution
- [ ] Get rid of all BUGBUGs
