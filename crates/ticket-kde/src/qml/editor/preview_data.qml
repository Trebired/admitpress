import QtQuick
import "../components"

QtObject {
  id: previewData

  property var root
  property var editor
  property var props
  property var subticketsModel

  function placeholder(dynamicKind) {
    if (dynamicKind === "price") return "{price}"
    if (dynamicKind === "date") return "{date}"
    if (dynamicKind === "number") return "{number}"
    if (dynamicKind === "bulk_name") return "{group name}"
    if (dynamicKind === "bulk_items") return "{tickets}"
    if (dynamicKind === "bulk_count") return "{count}"
    if (dynamicKind === "bulk_total") return "{total}"
    return "{value}"
  }

  function offset(index, limit) {
    const seed = editor ? Number(editor.samplePreviewSeed || 1) : 1
    const raw = Math.abs(Math.sin(seed * 12.9898 + index * 78.233) * 43758.5453)
    return Math.floor((raw - Math.floor(raw)) * limit)
  }

  function mainCurrency() {
    return root && root.appState ? root.appState.main_currency || "" : ""
  }

  function sampleCurrency(index) {
    const currencies = root && root.appState && root.appState.available_currencies
    ? root.appState.available_currencies
    : []
    if (currencies.length === 0) return mainCurrency()
    return currencies[offset(index, currencies.length)].code
  }

  function samplePriceText(index) {
    if (subticketsModel && subticketsModel.count > 0) {
      const subticket = subticketsModel.get(offset(index, subticketsModel.count))
      return MoneyFormat.format(subticket.amount_minor, sampleCurrency(index))
    }
    const samples = [1200, 2500, 5000, 7500, 9900]
    return MoneyFormat.format(samples[offset(index, samples.length)], sampleCurrency(index))
  }

  function sampleNumberText(index) {
    const start = Math.max(0, Number(props.startValue || 1))
    const increment = Math.max(1, Number(props.incrementValue || 1))
    const padding = Math.max(0, Number(props.paddingValue || 0))
    const value = start + offset(index, 60) * increment
    const padded = padding > 0 ? String(value).padStart(padding, "0") : String(value)
    return String(props.numberPrefix || "") + padded + String(props.numberSuffix || "")
  }

  function sampleDateParts(index) {
    const month = 1 + offset(index + 3, 12)
    const day = 1 + offset(index + 7, 28)
    const hour = 8 + offset(index + 11, 10)
    const minute = offset(index + 13, 60)
    const second = offset(index + 17, 60)
    return {
      isoDate: "2026-" + String(month).padStart(2, "0") + "-" + String(day).padStart(2, "0"),
      displayDate: String(day).padStart(2, "0") + "." + String(month).padStart(2, "0") + ".2026",
      time: String(hour).padStart(2, "0") + ":" + String(minute).padStart(2, "0"),
      second: String(second).padStart(2, "0"),
    }
  }

  function sampleDateText(format, index) {
    const parts = sampleDateParts(index)
    if (format === "date_time") return parts.displayDate + " " + parts.time
    if (format === "full_date_time") return parts.displayDate + " " + parts.time + ":" + parts.second
    if (format === "iso_date") return parts.isoDate
    if (format === "iso_date_time") return parts.isoDate + " " + parts.time + ":" + parts.second
    if (format === "time") return parts.time
    return parts.displayDate
  }

  function sampleBulkItemsText() {
    const currency = sampleCurrency(0)
    return "2x " + MoneyFormat.format(1200, currency) + "\n"
    + "3x " + MoneyFormat.format(700, currency) + "\n"
    + "Total: " + MoneyFormat.format(4500, currency)
  }

  function rawText(field, index) {
    if (!field) return ""
    if (field.field_kind !== "dynamic") return field.text
    if (!editor.samplePreviewEnabled) return placeholder(field.dynamic_kind)
    if (field.dynamic_kind === "price") return samplePriceText(index)
    if (field.dynamic_kind === "date") return sampleDateText(field.date_format, index)
    if (field.dynamic_kind === "number") return sampleNumberText(index)
    if (field.dynamic_kind === "bulk_name") return "Morning school group"
    if (field.dynamic_kind === "bulk_items") return sampleBulkItemsText()
    if (field.dynamic_kind === "bulk_count") return "5"
    if (field.dynamic_kind === "bulk_total") return MoneyFormat.format(4500, sampleCurrency(0))
    return field.text
  }

  function textFor(field, index) {
    return String(field.prefix || "") + rawText(field, index) + String(field.suffix || "")
  }
}
