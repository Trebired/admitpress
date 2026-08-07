# Printing

## Choosing a queue

**Settings** lists the CUPS queues the machine can see, discovered with
`lpstat`. Pick the one the ticket printer uses. **Refresh** rediscovers them
after you add a printer in your desktop's printer settings.

AdmitPress does not install drivers or create queues. Add the printer in your
desktop first; it then appears here.

## Development output

**Development output, do not use paper** is on by default.

| Setting | Print actions do |
| --- | --- |
| On | Write SVG and PDF files under `development-prints/` in the data directory |
| Off | Send the job to the selected CUPS queue |

Leave it on while you design a layout. Turn it off when you are ready to sell.
The banner at the top of Settings always states which mode you are in.

## Failures

A failed job is recorded, not swallowed. It appears in **Today's activity**
marked as failed, with the reason CUPS gave, and the allocated ticket number so
you know which numbers were consumed.

## Repairing a queue

**Printer debug** in Settings runs a diagnosis: the selected queue, the default
queue, the queues detected, the problems CUPS reports, and recent failures.
**Copy logs** puts the whole report on the clipboard.

Two repair buttons:

| Button | Does |
| --- | --- |
| **Repair printer** | Enables and accepts the queue as your user |
| **Repair as administrator** | The same through `pkexec`, for actions needing root |

The administrator repair runs as one elevated batch, so your desktop asks for
the password once. It needs `pkexec` and a polkit authentication agent. Without
them the application reports that plainly instead of hanging.

## What gets rendered

Each printed ticket is rendered to SVG, rasterised, and submitted to CUPS. The
files stay under `print-jobs/` in the data directory. They are regenerable
output, so backups exclude them, and they can be deleted at any time to reclaim
space.
