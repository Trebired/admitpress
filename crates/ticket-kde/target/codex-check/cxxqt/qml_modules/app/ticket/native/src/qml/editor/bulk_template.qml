import QtQuick

QtObject {
  id: bulkTemplate

  property var logic
  property var bulkFieldsModel

  function hasDynamicField(dynamicKind) {
    if (!bulkFieldsModel || !dynamicKind) return false
    for (let index = 0; index < bulkFieldsModel.count; index++) {
      const field = bulkFieldsModel.get(index)
      if (field.field_kind === "dynamic" && field.dynamic_kind === dynamicKind) return true
    }
    return false
  }

  function appendField(dynamicKind, xMm, yMm, widthMm, heightMm, fontSize) {
    const ticketWidth = logic.ticketWidthMm()
    const ticketHeight = logic.ticketHeightMm()
    const x = logic.clamp(xMm, 0, Math.max(0, ticketWidth - 1))
    const y = logic.clamp(yMm, 0, Math.max(0, ticketHeight - 1))
    const width = logic.clamp(widthMm, 1, Math.max(1, ticketWidth - x))
    const height = logic.clamp(heightMm, 1, Math.max(1, ticketHeight - y))
    bulkFieldsModel.append({
        id: "", template_kind: "bulk", field_kind: "dynamic",
        dynamic_kind: dynamicKind, text: "",
        x_mm: x, y_mm: y, width_mm: width, height_mm: height,
        font_family: "Noto Sans", font_size_pt: fontSize,
        font_weight: 400,
        horizontal_alignment: "center", vertical_alignment: "center", date_format: "date",
        prefix: "", suffix: "", sort_order: bulkFieldsModel.count
    })
  }

  function appendTotalField(width, y) {
    const ticketWidth = logic.ticketWidthMm()
    const ticketHeight = logic.ticketHeightMm()
    const x = logic.clamp(5 + width / 2, 0, Math.max(0, ticketWidth - 1))
    const fieldWidth = logic.clamp(width / 2 - 1, 1, Math.max(1, ticketWidth - x))
    const fieldHeight = logic.clamp(Math.min(7, ticketHeight), 1, Math.max(1, ticketHeight - y))
    appendField("bulk_total", x, y, fieldWidth, fieldHeight, 10)
  }

  function ensureRequiredFields() {
    const ticketHeight = logic.ticketHeightMm()
    const width = Math.max(8, logic.ticketWidthMm() - 8)
    const gap = Math.max(1, Math.min(3, ticketHeight / 20))
    let y = Math.min(4, Math.max(0, ticketHeight - 8))
    let added = 0
    if (!hasDynamicField("bulk_name")) {
      appendField("bulk_name", 4, y, width, Math.min(10, ticketHeight), 15)
      added += 1
    }
    y = Math.min(ticketHeight - 6, y + 10 + gap)
    if (!hasDynamicField("bulk_items")) {
      appendField("bulk_items", 4, y, width, Math.min(8, ticketHeight), 10)
      added += 1
    }
    y = Math.min(ticketHeight - 6, y + 8 + gap)
    if (!hasDynamicField("bulk_count")) {
      appendField("bulk_count", 4, y, width / 2 - 1, Math.min(7, ticketHeight), 10)
      added += 1
    }
    if (!hasDynamicField("bulk_total")) {
      appendTotalField(width, y)
      added += 1
    }
    return added
  }
}
