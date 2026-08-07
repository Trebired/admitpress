pragma Singleton

import QtQml

QtObject {
  function ticketCount(count) {
    const normalized = Number(count || 0)
    if (LocaleText.isCzech) {
      if (normalized === 1) return normalized + " vstupenka"
      if (normalized > 1 && normalized < 5) return normalized + " vstupenky"
      return normalized + " vstupenek"
    }
    return normalized + (normalized === 1 ? " ticket" : " tickets")
  }

  function ticketNumber(value, padding) {
    const number = String(value || 1)
    const width = Math.max(0, Number(padding || 0))
    return width > 0 ? number.padStart(width, "0") : number
  }

  function displayTicketNumber(value, padding, showPrefix) {
    const prefix = showPrefix === false ? "" : "#"
    return prefix + ticketNumber(value, padding)
  }
}
