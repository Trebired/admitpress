const CURRENCY_SUFFIXES: Readonly<Record<string, string>> = {
  CZK: "Kč",
  EUR: "€",
};

function formatAmount(amountMinor: number): string {
  const negative = amountMinor < 0;
  const absolute = Math.abs(amountMinor);
  const major = Math.floor(absolute / 100);
  const minor = absolute % 100;
  const sign = negative ? "-" : "";
  if (minor === 0) return `${sign}${major}`;
  return `${sign}${major},${minor.toString().padStart(2, "0")}`;
}

function formatMoney(amountMinor: number, currency: string): string {
  const amount = formatAmount(amountMinor);
  if (currency === "") return amount;
  if (currency === "USD") return `$${amount}`;
  const suffix = CURRENCY_SUFFIXES[currency];
  return suffix ? `${amount} ${suffix}` : `${amount} ${currency}`;
}

export { formatMoney };
