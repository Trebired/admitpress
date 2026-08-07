import QtQuick
import "../components"
QtObject {
  id: logic
  property var root
  property var backend
  property var editor
  property var props
  property var canvas
  property var fieldsModel
  property var classicFieldsModel
  property var bulkFieldsModel
  property var subticketsModel

  property QtObject selection: EditorSelection {
    editor: logic.editor
    fieldsModel: logic.fieldsModel
    logic: logic
  }

  property QtObject previewData: EditorPreviewData {
    root: logic.root
    editor: logic.editor
    props: logic.props
    subticketsModel: logic.subticketsModel
  }

  property QtObject grid: EditorGrid {
    logic: logic
  }

  property QtObject fieldDefaults: EditorFieldDefaults {
    logic: logic
  }

  property QtObject textMetrics: EditorTextMetrics {}

  property QtObject history: EditorHistory {
    logic: logic
  }

  property QtObject clipboard: EditorClipboard {
    root: logic.root
    backend: logic.backend
    editor: logic.editor
    logic: logic
    fieldsModel: logic.fieldsModel
  }

  function copySelectedFields() {
    return clipboard.copySelectedFields()
  }

  function pasteFields() {
    return clipboard.pasteFields()
  }

  property QtObject persistence: EditorPersistence {
    root: logic.root
    backend: logic.backend
    editor: logic.editor
    logic: logic
    props: logic.props
    fieldsModel: logic.fieldsModel
    classicFieldsModel: logic.classicFieldsModel
    bulkFieldsModel: logic.bulkFieldsModel
    subticketsModel: logic.subticketsModel
  }

  function load() {
    const response = root.acceptResponse(backend.loadTicket(editor.ticketId, backend.today()))
    if (!response) return false
    editor.summary = response
    props.ticketName = response.ticket.name
    props.widthValue = Number(response.ticket.width_mm / 10)
    props.heightValue = Number(response.ticket.height_mm / 10)
    props.dpiValue = response.ticket.dpi
    props.startValue = response.ticket.number_start
    props.incrementValue = response.ticket.number_increment
    props.paddingValue = response.ticket.number_padding
    props.numberPrefix = response.ticket.number_prefix
    props.numberSuffix = response.ticket.number_suffix
    editor.backgroundPath = response.ticket.background_svg_path || ""
    editor.borderStyle = response.ticket.border_style || "none"
    editor.borderWidthMm = Number(response.ticket.border_width_mm || 0.2)
    props.borderWidthValue = editor.borderWidthMm
    classicFieldsModel.clear()
    bulkFieldsModel.clear()
    for (let field of response.fields) {
      const templateKind = field.template_kind === "bulk" ? "bulk" : "classic"
      field.template_kind = templateKind
      if (templateKind === "bulk") {
        if (!isDeprecatedBulkTemplateField(field)) bulkFieldsModel.append(field)
      }
      else classicFieldsModel.append(field)
    }
    refreshFieldPalette()
    subticketsModel.clear()
    for (let subticket of response.subtickets) {
      subticketsModel.append(TextFormat.normalizeSubticketRow(subticket))
    }
    selection.selectSingle(fieldsModel.count > 0 ? 0 : -1)
    editor.markSavedState()
    history.reset()
    return true
  }

  function positiveNumber(value, fallback) {
    const numericValue = Number(value)
    return Number.isFinite(numericValue) && numericValue > 0 ? numericValue : fallback
  }

  function parsedNumber(value) {
    if (typeof value === "number") return value
    const numericValue = Number(String(value).replace(",", "."))
    return Number.isFinite(numericValue) ? numericValue : NaN
  }

  function ticketWidthCm() {
    return positiveNumber(props.widthValue, 5)
  }

  function ticketHeightCm() {
    return positiveNumber(props.heightValue, 7)
  }

  function ticketWidthMm() {
    return Math.max(1, ticketWidthCm() * 10)
  }

  function ticketHeightMm() {
    return Math.max(1, ticketHeightCm() * 10)
  }

  function isVerticalOrientation() {
    return ticketHeightCm() >= ticketWidthCm()
  }

  function clamp(value, minimum, maximum) {
    return Math.max(minimum, Math.min(maximum, value))
  }

  function setOrientation(vertical) {
    const width = ticketWidthCm()
    const height = ticketHeightCm()
    if (width !== height && isVerticalOrientation() === vertical) return
    const shortSide = Math.min(width, height)
    const longSide = Math.max(width, height)
    props.widthValue = vertical ? shortSide : longSide
    props.heightValue = vertical ? longSide : shortSide
    normalizeFieldsToTicket()
    if (canvas) canvas.updateScale()
  }

  function borderStyleIndex() {
    const styles = ["none", "solid", "dashed", "dotted"];
    const index = styles.indexOf(editor.borderStyle);
    return index < 0 ? 0 : index
  }

  function parsedBorderWidthMm() {
    return clamp(positiveNumber(props.borderWidthValue, 0.2), 0.1, 5.0)
  }

  function dateFormatIndex(format) {
    const formats = ["date", "date_time", "full_date_time", "iso_date", "iso_date_time", "time"];
    const index = formats.indexOf(format || "date");
    return index < 0 ? 0 : index
  }

  function dateFormatValue(index) {
    const formats = ["date", "date_time", "full_date_time", "iso_date", "iso_date_time", "time"];
    return formats[Math.max(0, Math.min(index, formats.length - 1))]
  }

  function previewTextForField(field, index) {
    return previewData.textFor(field, index)
  }

  function refreshSamplePreview() {
    editor.samplePreviewSeed = Math.floor(Math.random() * 1000000) + 1
  }

  function selectedField() {
    return editor.selectedFieldIndex >= 0 && editor.selectedFieldIndex < fieldsModel.count
    ? fieldsModel.get(editor.selectedFieldIndex)
    : null
  }

  function selectedFieldIsDate() {
    const field = selectedField();
    return !!field && field.field_kind === "dynamic" && field.dynamic_kind === "date"
  }

  function selectedFieldIsImage() {
    const field = selectedField();
    return !!field && field.field_kind === "image"
  }

  function selectedFieldIsNumber() {
    const field = selectedField();
    return !!field && field.field_kind === "dynamic" && field.dynamic_kind === "number"
  }

  function hasDynamicField(dynamicKind) {
    if (!dynamicKind) return false
    return hasDynamicFieldInModel(fieldsModel, dynamicKind)
  }

  function isDeprecatedBulkTemplateField(field) {
    if (!field || field.field_kind !== "dynamic") return false
    return field.dynamic_kind === "bulk_count" || field.dynamic_kind === "bulk_total"
  }

  function refreshFieldPalette() {
    if (editor) editor.fieldPaletteRevision += 1
  }

  function canAddField(fieldKind, dynamicKind) {
    return fieldKind !== "dynamic" || dynamicKind === "" || !hasDynamicField(dynamicKind)
  }

  function hasDynamicFieldInModel(model, dynamicKind) {
    if (!model || !dynamicKind) return false
    for (let index = 0; index < model.count; index++) {
      const field = model.get(index)
      if (field.field_kind === "dynamic" && field.dynamic_kind === dynamicKind) return true
    }
    return false
  }

  function selectedFieldUsesText() {
    const field = selectedField();
    return !!field && field.field_kind !== "image"
  }

  function appendField(fieldKind, dynamicKind, text, xMm, yMm, widthMm, heightMm) {
    fieldsModel.append({
        id: "", template_kind: editor.activeTicketType, field_kind: fieldKind,
        dynamic_kind: dynamicKind, text: text,
        x_mm: xMm, y_mm: yMm, width_mm: widthMm, height_mm: heightMm,
        font_family: "Noto Sans", font_size_pt: 12, font_weight: 400,
        horizontal_alignment: "center", vertical_alignment: "center", date_format: "date",
        prefix: "", suffix: "", sort_order: fieldsModel.count
    })
    selection.selectSingle(fieldsModel.count - 1)
    refreshFieldPalette()
  }

  function addField(fieldKind, dynamicKind, label) {
    addFieldAt(fieldKind, dynamicKind, label, ticketWidthMm() / 2, ticketHeightMm() / 2, false)
  }

  function addImportedImage(image) {
    if (!image || !image.path) return
    const size = fieldDefaults.imageSizeMm(image.width_px, image.height_px)
    appendField("image", null, image.path, (ticketWidthMm() - size.width) / 2, (ticketHeightMm() - size.height) / 2, size.width, size.height)
  }

  function addFieldAt(fieldKind, dynamicKind, label, centerXmm, centerYmm, useSnap) {
    const kind = dynamicKind === "" ? null : dynamicKind
    if (!canAddField(fieldKind, kind)) return false
    const width = fieldDefaults.widthMm(kind)
    const height = fieldDefaults.heightMm(kind)
    const shouldSnap = useSnap === undefined ? true : useSnap
    const rawX = centerXmm - width / 2
    const rawY = centerYmm - height / 2
    const x = clamp(shouldSnap ? grid.snapXMm(rawX) : rawX, 0, Math.max(0, ticketWidthMm() - width))
    const y = clamp(shouldSnap ? grid.snapYMm(rawY) : rawY, 0, Math.max(0, ticketHeightMm() - height))
    appendField(fieldKind, kind, fieldKind === "static" ? label : "", x, y, width, height)
    return true
  }

  function updateFieldGeometry(index, xMm, yMm, widthMm, heightMm, useSnap) {
    if (index < 0 || index >= fieldsModel.count) return
    const shouldSnap = useSnap === undefined ? true : useSnap
    const width = clamp(shouldSnap ? grid.snapXLengthMm(widthMm) : widthMm, 1, ticketWidthMm())
    const height = clamp(shouldSnap ? grid.snapYLengthMm(heightMm) : heightMm, 1, ticketHeightMm())
    const x = clamp(shouldSnap ? grid.snapXMm(xMm) : xMm, 0, Math.max(0, ticketWidthMm() - width))
    const y = clamp(shouldSnap ? grid.snapYMm(yMm) : yMm, 0, Math.max(0, ticketHeightMm() - height))
    fieldsModel.setProperty(index, "x_mm", x)
    fieldsModel.setProperty(index, "y_mm", y)
    fieldsModel.setProperty(index, "width_mm", width)
    fieldsModel.setProperty(index, "height_mm", height)
  }

  function normalizeFieldsToTicket() {
    for (let index = 0; index < fieldsModel.count; ++index) {
      const field = fieldsModel.get(index)
      updateFieldGeometry(index, field.x_mm, field.y_mm, field.width_mm, field.height_mm)
    }
  }

  function selectedFieldValue(name, fallback) {
    const field = selectedField();
    return field ? Number(field[name]) : fallback
  }

  function setSelectedFieldNumber(name, rawValue) {
    const field = selectedField()
    if (!field) return
    const parsed = parsedNumber(rawValue)
    if (!Number.isFinite(parsed)) return
    let minimum = 0
    let maximum = 9999
    if (name === "x_mm") maximum = Math.max(0, ticketWidthMm() - field.width_mm)
    else if (name === "y_mm") maximum = Math.max(0, ticketHeightMm() - field.height_mm)
    else if (name === "width_mm") {
      minimum = 1;
      maximum = Math.max(1, ticketWidthMm() - field.x_mm)
    }
    else if (name === "height_mm") {
      minimum = 1;
      maximum = Math.max(1, ticketHeightMm() - field.y_mm)
    }
    let snapped = parsed
    if (name === "x_mm") snapped = grid.snapXMm(parsed)
    else if (name === "y_mm") snapped = grid.snapYMm(parsed)
    else if (name === "width_mm") snapped = grid.snapXLengthMm(parsed)
    else if (name === "height_mm") snapped = grid.snapYLengthMm(parsed)
    fieldsModel.setProperty(editor.selectedFieldIndex, name, clamp(snapped, minimum, maximum))
  }

  function centerSelectedFieldHorizontally() {
    const field = selectedField();
    if (!field) return;
    updateFieldGeometry(editor.selectedFieldIndex, (ticketWidthMm() - field.width_mm) / 2, field.y_mm, field.width_mm, field.height_mm, false);
    fieldsModel.setProperty(editor.selectedFieldIndex, "horizontal_alignment", "center")
  }

  function centerSelectedFieldVertically() {
    const field = selectedField();
    if (!field) return;
    updateFieldGeometry(editor.selectedFieldIndex, field.x_mm, (ticketHeightMm() - field.height_mm) / 2, field.width_mm, field.height_mm, false);
    fieldsModel.setProperty(editor.selectedFieldIndex, "vertical_alignment", "center")
  }

  function centerSelectedFieldBoth() {
    const field = selectedField();
    if (!field) return;
    updateFieldGeometry(
      editor.selectedFieldIndex,
      (ticketWidthMm() - field.width_mm) / 2,
      (ticketHeightMm() - field.height_mm) / 2,
      field.width_mm,
      field.height_mm,
      false
    );
    fieldsModel.setProperty(editor.selectedFieldIndex, "horizontal_alignment", "center");
    fieldsModel.setProperty(editor.selectedFieldIndex, "vertical_alignment", "center")
  }

  function removeSelectedField() {
    const indices = selection.selectedIndices()
    if (indices.length === 0) return
    const nextIndex = indices[0]
    for (let offset = indices.length - 1; offset >= 0; --offset) {
      fieldsModel.remove(indices[offset])
    }
    selection.selectSingle(Math.min(nextIndex, fieldsModel.count - 1))
    refreshFieldPalette()
  }

}
