import QtQuick
import "../components"

QtObject {
  id: persistence

  property var root
  property var backend
  property var editor
  property var logic
  property var props
  property var fieldsModel
  property var classicFieldsModel
  property var bulkFieldsModel
  property var subticketsModel

  function serializeModel(model, templateKind) {
    const values = []
    for (let index = 0; index < model.count; ++index) {
      const field = Object.assign({}, model.get(index))
      field.template_kind = templateKind
      field.sort_order = index
      values.push(field)
    }
    return values
  }

  function serializeFields() {
    return serializeModel(classicFieldsModel, "classic")
    .concat(serializeModel(bulkFieldsModel, "bulk"))
  }

  function priceLabel(amountMinor, currency) {
    return root.formatMoney(amountMinor, currency || "")
  }

  function serializeSubtickets() {
    const values = []
    for (let index = 0; index < subticketsModel.count; ++index) {
      const subticket = subticketsModel.get(index)
      const label = priceLabel(subticket.amount_minor, subticket.currency)
      values.push({
          id: subticket.id || "",
          name: label,
          amount_minor: Number(subticket.amount_minor || 0),
          currency: subticket.currency || "",
          display_label: label,
          keyboard_shortcut: subticket.keyboard_shortcut || null,
          enabled: subticket.enabled === undefined ? true : subticket.enabled,
          sort_order: index,
      })
    }
    return values
  }

  function snapshot() {
    return JSON.stringify({
        ticket: {
          name: props.ticketName,
          background_path: editor.backgroundPath || "",
          width_cm: logic.positiveNumber(props.widthValue, 5),
          height_cm: logic.positiveNumber(props.heightValue, 7),
          border_style: editor.borderStyle,
          border_width_mm: logic.parsedBorderWidthMm(),
          dpi: props.dpiValue,
          number_start: props.startValue,
          number_increment: props.incrementValue,
          number_padding: props.paddingValue,
          number_prefix: props.numberPrefix,
          number_suffix: props.numberSuffix,
        },
        fields: serializeFields(),
        subtickets: serializeSubtickets(),
    })
  }

  function restore(snapshotJson) {
    const state = JSON.parse(snapshotJson)
    const ticket = state.ticket
    props.ticketName = ticket.name
    props.widthValue = ticket.width_cm
    props.heightValue = ticket.height_cm
    props.dpiValue = ticket.dpi
    props.startValue = ticket.number_start
    props.incrementValue = ticket.number_increment
    props.paddingValue = ticket.number_padding
    props.numberPrefix = ticket.number_prefix
    props.numberSuffix = ticket.number_suffix
    props.borderWidthValue = ticket.border_width_mm
    editor.backgroundPath = ticket.background_path
    editor.borderStyle = ticket.border_style
    editor.borderWidthMm = ticket.border_width_mm

    classicFieldsModel.clear()
    bulkFieldsModel.clear()
    for (const field of state.fields) {
      const templateKind = field.template_kind === "bulk" ? "bulk" : "classic"
      field.template_kind = templateKind
      if (templateKind === "bulk") bulkFieldsModel.append(field)
      else classicFieldsModel.append(field)
    }
    logic.ensureRequiredBulkFields()
    logic.refreshFieldPalette()
    subticketsModel.clear()
    for (const subticket of state.subtickets) subticketsModel.append(subticket)
    logic.selection.selectSingle(fieldsModel.count > 0 ? 0 : -1)
  }

  function save(completion) {
    const widthCm = logic.positiveNumber(props.widthValue, 5)
    const heightCm = logic.positiveNumber(props.heightValue, 7)
    const fieldsJson = JSON.stringify(serializeFields())
    const subticketsJson = JSON.stringify(serializeSubtickets())
    const started = root.runBackendBusy(LocaleText.tr("Saving ticket"), "save_ticket", function() {
        backend.saveTicketAsync(
          editor.ticketId, props.ticketName, widthCm, heightCm, "",
          editor.borderStyle, logic.parsedBorderWidthMm(), props.dpiValue,
          props.startValue, props.incrementValue, props.paddingValue,
          props.numberPrefix, props.numberSuffix, fieldsJson, subticketsJson
        )
      }, function(responseJson) {
        const response = root.acceptResponse(responseJson, LocaleText.tr("Ticket saved"))
        if (response) {
          logic.load()
          if (completion) completion(true)
        } else if (completion) {
          completion(false)
        }
    })
    if (!started && completion) completion(false)
  }
}
