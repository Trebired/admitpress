import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "."

ColumnLayout {
  id: currencyPicker

  property var root
  property string value: ""
  property string placeholderLabel: ""
  property string filterText: ""
  readonly property var currencies: root && root.appState.available_currencies
  ? root.appState.available_currencies
  : []
  readonly property var filteredCurrencies: filterCurrencies(filterText)

  signal currencySelected(string currency)

  Layout.fillWidth: true
  spacing: Kirigami.Units.smallSpacing

  function filterCurrencies(term) {
    const normalized = String(term || "").trim().toLowerCase()
    if (normalized === "") return currencies
    return currencies.filter(function(currency) {
        return currency.code.toLowerCase().indexOf(normalized) >= 0
        || currency.name.toLowerCase().indexOf(normalized) >= 0
    })
  }

  function currencyLabel(code) {
    if (!code) return placeholderLabel
    for (let currency of currencies) {
      if (currency.code === code) return currency.code + " – " + currency.name
    }
    return code
  }

  function currentCurrencyIndex() {
    for (let index = 0; index < filteredCurrencies.length; ++index) {
      if (filteredCurrencies[index].code === value) return index
    }
    return 0
  }

  function applyCurrency(currency) {
    if (!currency) return
    currencySelected(currency.code)
    currencyPopup.close()
  }

  Controls.Button {
    id: currencyButton

    Layout.fillWidth: true
    text: currencyPicker.currencyLabel(currencyPicker.value)
    icon.name: AppIcons.pick("money")
    onClicked: currencyPopup.open()

    Controls.Popup {
      id: currencyPopup

      x: 0
      y: currencyButton.height
      width: currencyButton.width
      height: currencyPopupContent.implicitHeight + padding * 2
      padding: Kirigami.Units.smallSpacing
      focus: true
      closePolicy: Controls.Popup.CloseOnEscape | Controls.Popup.CloseOnPressOutside

      onOpened: {
        currencyPicker.filterText = ""
        currencySearch.text = ""
        currencyList.currentIndex = currencyPicker.currentCurrencyIndex()
        currencySearch.forceActiveFocus()
      }

      contentItem: ColumnLayout {
        id: currencyPopupContent

        spacing: Kirigami.Units.smallSpacing

        Controls.TextField {
          id: currencySearch

          Layout.fillWidth: true
          placeholderText: LocaleText.tr("Search currencies")
          selectByMouse: true
          onTextChanged: currencyPicker.filterText = text
          Keys.onPressed: event => {
            if (event.key === Qt.Key_Down && currencyList.count > 0) {
              currencyList.forceActiveFocus()
              currencyList.currentIndex = 0
              event.accepted = true
            }
          }
        }

        ListView {
          id: currencyList

          readonly property bool needsScrollbar: contentHeight > height + 1

          Layout.fillWidth: true
          Layout.preferredHeight: Math.min(contentHeight, Kirigami.Units.gridUnit * 16)
          implicitHeight: Layout.preferredHeight
          clip: true
          keyNavigationWraps: false
          model: currencyPicker.filteredCurrencies
          Controls.ScrollBar.vertical: Controls.ScrollBar {
            active: currencyPopup.opened && currencyList.needsScrollbar
            policy: currencyList.needsScrollbar
            ? Controls.ScrollBar.AlwaysOn
            : Controls.ScrollBar.AlwaysOff
          }

          Keys.onReturnPressed: currencyPicker.applyCurrency(currencyPicker.filteredCurrencies[currentIndex])
          Keys.onEnterPressed: currencyPicker.applyCurrency(currencyPicker.filteredCurrencies[currentIndex])

          delegate: Controls.ItemDelegate {
            required property var modelData

            width: currencyList.width
            text: modelData.code + " – " + modelData.name
            highlighted: ListView.isCurrentItem || modelData.code === currencyPicker.value
            onClicked: currencyPicker.applyCurrency(modelData)
          }
        }

        EmptyState {
          visible: currencyList.count === 0
          text: LocaleText.tr("No currencies found")
        }
      }
    }
  }
}
