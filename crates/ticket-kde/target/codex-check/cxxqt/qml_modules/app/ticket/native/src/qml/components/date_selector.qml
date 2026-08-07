import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Item {
  id: dateSelector

  property string selectedDate: ""
  property date displayedDate: parsedDate(selectedDate)
  readonly property date selectedDateValue: parsedDate(selectedDate)
  readonly property int selectedMonth: selectedDateValue.getMonth()
  readonly property int selectedYear: selectedDateValue.getFullYear()
  readonly property int selectedDay: selectedDateValue.getDate()
  readonly property int displayedMonth: displayedDate.getMonth()
  readonly property int displayedYear: displayedDate.getFullYear()

  signal dateSelected(string localDate)

  implicitWidth: dateButton.implicitWidth
  implicitHeight: dateButton.implicitHeight

  function parsedDate(value) {
    const match = /^(\d{4})-(\d{2})-(\d{2})$/.exec(String(value || ""))
    if (!match) return new Date()
    return new Date(Number(match[1]), Number(match[2]) - 1, Number(match[3]))
  }

  function formattedDate(value) {
    const year = value.getFullYear()
    const month = String(value.getMonth() + 1).padStart(2, "0")
    const day = String(value.getDate()).padStart(2, "0")
    return year + "-" + month + "-" + day
  }

  function sameDisplayedMonth(month, year) {
    return Number(month) === displayedMonth && Number(year) === displayedYear
  }

  function selectDate(value) {
    const localDate = formattedDate(value)
    datePopup.close()
    dateSelected(localDate)
  }

  function shiftMonth(offset) {
    displayedDate = new Date(displayedYear, displayedMonth + offset, 1)
  }

  function popupAnchor() {
    const overlay = Controls.Overlay.overlay
    if (!overlay) return Qt.point(0, dateButton.height + Kirigami.Units.smallSpacing)
    return dateSelector.mapToItem(
      overlay,
      0,
      dateSelector.height + Kirigami.Units.smallSpacing
    )
  }

  function popupX() {
    const overlay = Controls.Overlay.overlay
    const anchor = popupAnchor()
    if (!overlay) return anchor.x
    const inset = Kirigami.Units.smallSpacing
    return Math.max(inset, Math.min(anchor.x, overlay.width - datePopup.width - inset))
  }

  function popupY() {
    const overlay = Controls.Overlay.overlay
    const anchor = popupAnchor()
    if (!overlay) return anchor.y
    const inset = Kirigami.Units.smallSpacing
    return Math.max(inset, Math.min(anchor.y, overlay.height - datePopup.height - inset))
  }

  Controls.Button {
    id: dateButton

    anchors.fill: parent
    text: dateSelector.selectedDate
    icon.name: "view-calendar"
    onClicked: {
      dateSelector.displayedDate = dateSelector.parsedDate(dateSelector.selectedDate)
      datePopup.open()
    }
  }

  Controls.CalendarModel {
    id: calendarModel

    from: new Date(1900, 0, 1)
    to: new Date(2200, 11, 31)
  }

  Controls.Popup {
    id: datePopup

    parent: Controls.Overlay.overlay
    x: dateSelector.popupX()
    y: dateSelector.popupY()
    width: Math.max(dateButton.width, datePopupContent.implicitWidth + padding * 2)
    padding: Kirigami.Units.smallSpacing
    closePolicy: Controls.Popup.CloseOnEscape | Controls.Popup.CloseOnPressOutside
    modal: false
    dim: false
    focus: true

    contentItem: ColumnLayout {
      id: datePopupContent

      spacing: Kirigami.Units.smallSpacing

      RowLayout {
        Layout.fillWidth: true
        spacing: Kirigami.Units.smallSpacing

        Controls.ToolButton {
          icon.name: "go-previous"
          text: LocaleText.tr("Previous month")
          display: Controls.AbstractButton.IconOnly
          onClicked: dateSelector.shiftMonth(-1)
        }

        Controls.Label {
          Layout.fillWidth: true
          text: LocaleText.monthName(dateSelector.displayedMonth)
          + " "
          + dateSelector.displayedYear
          horizontalAlignment: Text.AlignHCenter
        }

        Controls.ToolButton {
          icon.name: "go-next"
          text: LocaleText.tr("Next month")
          display: Controls.AbstractButton.IconOnly
          onClicked: dateSelector.shiftMonth(1)
        }
      }

      RowLayout {
        Layout.fillWidth: true
        spacing: Kirigami.Units.smallSpacing

        Repeater {
          model: 7

          delegate: Controls.Label {
            required property int index

            Layout.preferredWidth: Kirigami.Units.gridUnit * 2
            text: LocaleText.weekdayName(index)
            horizontalAlignment: Text.AlignHCenter
            color: Kirigami.Theme.disabledTextColor
          }
        }
      }

      Controls.MonthGrid {
        id: monthGrid

        Layout.fillWidth: true
        source: calendarModel
        month: dateSelector.displayedMonth
        year: dateSelector.displayedYear

        onClicked: function(date) {
          dateSelector.selectDate(date)
        }

        delegate: Rectangle {
          required property var model

          readonly property bool currentMonth: dateSelector.sameDisplayedMonth(
            model.month,
            monthGrid.year
          )
          readonly property bool selectedDay: currentMonth
          && dateSelector.displayedMonth === dateSelector.selectedMonth
          && dateSelector.displayedYear === dateSelector.selectedYear
          && model.day === dateSelector.selectedDay

          width: Kirigami.Units.gridUnit * 2
          height: Kirigami.Units.gridUnit * 2
          radius: 3
          opacity: currentMonth ? 1 : 0
          color: selectedDay ? Kirigami.Theme.highlightColor : "transparent"

          Controls.Label {
            anchors.fill: parent
            text: model.day
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            color: selectedDay ? Kirigami.Theme.highlightedTextColor : Kirigami.Theme.textColor
          }
        }
      }

      Controls.Button {
        Layout.fillWidth: true
        text: LocaleText.tr("Today")
        icon.name: "go-jump-today"
        onClicked: dateSelector.selectDate(new Date())
      }
    }
  }
}
