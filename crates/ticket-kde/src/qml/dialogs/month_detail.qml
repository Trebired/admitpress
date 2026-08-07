import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: monthDetailDialog

  property string monthKey: ""
  property string monthTitle: ""
  property var summary: null
  property var rows: []

  title: monthTitle || LocaleText.tr("Month details")
  dialogWidth: 520
  dialogHeight: 520

  function openFor(monthKeyValue, monthTitleValue, summaryValue, rowsValue) {
    monthKey = String(monthKeyValue || "")
    monthTitle = String(monthTitleValue || "")
    summary = summaryValue || null
    rows = rowsValue || []
    open()
  }

  contentItem: ColumnLayout {
    spacing: monthDetailDialog.contentSpacing

    Controls.ScrollView {
      id: monthDetailScroll

      Layout.fillWidth: true
      Layout.fillHeight: true
      clip: true
      contentWidth: availableWidth

      ColumnLayout {
        id: monthDetailContent
        width: monthDetailScroll.availableWidth
        spacing: monthDetailDialog.contentSpacing

        RowLayout {
          Layout.fillWidth: true

          Controls.Label {
            text: monthDetailDialog.summary
            ? TextFormat.dayCount(monthDetailDialog.summary.dayCount)
            : ""
          }

          Controls.Label {
            Layout.fillWidth: true
            text: monthDetailDialog.summary
            ? TextFormat.ticketCount(monthDetailDialog.summary.ticketTotal)
            : ""
          }

          Controls.Label {
            text: monthDetailDialog.summary
            ? MoneyFormat.format(
              monthDetailDialog.summary.moneyTotal,
              monthDetailDialog.summary.currency
            )
            : ""
          }
        }

        SectionBorder {}

        DaySalesTable {
          rows: monthDetailDialog.rows
          visible: rows.length > 0
        }

        Kirigami.PlaceholderMessage {
          Layout.fillWidth: true
          visible: monthDetailDialog.rows.length === 0
          text: LocaleText.tr("No sales")
        }
      }
    }
  }
}
