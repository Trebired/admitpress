# Designing Tickets

A ticket is a paper size plus a field layout. Open one with **Edit** from the
ticket list.

![The ticket editor](../screenshots/editor.png)

## The two layouts

Every ticket carries two independent layouts, switched with the tabs above the
canvas:

| Layout | Printed when |
| --- | --- |
| **Classic** | One ticket per item sold |
| **Group ticket** | A sale of two or more items is printed as a single ticket |

Fields placed on one layout do not appear on the other. A ticket you never sell
in groups needs only the classic layout.

## Fields

Add a field from the palette in the lower right. Drag it on the canvas to move
it, drag its corner to resize it.

| Field | Prints |
| --- | --- |
| **Static** | Fixed text you type, such as a venue name or a website |
| **Image** | A picture you import, copied into the data directory |
| **Price** | The price of the item sold, with its currency symbol |
| **Date** | The date of the sale, in the format you pick |
| **Number** | The ticket number for that sale |
| **Group name** | The name given to the sale, group layout only |
| **Tickets** | The list of items in the sale, group layout only |

A dynamic field can appear once per layout. The palette disables a field that
is already placed.

## Selected field

Selecting a field opens its properties on the right:

- **X, Y, W, H** in millimetres, with **Center H**, **Center V** and **Both**
- **Font**, **Font size**, and horizontal and vertical alignment
- **Prefix** and **Suffix**, wrapping the value with fixed text
- **Date format** for date fields: date, date and time, ISO, or time alone
- **Numbering** for number fields, covering padding and the starting number

**Snap to centered grid** aligns fields to a grid measured from the centre of
the paper. Turn it off for free positioning.

## Copy and paste

Select one or more fields and press Ctrl+C, then Ctrl+V. Pasting offsets the
copy by 2 mm and keeps it inside the paper.

Pasting works across tickets. Open another ticket and paste. An image field
pasted into a different ticket has its picture copied into that ticket's own
artwork folder, so the two tickets never share a file.

A field is not pasted when the target layout cannot hold it, for example a
group name field pasted onto a classic layout, or a second date field.

## Preview

**Preview** renders the ticket with sample data so you can see the result
without printing. Sample prices use a random currency, so the layout is
checked against symbols wider than your own.

**Save** writes the layout. Closing the editor with unsaved changes asks first.
