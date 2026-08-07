# Currency

## Main currency

**Settings** holds the main currency. It defaults to the currency of your
operating system locale, falling back to the US dollar when the locale does not
name one.

Every price is stored in the main currency. The prices dialog states which
currency you are typing in.

Changing the main currency does not convert prices already entered. It changes
the currency those numbers are interpreted as, so change it only when setting
up, or reprice afterwards.

## Selling in another currency

The print page has a currency picker above the prices. It is searchable, and it
lists every ISO currency.

Pick another currency and the totals shown convert to it. The ticket prints the
amount in that currency, with that currency's symbol. The sale is recorded with
the currency it was sold in, so the sales record and the export stay honest
about what the customer paid.

Selling in the main currency needs no action; the picker already shows it.

## Exchange rates

Rates are fetched from `open.er-api.com` and cached for six hours, on a rolling
window from the moment each pair was fetched. The cache is stored in the
database, so restarting does not refetch.

Conversion needs a rate. If a pair has never been fetched and the machine is
offline, the application cannot convert and says so rather than inventing a
number. Sell in the main currency until the connection returns.

Rates are for display and for recording what was charged. They are not a
financial data source.
