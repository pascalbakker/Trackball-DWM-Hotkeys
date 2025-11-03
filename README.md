# DWM Hotkeys for ELECOM Deft Pro Trackball

Adds some movement gestures for ELECOM Deft Pro Trackball

## New Hotkeys 
BTN_TASK + Right movement => moves to right screen
BTN_TASK + Left movement => moves to left screen
BTN_TASK + Up movement => Moves window to next tag
BTN_TASK + Down movement => Moves window to previous tag

Adjust horizontal and vertical thresholds for how fast it should activate

## Prerequisites

For this to work you must

1. Have the DWM IPC patch [Link](https://dwm.suckless.org/patches/ipc/)
2. Have the next tag patch [Link](https://dwm.suckless.org/patches/nextprev/)
3. Include the commands in your IPC_COMMANDS
```
static IPCCommand ipccommands[] = {
  IPCCOMMAND(  view,                1,      {ARG_TYPE_UINT}   ),
  IPCCOMMAND(  toggleview,          1,      {ARG_TYPE_UINT}   ),
  IPCCOMMAND(  tag,                 1,      {ARG_TYPE_UINT}   ),
  IPCCOMMAND(  toggletag,           1,      {ARG_TYPE_UINT}   ),
  IPCCOMMAND(  tagmon,              1,      {ARG_TYPE_UINT}   ),
  IPCCOMMAND(  focusmon,            1,      {ARG_TYPE_SINT}   ),
  IPCCOMMAND(  focusstack,          1,      {ARG_TYPE_SINT}   ),
  IPCCOMMAND(  zoom,                1,      {ARG_TYPE_NONE}   ),
  IPCCOMMAND(  incnmaster,          1,      {ARG_TYPE_SINT}   ),
  IPCCOMMAND(  killclient,          1,      {ARG_TYPE_SINT}   ),
  IPCCOMMAND(  togglefloating,      1,      {ARG_TYPE_NONE}   ),
  IPCCOMMAND(  setmfact,            1,      {ARG_TYPE_FLOAT}  ),
  IPCCOMMAND(  setlayoutsafe,       1,      {ARG_TYPE_PTR}    ),
  IPCCOMMAND(  quit,                1,      {ARG_TYPE_NONE}   ),
  // Add these last 4 
  IPCCOMMAND(  tagtonext,           1,      {ARG_TYPE_NONE}   ),
  IPCCOMMAND(  tagtoprev,           1,      {ARG_TYPE_NONE}   ),
  IPCCOMMAND(  viewnext,            1,      {ARG_TYPE_NONE}   ),
  IPCCOMMAND(  viewprev,            1,      {ARG_TYPE_NONE}   ),
  };
```

# Installation

```
sh install_application.sh
```

# Execution
```
hotkeys&
```
