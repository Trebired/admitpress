import QtQuick
import QtQuick.Controls as Controls
import org.kde.kirigami as Kirigami

Rectangle {
  id: fieldItem

  required property int index
  required property string field_kind
  required property var dynamic_kind
  required property string text
  required property real x_mm
  required property real y_mm
  required property real width_mm
  required property real height_mm
  required property string font_family
  required property real font_size_pt
  required property int font_weight
  required property string horizontal_alignment
  required property string vertical_alignment
  required property string date_format
  required property string prefix
  required property string suffix

  property var root
  property var editor
  property var logic
  property var ticketCanvas
  readonly property int innerGap: 3
  property bool selected: logic && logic.selection ? logic.selection.isSelected(index) : editor.selectedFieldIndex === index
  property bool primarySelected: editor.selectedFieldIndex === index

  x: x_mm * editor.canvasScale
  y: y_mm * editor.canvasScale
  width: Math.max(8, width_mm * editor.canvasScale)
  height: Math.max(8, height_mm * editor.canvasScale)
  color: field_kind === "image"
  ? "transparent"
  : selected ? "#334f9cff" : "#1f4f9cff"
  border.width: selected ? 2 : 1
  border.color: selected ? "#1d5fd1" : "#4f9cff"
  radius: 2

  function previewText() {
    return logic.previewTextForField(fieldItem, index)
  }

  function capturePointer(mouse, area) {
    const shouldDrag = logic.selection.selectForPress(index, mouse.modifiers)
    const point = area.mapToItem(ticketCanvas, mouse.x, mouse.y)
    area.pressTicketX = point.x
    area.pressTicketY = point.y
    area.startFields = shouldDrag ? logic.selection.fieldGeometrySnapshot(logic.selection.selectedIndices()) : []
    return shouldDrag
  }

  function captureResizePointer(mouse, area) {
    logic.selection.setPrimary(index)
    const point = area.mapToItem(ticketCanvas, mouse.x, mouse.y)
    area.pressTicketX = point.x
    area.pressTicketY = point.y
    area.startFields = logic.selection.fieldGeometrySnapshot([index])
  }

  Image {
    x: fieldItem.innerGap
    y: fieldItem.innerGap
    width: parent.width - fieldItem.innerGap * 2
    height: parent.height - fieldItem.innerGap * 2
    source: field_kind === "image" ? root.fileUrl(text) : ""
    fillMode: Image.PreserveAspectFit
    asynchronous: true
    cache: false
    smooth: true
    visible: field_kind === "image"
  }

  Controls.Label {
    x: fieldItem.innerGap
    y: fieldItem.innerGap
    width: parent.width - fieldItem.innerGap * 2
    height: parent.height - fieldItem.innerGap * 2
    visible: field_kind !== "image"
    text: fieldItem.previewText()
    textFormat: Text.PlainText
    lineHeight: 1.18
    color: "black"
    font.family: fieldItem.font_family || "Noto Sans"
    font.pixelSize: Math.max(5, logic.textMetrics.fittedFontSizeMm(fieldItem, fieldItem.previewText()) * editor.canvasScale)
    font.weight: font_weight
    horizontalAlignment: horizontal_alignment === "center"
    ? Text.AlignHCenter
    : horizontal_alignment === "right" ? Text.AlignRight : Text.AlignLeft
    verticalAlignment: vertical_alignment === "top"
    ? Text.AlignTop
    : vertical_alignment === "bottom" ? Text.AlignBottom : Text.AlignVCenter
    elide: Text.ElideNone
    clip: true
  }

  MouseArea {
    id: dragArea
    anchors.fill: parent
    preventStealing: true
    property real pressTicketX: 0
    property real pressTicketY: 0
    property var startFields: []
    property bool dragActive: false
    property bool moved: false

    onPressed: mouse => {
      dragActive = fieldItem.capturePointer(mouse, dragArea)
      moved = false
    }

    onPositionChanged: mouse => {
      if (!pressed || !dragActive) return
      moved = true
      const point = dragArea.mapToItem(ticketCanvas, mouse.x, mouse.y)
      logic.selection.moveGeometrySnapshot(
        startFields,
        (point.x - pressTicketX) / editor.canvasScale,
        (point.y - pressTicketY) / editor.canvasScale,
        false
      )
    }

    onReleased: {
      if (dragActive && moved) logic.selection.settleGeometrySnapshot(startFields)
      dragActive = false
      moved = false
    }
  }

  Rectangle {
    visible: fieldItem.primarySelected
    width: 12
    height: 12
    anchors.right: parent.right
    anchors.bottom: parent.bottom
    color: "#1d5fd1"
    border.color: "white"

    MouseArea {
      id: resizeArea
      anchors.fill: parent
      cursorShape: Qt.SizeFDiagCursor
      preventStealing: true
      property real pressTicketX: 0
      property real pressTicketY: 0
      property real startXmm: 0
      property real startYmm: 0
      property real startWidthMm: 0
      property real startHeightMm: 0
      property var startFields: []
      property bool moved: false

      onPressed: mouse => {
        const area = resizeArea
        fieldItem.captureResizePointer(mouse, area)
        moved = false
        const field = startFields.length > 0 ? startFields[0] : null
        if (!field) return
        startXmm = field.x_mm
        startYmm = field.y_mm
        startWidthMm = field.width_mm
        startHeightMm = field.height_mm
      }

      onPositionChanged: mouse => {
        if (!pressed) return
        moved = true
        const point = resizeArea.mapToItem(ticketCanvas, mouse.x, mouse.y)
        logic.updateFieldGeometry(
          index,
          startXmm,
          startYmm,
          startWidthMm + (point.x - pressTicketX) / editor.canvasScale,
          startHeightMm + (point.y - pressTicketY) / editor.canvasScale,
          false
        )
      }

      onReleased: {
        if (moved) logic.updateFieldGeometry(index, x_mm, y_mm, width_mm, height_mm, true)
        moved = false
      }
    }
  }
}
