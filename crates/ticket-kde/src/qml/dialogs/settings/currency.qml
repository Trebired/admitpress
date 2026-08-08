import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../../components"

ColumnLayout {
  id: settingsCurrency

  property var settingsDialog
  readonly property var appRoot: settingsDialog ? settingsDialog.root : null
  readonly property var backend: settingsDialog ? settingsDialog.backend : null

  Layout.fillWidth: true
  spacing: Kirigami.Units.smallSpacing

  Controls.Label {
    text: LocaleText.tr("Main currency")
    font.pointSize: Kirigami.Theme.defaultFont.pointSize + 1
  }

  CurrencyPicker {
    root: settingsCurrency.appRoot
    value: appRoot ? appRoot.appState.main_currency : ""
    placeholderLabel: LocaleText.tr("Choose a currency")
    onCurrencySelected: function(currency) {
      const data = appRoot.acceptResponse(
        backend.setMainCurrency(currency),
        LocaleText.tr("Main currency updated")
      )
      if (!data) return
      appRoot.appState = Object.assign({}, appRoot.appState, {
          main_currency: data.main_currency || currency
      })
      const page = appRoot.pageStack.currentItem
      if (page && page.load) page.load()
      if (page && page.refreshSaleCurrencyDisplay) page.refreshSaleCurrencyDisplay()
    }
  }

  Controls.Label {
    Layout.fillWidth: true
    wrapMode: Text.WordWrap
    color: Kirigami.Theme.disabledTextColor
    text: LocaleText.tr("Ticket prices are stored in this currency. "
      + "You can sell and print in another currency from the print page.")
  }
}
