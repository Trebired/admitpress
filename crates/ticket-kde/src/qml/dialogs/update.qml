import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: updateDialog

  property string latestVersion: ""
  property string currentVersion: ""
  property string releaseUrl: ""

  title: LocaleText.tr("A new version is available")
  dialogWidth: 520

  function openFor(release) {
    if (!release) return
    updateDialog.latestVersion = release.latest_version || ""
    updateDialog.currentVersion = release.current_version || ""
    updateDialog.releaseUrl = release.release_url || ""
    updateDialog.open()
  }

  contentItem: ColumnLayout {
    spacing: updateDialog.contentSpacing

    Controls.Label {
      Layout.fillWidth: true
      wrapMode: Text.WordWrap
      text: LocaleText.tr("Version ") + updateDialog.latestVersion
      + LocaleText.tr(" has been released. This copy is version ")
      + updateDialog.currentVersion + "."
    }

    SectionBorder {}

    Controls.Label {
      Layout.fillWidth: true
      wrapMode: Text.WordWrap
      color: Kirigami.Theme.disabledTextColor
      text: LocaleText.tr("You can install it at:")
    }

    Controls.TextField {
      Layout.fillWidth: true
      readOnly: true
      text: updateDialog.releaseUrl
      onFocusChanged: if (focus) selectAll()
    }

    Controls.Label {
      Layout.fillWidth: true
      wrapMode: Text.WordWrap
      color: Kirigami.Theme.disabledTextColor
      text: root.appDisplayName + LocaleText.tr(" does not install updates itself.")
    }

    DialogActions {
      Controls.Button {
        text: LocaleText.tr("Open the release page")
        icon.name: AppIcons.pick("link")
        highlighted: true
        enabled: updateDialog.releaseUrl !== ""
        onClicked: Qt.openUrlExternally(updateDialog.releaseUrl)
      }
    }
  }
}
