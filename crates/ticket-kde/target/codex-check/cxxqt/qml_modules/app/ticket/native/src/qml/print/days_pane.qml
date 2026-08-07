import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

Item {
  id: daysPane

  property var root
  property var printPage
  readonly property int contentGap: Kirigami.Units.largeSpacing
  readonly property int listHeight: Math.max(
    Kirigami.Units.gridUnit * 8,
    height - contentGap * 2 - daysHeader.implicitHeight - Kirigami.Units.smallSpacing
  )

  Layout.fillHeight: true
  Layout.fillWidth: true
  Layout.preferredWidth: 760
  Layout.minimumWidth: 390

  Controls.ScrollView {
    id: daysScroll

    anchors.fill: parent
    clip: true
    contentWidth: availableWidth
    contentHeight: daysContentFrame.implicitHeight

    Controls.ScrollBar.horizontal.policy: Controls.ScrollBar.AlwaysOff

    Item {
      id: daysContentFrame

      width: daysScroll.availableWidth
      implicitHeight: daysContent.implicitHeight + daysPane.contentGap * 2

      ColumnLayout {
        id: daysContent

        x: daysPane.contentGap
        y: daysPane.contentGap
        width: Math.max(0, daysContentFrame.width - daysPane.contentGap * 2)
        spacing: Kirigami.Units.smallSpacing

        RowLayout {
          id: daysHeader

          Layout.fillWidth: true

          Controls.Label {
            text: LocaleText.tr("Days")
            font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
          }

          Controls.Label {
            Layout.fillWidth: true
            text: printPage.selectedDaySummaryText()
            horizontalAlignment: Text.AlignRight
            elide: Text.ElideRight
            color: printPage.selectedDayCount() > 0
            ? Kirigami.Theme.textColor
            : Kirigami.Theme.disabledTextColor
          }

          Controls.CheckBox {
            text: LocaleText.tr("All")
            checked: printPage.allDaysSelected()
            enabled: printPage.daySummaries().length > 0
            padding: 0
            onClicked: {
              if (checked) {
                printPage.selectAllDays()
              } else {
                printPage.clearSelectedDays()
              }
            }
          }
        }

        PrintScrollList {
          maxHeight: daysPane.listHeight
          hideWhenEmpty: true
          model: printPage.summary && printPage.summary.day_summaries
          ? printPage.summary.day_summaries
          : []

          delegate: DaySummaryRow {
            required property var modelData
            day: modelData
            appRoot: root
            printPage: daysPane.printPage
            onDetailRequested: {
              root.dayDetailDialogObject.openFor(printPage.ticketId, modelData)
            }
          }
        }
      }
    }
  }
}
