import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "."

ColumnLayout {
  id: fontPicker

  property string value: "Noto Sans"
  property string filterText: ""
  property var installedFamilies: []
  readonly property var filteredFamilies: filterFamilies(filterText)

  signal familySelected(string family)

  Layout.fillWidth: true
  spacing: Kirigami.Units.smallSpacing

  function ensureFamiliesLoaded() {
    if (installedFamilies.length > 0) return
    installedFamilies = Qt.fontFamilies().sort(function(left, right) {
        return left.localeCompare(right)
    })
  }

  function filterFamilies(term) {
    const normalized = String(term || "").trim().toLowerCase()
    if (normalized === "") return installedFamilies
    return installedFamilies.filter(function(family) {
        return family.toLowerCase().indexOf(normalized) >= 0
    })
  }

  function currentFamilyIndex() {
    return Math.max(0, filteredFamilies.indexOf(value))
  }

  function applyFamily(family) {
    if (!family) return
    familySelected(family)
    fontPopup.close()
  }

  Controls.Button {
    id: fontButton

    Layout.fillWidth: true
    text: fontPicker.value || "Noto Sans"
    icon.name: AppIcons.pick("font")
    font.family: fontPicker.value || "Noto Sans"
    onClicked: {
      fontPicker.ensureFamiliesLoaded()
      fontPopup.open()
    }

    Controls.Popup {
      id: fontPopup

      x: 0
      y: fontButton.height
      width: fontButton.width
      height: fontPopupContent.implicitHeight + padding * 2
      padding: Kirigami.Units.smallSpacing
      focus: true
      closePolicy: Controls.Popup.CloseOnEscape | Controls.Popup.CloseOnPressOutside

      onOpened: {
        fontPicker.filterText = ""
        fontSearch.text = ""
        fontList.currentIndex = fontPicker.currentFamilyIndex()
        fontSearch.forceActiveFocus()
      }

      contentItem: ColumnLayout {
        id: fontPopupContent

        spacing: Kirigami.Units.smallSpacing

        Controls.TextField {
          id: fontSearch

          Layout.fillWidth: true
          placeholderText: LocaleText.tr("Search fonts")
          selectByMouse: true
          onTextChanged: fontPicker.filterText = text
          Keys.onPressed: event => {
            if (event.key === Qt.Key_Down && fontList.count > 0) {
              fontList.forceActiveFocus()
              fontList.currentIndex = 0
              event.accepted = true
            }
          }
        }

        ListView {
          id: fontList

          readonly property bool needsScrollbar: contentHeight > height + 1

          Layout.fillWidth: true
          Layout.preferredHeight: Math.min(contentHeight, Kirigami.Units.gridUnit * 16)
          implicitHeight: Layout.preferredHeight
          clip: true
          keyNavigationWraps: false
          model: fontPicker.filteredFamilies
          Controls.ScrollBar.vertical: Controls.ScrollBar {
            active: fontPopup.opened && fontList.needsScrollbar
            policy: fontList.needsScrollbar ? Controls.ScrollBar.AlwaysOn : Controls.ScrollBar.AlwaysOff
          }

          Keys.onReturnPressed: fontPicker.applyFamily(fontPicker.filteredFamilies[currentIndex])
          Keys.onEnterPressed: fontPicker.applyFamily(fontPicker.filteredFamilies[currentIndex])

          delegate: Controls.ItemDelegate {
            required property string modelData

            width: fontList.width
            text: modelData
            highlighted: ListView.isCurrentItem || modelData === fontPicker.value
            font.family: modelData
            onClicked: fontPicker.applyFamily(modelData)
          }
        }

        Kirigami.PlaceholderMessage {
          Layout.fillWidth: true
          visible: fontList.count === 0
          text: LocaleText.tr("No fonts found")
        }
      }
    }
  }
}
