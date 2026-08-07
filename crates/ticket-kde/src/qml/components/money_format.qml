pragma Singleton

import QtQml

QtObject {
  property var backend: null

  function groupThousands(digits) {
    return digits.replace(/\B(?=(\d{3})+(?!\d))/g, " ")
  }

  function fallbackFormat(amountMinor, currency) {
    const sign = amountMinor < 0 ? "-" : ""
    const absolute = Math.abs(Math.round(Number(amountMinor || 0)))
    const major = groupThousands(String(Math.floor(absolute / 100)))
    const minor = absolute % 100
    const amount = minor === 0 ? sign + major : sign + major + "," + String(minor).padStart(2, "0")
    return currency ? amount + " " + currency : amount
  }

  function format(amountMinor, currency) {
    if (backend && backend.formatMoney) {
      return backend.formatMoney(Number(amountMinor || 0), currency || "")
    }
    return fallbackFormat(amountMinor, currency)
  }
}
