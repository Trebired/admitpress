import QtQuick
import "../components"

QtObject {
  id: fieldClipboardLogic

  property var root
  property var backend
  property var editor
  property var logic
  property var fieldsModel

  function dynamicKindFitsTemplate(dynamicKind, templateKind) {
    const kind = dynamicKind || ""
    if (kind === "" || kind === "date") return true
    if (templateKind === "bulk") return kind === "bulk_name" || kind === "bulk_items"
    return kind === "price" || kind === "number"
  }

  function copySelectedFields() {
    const indices = logic.selection.selectedIndices()
    if (indices.length === 0) return false
    const copied = []
    for (let index of indices) {
      copied.push(Object.assign({}, fieldsModel.get(index)))
    }
    FieldClipboard.store(copied, editor.ticketId, editor.activeTicketType)
    return true
  }

  function canPasteField(field) {
    if (field.field_kind !== "dynamic") return true
    const kind = field.dynamic_kind || ""
    if (!dynamicKindFitsTemplate(kind, editor.activeTicketType)) return false
    return kind === "" || !logic.hasDynamicField(kind)
  }

  function pastedFieldText(field) {
    if (field.field_kind !== "image") return field.text
    if (FieldClipboard.sourceTicketId === editor.ticketId) return field.text
    const response = root.parseResponse(backend.importImage(editor.ticketId, field.text))
    return response.ok ? response.data.path : field.text
  }

  function pasteFields() {
    if (!FieldClipboard.hasFields) return false
    const offset = 2
    const pasted = []
    for (let field of FieldClipboard.fields) {
      if (!canPasteField(field)) continue
      const copy = Object.assign({}, field)
      copy.id = ""
      copy.template_kind = editor.activeTicketType
      copy.text = pastedFieldText(field)
      copy.x_mm = logic.clamp(
        Number(field.x_mm) + offset,
        0,
        Math.max(0, logic.ticketWidthMm() - field.width_mm)
      )
      copy.y_mm = logic.clamp(
        Number(field.y_mm) + offset,
        0,
        Math.max(0, logic.ticketHeightMm() - field.height_mm)
      )
      copy.sort_order = fieldsModel.count
      fieldsModel.append(copy)
      pasted.push(fieldsModel.count - 1)
    }
    if (pasted.length === 0) return false
    logic.selection.selectIndices(pasted)
    logic.refreshFieldPalette()
    return true
  }
}
