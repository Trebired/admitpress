import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

ListView {
  id: scrollList

  property int maxHeight: Kirigami.Units.gridUnit * 8
  property bool hideWhenEmpty: false

  property int rowHeight: Kirigami.Units.gridUnit * 2.2
  readonly property int fullHeight: count * rowHeight
  readonly property bool needsScrollbar: fullHeight > maxHeight
  readonly property int scrollbarWidth: Math.ceil(Math.max(
      scrollBar.width,
      scrollBar.implicitWidth,
      Kirigami.Units.smallSpacing
  ))
  readonly property int scrollbarReserve: needsScrollbar
  ? scrollbarWidth + Kirigami.Units.smallSpacing
  : 0

  Layout.fillWidth: true
  Layout.preferredHeight: Math.min(Math.max(fullHeight, 1), maxHeight)
  Layout.maximumHeight: maxHeight
  clip: true
  boundsBehavior: Flickable.StopAtBounds
  interactive: needsScrollbar
  visible: !hideWhenEmpty || count > 0

  Controls.ScrollBar.vertical: Controls.ScrollBar {
    id: scrollBar

    policy: scrollList.needsScrollbar ? Controls.ScrollBar.AsNeeded : Controls.ScrollBar.AlwaysOff
  }
}
