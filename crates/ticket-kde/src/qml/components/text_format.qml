pragma Singleton

import QtQml

QtObject {
  function datePattern() {
    const pattern = Qt.locale().dateFormat(Locale.ShortFormat)
    return pattern.indexOf("yyyy") >= 0 ? pattern : pattern.replace(/y+/, "yyyy")
  }

  function timePattern() {
    return Qt.locale().timeFormat(Locale.ShortFormat)
  }

  function parseLocalDate(value) {
    const match = /^(\d{4})-(\d{2})-(\d{2})/.exec(String(value || ""))
    if (!match) return null
    return new Date(Number(match[1]), Number(match[2]) - 1, Number(match[3]))
  }

  function parseStamp(value) {
    const parsed = new Date(String(value || ""))
    return isNaN(parsed.getTime()) ? null : parsed
  }

  function dayDate(value) {
    const parsed = parseLocalDate(value)
    return parsed ? Qt.formatDate(parsed, datePattern()) : String(value || "")
  }

  function dayDateLong(value) {
    const parsed = parseLocalDate(value)
    return parsed ? parsed.toLocaleDateString(Qt.locale(), Locale.LongFormat) : String(value || "")
  }

  function stampDateTime(value) {
    const parsed = parseStamp(value)
    if (!parsed) return String(value || "")
    return Qt.formatDateTime(parsed, datePattern() + " " + timePattern())
  }

  function stampTime(value) {
    const parsed = parseStamp(value)
    return parsed ? Qt.formatTime(parsed, timePattern()) : String(value || "")
  }

  function ticketCount(count) {
    const normalized = Number(count || 0)
    if (LocaleText.isCzech) {
      if (normalized === 1) return normalized + " vstupenka"
      if (normalized > 1 && normalized < 5) return normalized + " vstupenky"
      return normalized + " vstupenek"
    }
    return normalized + (normalized === 1 ? " ticket" : " tickets")
  }

  function dayCount(count) {
    const normalized = Number(count || 0)
    if (LocaleText.isCzech) {
      if (normalized === 1) return normalized + " den"
      if (normalized > 1 && normalized < 5) return normalized + " dny"
      return normalized + " dnů"
    }
    return normalized + (normalized === 1 ? " day" : " days")
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

  function normalizeSubticketRow(subticket) {
    const source = subticket || {}
    return {
      id: source.id || "",
      amount_minor: Number(source.amount_minor || 0),
      currency: source.currency || "",
      keyboard_shortcut: source.keyboard_shortcut || "",
      enabled: source.enabled === undefined ? true : source.enabled,
    }
  }

  function serializeSubticketModel(model) {
    const values = []
    for (let index = 0; index < model.count; ++index) {
      const subticket = model.get(index)
      const label = MoneyFormat.format(subticket.amount_minor, subticket.currency)
      values.push({
          id: subticket.id || "",
          name: label,
          amount_minor: Number(subticket.amount_minor || 0),
          currency: subticket.currency || "",
          display_label: label,
          keyboard_shortcut: subticket.keyboard_shortcut ? subticket.keyboard_shortcut : null,
          enabled: subticket.enabled === undefined ? true : subticket.enabled,
          sort_order: index,
      })
    }
    return values
  }
}
