import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: salesInfoDialog

  property int ticketCount: 0
  property int totalMinor: 0
  property string currency: ""
  property var rows: []

  title: LocaleText.tr("Sales summary")
  dialogWidth: 520
  dialogHeight: 520

  function openFor(ticketCountValue, totalMinorValue, currencyValue, rowsValue) {
    ticketCount = Number(ticketCountValue || 0)
    totalMinor = Number(totalMinorValue || 0)
    currency = String(currencyValue || "")
    rows = rowsValue || []
    open()
  }

  contentItem: ColumnLayout {
    spacing: salesInfoDialog.contentSpacing

    Controls.ScrollView {
      id: salesInfoScroll

      Layout.fillWidth: true
      Layout.fillHeight: true
      clip: true
      contentWidth: availableWidth

      ColumnLayout {
        id: salesInfoContent
        width: salesInfoScroll.availableWidth
        spacing: salesInfoDialog.contentSpacing

        RowLayout {
          Layout.fillWidth: true

          Controls.Label {
            Layout.fillWidth: true
            text: TextFormat.ticketCount(salesInfoDialog.ticketCount)
          }

          Controls.Label {
            text: MoneyFormat.format(
              salesInfoDialog.totalMinor,
              salesInfoDialog.currency || root.appState.main_currency || ""
            )
          }
        }

        SectionBorder {}

        DaySalesTable {
          rows: salesInfoDialog.rows
          fallbackCurrency: root.appState.main_currency || ""
          visible: salesInfoDialog.rows.length > 0
        }

        Kirigami.PlaceholderMessage {
          Layout.fillWidth: true
          visible: salesInfoDialog.rows.length === 0
          text: LocaleText.tr("No sales")
        }
      }
    }
  }
}
