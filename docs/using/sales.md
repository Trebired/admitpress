# Sales and Export

The **Sales** tab on the print page holds every recorded day.

![The sales overview](../screenshots/sales.png)

## Reading the overview

Days are grouped by month. Each month row shows the number of days with sales,
the ticket count and the money total. The current month is expanded, earlier
months are collapsed. Click a month row to fold it.

Each day row shows its ticket count and total. The info button on any row opens
a detail dialog breaking the figures down per price.

**Show empty** includes days with no sales, so a month reads as a calendar.
Turn it off to see only days that recorded something.

## Selecting days

Tick a day to select it. Ticking a month selects every day in it; the month
checkbox is partly filled when only some days are selected.

**All** selects every day that has recorded sales. It does not select empty
calendar days, so a month with three trading days selects three days, not
thirty one.

## Export

With days selected, press **Export** and choose a format:

| Format | Use |
| --- | --- |
| `.xlsx` | Excel and most spreadsheets |
| `.ods` | LibreOffice Calc |

The export contains a row per day and price, with quantities, unit prices and
totals. Files are written wherever you choose in the save dialog; the
application also keeps a copy under `exports/` in the data directory.

## Deleting a day

The day detail dialog offers deletion. It removes the sales record for that day.
It cannot be undone, and it does not reset counters for other days.
