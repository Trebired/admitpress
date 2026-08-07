import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"
import "."

Rectangle {
  id: canvasArea

  property var root
  property var editor
  property var logic
  property var fieldsModel

  color: Kirigami.Theme.alternateBackgroundColor
  property real ticketWidthMm: logic ? logic.ticketWidthMm() : 50
  property real ticketHeightMm: logic ? logic.ticketHeightMm() : 70
  property real rulerThickness: Kirigami.Units.gridUnit * 2.2
  property real ticketPixelWidth: ticketWidthMm * editor.canvasScale
  property real ticketPixelHeight: ticketHeightMm * editor.canvasScale

  onWidthChanged: updateScale()
  onHeightChanged: updateScale()
  onTicketWidthMmChanged: updateScale()
  onTicketHeightMmChanged: updateScale()

  function updateScale() {
    const usableWidth = Math.max(1, width - rulerThickness - 60)
    const usableHeight = Math.max(1, height - rulerThickness - 60)
    editor.canvasScale = Math.max(0.2, Math.min(
        usableWidth / ticketWidthMm,
        usableHeight / ticketHeightMm
    ))
  }

  Component.onCompleted: updateScale()

  MouseArea {
    anchors.fill: parent
    acceptedButtons: Qt.LeftButton
    onClicked: {
      if (logic && logic.selection) logic.selection.clear()
    }
  }

  Item {
    id: canvasFrame
    anchors.centerIn: parent
    width: canvasArea.ticketPixelWidth + canvasArea.rulerThickness
    height: canvasArea.ticketPixelHeight + canvasArea.rulerThickness

    Rectangle {
      x: canvasArea.rulerThickness
      y: 0
      width: canvasArea.ticketPixelWidth
      height: canvasArea.rulerThickness - 4
      color: Kirigami.Theme.backgroundColor
      border.color: Kirigami.Theme.disabledTextColor

      Controls.Label {
        anchors.centerIn: parent
        text: LocaleText.tr("Horizontal") + " "
        + Number(canvasArea.ticketWidthMm / 10).toLocaleString(Qt.locale(), "f", 1) + " cm"
      }
    }

    Rectangle {
      x: 0
      y: canvasArea.rulerThickness
      width: canvasArea.rulerThickness - 4
      height: canvasArea.ticketPixelHeight
      color: Kirigami.Theme.backgroundColor
      border.color: Kirigami.Theme.disabledTextColor

      Controls.Label {
        anchors.centerIn: parent
        text: LocaleText.tr("Vertical") + " "
        + Number(canvasArea.ticketHeightMm / 10).toLocaleString(Qt.locale(), "f", 1) + " cm"
        rotation: -90
      }
    }

    Rectangle {
      id: ticketCanvas
      x: canvasArea.rulerThickness
      y: canvasArea.rulerThickness
      width: canvasArea.ticketPixelWidth
      height: canvasArea.ticketPixelHeight
      color: "white"
      border.width: 1
      border.color: Kirigami.Theme.disabledTextColor
      clip: true
      property bool marqueeActive: false
      property bool marqueeMoved: false
      property bool marqueeAdditive: false
      property real marqueeStartX: 0
      property real marqueeStartY: 0
      property real marqueeEndX: 0
      property real marqueeEndY: 0
      readonly property real marqueeLeft: Math.min(marqueeStartX, marqueeEndX)
      readonly property real marqueeTop: Math.min(marqueeStartY, marqueeEndY)
      readonly property real marqueeRight: Math.max(marqueeStartX, marqueeEndX)
      readonly property real marqueeBottom: Math.max(marqueeStartY, marqueeEndY)

      function clampCanvasAxis(value, maximum) {
        return Math.max(0, Math.min(maximum, value))
      }

      function applyMarqueeSelection() {
        if (!logic || !logic.selection) return
        const scale = Math.max(0.0001, editor.canvasScale)
        logic.selection.selectRect(
          marqueeLeft / scale,
          marqueeTop / scale,
          marqueeRight / scale,
          marqueeBottom / scale,
          marqueeAdditive
        )
      }

      Image {
        anchors.fill: parent
        source: root.fileUrl(editor.backgroundPath)
        fillMode: Image.Stretch
        asynchronous: true
        visible: source !== ""
      }

      DropArea {
        anchors.fill: parent
        keys: ["ticket-field"]
        onDropped: function(drop) {
          const source = drop.source
          if (!source || !source.dragFieldKind) {
            return
          }
          const added = logic.addFieldAt(
            source.dragFieldKind,
            source.dragDynamicKind,
            source.dragDefaultText,
            drop.x / editor.canvasScale,
            drop.y / editor.canvasScale
          )
          if (added) drop.acceptProposedAction()
        }
      }

      Rectangle {
        visible: editor.snapToGrid
        x: (canvasArea.ticketWidthMm / 2 - logic.grid.gridStepMm(canvasArea.ticketWidthMm) / 2) * editor.canvasScale
        y: (canvasArea.ticketHeightMm / 2 - logic.grid.gridStepMm(canvasArea.ticketHeightMm) / 2) * editor.canvasScale
        width: logic.grid.gridStepMm(canvasArea.ticketWidthMm) * editor.canvasScale
        height: logic.grid.gridStepMm(canvasArea.ticketHeightMm) * editor.canvasScale
        color: "#201d5fd1"
        border.width: 1
        border.color: "#901d5fd1"
      }

      Repeater {
        model: editor.snapToGrid ? logic.grid.gridLineCount(canvasArea.ticketWidthMm) : 0
        delegate: Rectangle {
          required property int index
          x: logic.grid.gridLineMm(index, canvasArea.ticketWidthMm) * editor.canvasScale
          width: 1
          height: ticketCanvas.height
          color: logic.grid.isMajorGridLine(index, canvasArea.ticketWidthMm) ? "#70909090" : "#40909090"
        }
      }

      Repeater {
        model: editor.snapToGrid ? logic.grid.gridLineCount(canvasArea.ticketHeightMm) : 0
        delegate: Rectangle {
          required property int index
          y: logic.grid.gridLineMm(index, canvasArea.ticketHeightMm) * editor.canvasScale
          width: ticketCanvas.width
          height: 1
          color: logic.grid.isMajorGridLine(index, canvasArea.ticketHeightMm) ? "#70909090" : "#40909090"
        }
      }

      Canvas {
        id: ticketBorderCanvas
        anchors.fill: parent
        z: 40
        visible: editor.borderStyle !== "none" && editor.borderWidthMm > 0
        property real lineWidthPx: Math.max(1, editor.borderWidthMm * editor.canvasScale)
        property string paintKey: editor.borderStyle + ":" + editor.borderWidthMm + ":" + editor.canvasScale

        onVisibleChanged: requestPaint()
        onLineWidthPxChanged: requestPaint()
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        onPaintKeyChanged: requestPaint()

        onPaint: {
          const ctx = getContext("2d")
          ctx.clearRect(0, 0, width, height)
          if (!visible) {
            return
          }

          const lineWidth = lineWidthPx
          const inset = lineWidth / 2
          ctx.strokeStyle = "black"
          ctx.lineWidth = lineWidth
          ctx.lineCap = editor.borderStyle === "dotted" ? "round" : "butt"
          if (ctx.setLineDash) {
            if (editor.borderStyle === "dashed") {
              ctx.setLineDash([lineWidth * 8, lineWidth * 4])
            } else if (editor.borderStyle === "dotted") {
              ctx.setLineDash([lineWidth * 0.1, lineWidth * 3])
            } else {
              ctx.setLineDash([])
            }
          }
          ctx.strokeRect(inset, inset, Math.max(0, width - lineWidth), Math.max(0, height - lineWidth))
        }
      }

      MouseArea {
        id: marqueeArea

        anchors.fill: parent
        acceptedButtons: Qt.LeftButton
        cursorShape: Qt.CrossCursor
        preventStealing: true

        onPressed: mouse => {
          if (mouse.button !== Qt.LeftButton) return
          ticketCanvas.marqueeActive = true
          ticketCanvas.marqueeMoved = false
          ticketCanvas.marqueeAdditive = (mouse.modifiers & Qt.ShiftModifier) !== 0
          ticketCanvas.marqueeStartX = ticketCanvas.clampCanvasAxis(mouse.x, ticketCanvas.width)
          ticketCanvas.marqueeStartY = ticketCanvas.clampCanvasAxis(mouse.y, ticketCanvas.height)
          ticketCanvas.marqueeEndX = ticketCanvas.marqueeStartX
          ticketCanvas.marqueeEndY = ticketCanvas.marqueeStartY
          mouse.accepted = true
        }

        onPositionChanged: mouse => {
          if (!pressed || !ticketCanvas.marqueeActive) return
          ticketCanvas.marqueeEndX = ticketCanvas.clampCanvasAxis(mouse.x, ticketCanvas.width)
          ticketCanvas.marqueeEndY = ticketCanvas.clampCanvasAxis(mouse.y, ticketCanvas.height)
          const deltaX = ticketCanvas.marqueeEndX - ticketCanvas.marqueeStartX
          const deltaY = ticketCanvas.marqueeEndY - ticketCanvas.marqueeStartY
          if (Math.abs(deltaX) > 3 || Math.abs(deltaY) > 3) {
            ticketCanvas.marqueeMoved = true
            ticketCanvas.applyMarqueeSelection()
          }
        }

        onReleased: mouse => {
          if (!ticketCanvas.marqueeActive) return
          if (ticketCanvas.marqueeMoved) ticketCanvas.applyMarqueeSelection()
          else if (logic && logic.selection) logic.selection.clear()
          ticketCanvas.marqueeActive = false
          mouse.accepted = true
        }

        onCanceled: ticketCanvas.marqueeActive = false
      }

      Repeater {
        model: canvasArea.fieldsModel

        delegate: EditorCanvasField {
          root: canvasArea.root
          editor: canvasArea.editor
          logic: canvasArea.logic
          ticketCanvas: ticketCanvas
        }
      }

      Rectangle {
        z: 100
        visible: ticketCanvas.marqueeActive && ticketCanvas.marqueeMoved
        x: ticketCanvas.marqueeLeft
        y: ticketCanvas.marqueeTop
        width: Math.max(1, ticketCanvas.marqueeRight - ticketCanvas.marqueeLeft)
        height: Math.max(1, ticketCanvas.marqueeBottom - ticketCanvas.marqueeTop)
        color: "#264f9cff"
        border.width: 1
        border.color: "#4f9cff"
      }
    }
  }
}
