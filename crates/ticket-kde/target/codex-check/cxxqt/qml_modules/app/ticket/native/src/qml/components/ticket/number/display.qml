import QtQuick.Controls as Controls
import "../.."

Controls.Label {
  property var value: 1
  property var padding: 0
  property bool showPrefix: true

  text: TextFormat.displayTicketNumber(value, padding, showPrefix)
}
