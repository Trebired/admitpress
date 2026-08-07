import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: sudoFixDialog

  property var backend: root ? root.backendObject : null

  title: LocaleText.tr("Fix printer with sudo")
  dialogWidth: 460

  contentItem: ColumnLayout {

    Controls.Label {
      Layout.fillWidth: true
      text: LocaleText.tr("This can start CUPS and enable or accept jobs for the selected printer queue. ")
      + LocaleText.tr("The admin repair is run as one elevated batch, so the system password prompt should appear once when it starts.")
      wrapMode: Text.WordWrap
    }

    DialogActions {
      Controls.Button {
        text: LocaleText.tr("Run fix")
        icon.name: "tools-wizard"
        highlighted: true
        onClicked: {
          sudoFixDialog.close()
          root.settingsDialogObject.runPrinterFix(true)
        }
      }
    }
  }
}
