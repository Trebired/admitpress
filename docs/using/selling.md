# Selling

The print page is where sales happen. Open it with **Start printing**.

![The print page](../screenshots/print.png)

## Making a sale

Set a quantity against one or more prices with the steppers. The header shows
how many tickets are selected. **Reset selection**, the red circular arrow,
clears every quantity at once.

Then choose one of three actions:

| Action | Prints | Advances counters | Recorded |
| --- | --- | --- | --- |
| **Print tickets** | One ticket per item | Yes | Yes |
| **Print group ticket** | A single ticket listing every item | Yes | Yes |
| **Record only** | Nothing | Yes | Yes |

**Record only** exists for sales that need no paper, such as a customer who
declines a ticket, or a correction made after the fact. It counts as a real
sale everywhere in the application.

**Print group ticket** needs the group layout to have fields on it, and is
intended for two or more items.

## Naming a sale

**Sale name** is optional and applies to any sale, including a single ticket.
The name appears in the day's activity and in the sales record, and on the
group ticket itself when the layout has a group name field.

Use it for anything you want to find later: a school group, a coach party, a
reservation reference.

## Today's activity

Below the actions, **Today's activity** lists everything that happened today:
tickets printed, sales recorded without printing, failed print jobs, and
counter corrections.

Search filters by text. The dropdown filters by kind, so you can show only
corrections, or only failures. Selecting a print entry opens its detail, with
the items, the totals and the allocated numbers.

## Reverting a sale

The overflow menu offers **Revert last sale**. It rolls every counter in the
last sale back by the quantity sold, clamped so a counter never falls below its
starting number, and records the reversal with the reason `Revert last sale`.

It reverses the numbering. It does not un-print paper already produced.

## The date

The date button at the top selects the day you are selling for. It normally
shows today. Selecting an earlier day lets you record a sale you missed, using
that day's counters.
