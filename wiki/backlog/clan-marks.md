---
kind: backlog
id: clan-marks
name: Clan marks
status: not-in-game-yet
summary: Each clan shows a mark, either a background and symbol from the game's sets or a custom image the master uploads once a week
npcs:
  - "[[npcs/1115-clan-owner-burtland|Clan Owner Burtland]]"
source:
  data: 3DDATA/CONTROL/RES/CLANBACK.TSI (17 backgrounds) and CLANCENTER.TSI (20 symbols) with their DDS files; DlgOrganizeClan.xml; DlgClan.xml; LIST_STRING.STL strings 620-634
  reference: iROSE 129_129en client data and TRose.exe strings (CClanMarkTransfer, "clanmark\\", "%s%d_%d.bmp", ".\\Mark.bmp"); ClanMark type in crates/rose-game-common/src/components/clan.rs
---
# Clan marks

A clan's mark is shown next to its name over every member's head and in the
[[backlog/clans|clan]] window.

## How it works in iROSE 129

There are two kinds of mark (the vendored `ClanMark` type has the same two):

### Premade mark

Picked when the clan is founded, in the two grids of the founding window:

| Part | Choices | Sprites |
| --- | --- | --- |
| Background | 17 | CLANBACK.TSI on CLANBACK.DDS |
| Symbol | 20 | CLANCENTER.TSI on CLANCENTER.DDS |

The symbol is drawn on top of the background. The server stores the two numbers
(background and foreground, both 1 or higher).

### Custom mark

The master can replace the premade mark with an image, from the Info tab of the clan window
("Register Clan Mark", with a Preview button first):

- The image is the file `Mark.bmp` in the game folder (string 624 names it).
- The client checks the size and colours and refuses a wrong file: "Not suitable image
  size." (625), "Cannot find the file." (626), "The registered image is in wrong format"
  (627), "Please refer to the color restriction." (629).
- It is gzip-compressed and sent to the server. The server keeps it and gives the clan a
  CRC16 of the image; that CRC is what other players receive.
- Other clients download the image when they see an unknown CRC and cache it in the
  `clanmark\` folder as `<server>_<clan id>.bmp`.
- A clan can register a mark **once per week** (632); the window shows the registering
  date (634). Success: "Congratulations! You have successfully registered Clan Mark." (630).

> Open question: the exact image size and colour limits. String 623 sends players to the
> ROSE Online web page for them; the check is in the client code, not in the data.
> Commonly remembered as a small 20 × 20 bitmap, but that is not verified.

## What our game does today

No clans, so no marks.

## Building it

- **Server**: store the premade pair or a custom image (with CRC and upload date) on the
  clan; enforce once a week; serve the image to clients that ask for it.
- **Client**: the two grids in the founding window from CLANBACK/CLANCENTER; draw the mark
  next to clan names; custom image upload with the size and colour checks, and a cache.
- **Data**: none to change.
